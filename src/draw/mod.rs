use wgpu::{
    BackendBit, CommandBuffer, Device, DeviceDescriptor, Features, Instance, Limits,
    PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface, SwapChain,
    SwapChainDescriptor, TextureUsage,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{camera::Camera, console::Console, time::Frame};

use self::{overlay::OverlaySubsystem, voxels::VoxelSubsystem};

mod overlay;
mod texture;
mod voxels;

pub struct DrawSystem {
    surface: Surface,
    device: Device,
    queue: Queue,
    swapchain_desc: SwapChainDescriptor,
    swapchain: SwapChain,
    voxel_ss: VoxelSubsystem,
    overlay_ss: OverlaySubsystem,
}

impl DrawSystem {
    pub async fn new(window: &Window) -> Self {
        let instance = Instance::new(BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::empty(),
                    limits: Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let PhysicalSize { width, height } = window.inner_size();
        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();
        let swapchain_desc = SwapChainDescriptor {
            usage: TextureUsage::RENDER_ATTACHMENT,
            format: swapchain_format,
            width,
            height,
            present_mode: PresentMode::Fifo,
        };
        let swapchain = device.create_swap_chain(&surface, &swapchain_desc);
        let voxel_ss = VoxelSubsystem::new(&device, &swapchain_desc);
        let console_ss = OverlaySubsystem::new(&device);
        Self {
            surface,
            device,
            queue,
            swapchain_desc,
            swapchain,
            voxel_ss,
            overlay_ss: console_ss,
        }
    }

    pub fn resize_surface(&mut self, new_size: &PhysicalSize<u32>) {
        self.swapchain_desc.width = new_size.width;
        self.swapchain_desc.height = new_size.height;
        self.swapchain = self
            .device
            .create_swap_chain(&self.surface, &self.swapchain_desc);
        self.voxel_ss
            .update_swapchain(&self.device, &self.swapchain_desc);
    }

    pub fn redraw(&mut self, frame: &Frame, camera: &Camera, console: &Console) {
        let swapchain_texture = match self.swapchain.get_current_frame() {
            Ok(frame) => frame.output,
            Err(wgpu::SwapChainError::OutOfMemory) => panic!("Out of memory!"),
            Err(_) => return, // Handled on the next frame
        };
        let color_target = &swapchain_texture.view;
        let color_target_bounds = (self.swapchain_desc.width, self.swapchain_desc.height);
        let commands: Vec<CommandBuffer> = vec![
            self.voxel_ss.draw(
                &self.device,
                &self.queue,
                color_target,
                color_target_bounds,
                camera,
            ),
            self.overlay_ss.draw(
                frame,
                &self.device,
                color_target,
                color_target_bounds,
                console,
                self.voxel_ss.triangle_count,
            ),
        ];
        self.queue.submit(commands);
        if console.is_showing() {
            self.overlay_ss.recycle();
        }
    }
}

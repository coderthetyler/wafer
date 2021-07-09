use wgpu::{
    BackendBit, CommandBuffer, Device, DeviceDescriptor, Features, Instance, Limits,
    PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface, SwapChain,
    SwapChainDescriptor, TextureUsage, TextureView,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    app::AppConfig,
    camera::Camera,
    entity::{Ecs, Entity},
    frame::Frame,
    session::ConsoleSession,
};

use self::{overlay::OverlayPainter, scene::ScenePainter};

mod overlay;
mod scene;
mod texture;

pub struct PaintSurface {
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub swapchain_desc: SwapChainDescriptor,
    pub swapchain: SwapChain,
}

pub struct PaintContext<'surface> {
    pub surface: &'surface PaintSurface,
    pub color_target: &'surface TextureView,
    pub width: f32,
    pub height: f32,
}

pub struct PaintSystem {
    surface: PaintSurface,
    fallback_camera: Camera,
    pub active_camera: Entity,
    pub scene: ScenePainter,
    pub overlay: OverlayPainter,
}

impl PaintSystem {
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
        let world_painter = ScenePainter::new(&device, &swapchain_desc);
        let overlay_painter = OverlayPainter::new(&device);
        Self {
            active_camera: Entity::none(),
            fallback_camera: Camera::new(),
            surface: PaintSurface {
                surface,
                device,
                queue,
                swapchain_desc,
                swapchain,
            },
            scene: world_painter,
            overlay: overlay_painter,
        }
    }

    pub fn resize_surface(&mut self, new_size: &PhysicalSize<u32>) {
        let surface = &mut self.surface;
        surface.swapchain_desc.width = new_size.width;
        surface.swapchain_desc.height = new_size.height;
        surface.swapchain = surface
            .device
            .create_swap_chain(&surface.surface, &surface.swapchain_desc);
        self.scene
            .update_swapchain(&surface.device, &surface.swapchain_desc);
    }

    pub fn redraw(
        &mut self,
        state: &AppConfig,
        frame: &Frame,
        session: &ConsoleSession,
        ecs: &Ecs,
    ) {
        let camera = ecs
            .comps
            .camera
            .get(self.active_camera)
            .unwrap_or(&self.fallback_camera);
        let surface = &mut self.surface;
        let swapchain_texture = match surface.swapchain.get_current_frame() {
            Ok(frame) => frame.output,
            Err(wgpu::SwapChainError::OutOfMemory) => panic!("Out of memory!"),
            Err(_) => return, // Handled on the next frame
        };
        let color_target = &swapchain_texture.view;
        let color_target_bounds = (surface.swapchain_desc.width, surface.swapchain_desc.height);
        let mut context = PaintContext {
            surface,
            color_target,
            width: surface.swapchain_desc.width as f32,
            height: surface.swapchain_desc.height as f32,
        };
        // ctx: &mut PaintContext, camera: &Camera
        let commands: Vec<CommandBuffer> = vec![
            self.scene
                .paint(state, &mut context, self.active_camera, ecs),
            self.overlay.draw(
                state,
                frame,
                &self.surface.device,
                color_target,
                color_target_bounds,
                session,
                10, // TODO re-add triangle counting
            ),
        ];
        self.surface.queue.submit(commands);
        if session.is_showing() {
            self.overlay.recycle();
        }
    }
}

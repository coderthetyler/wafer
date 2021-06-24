use crate::{
    camera::Camera,
    generation::{
        GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexIter, GenerationalIndexVec,
    },
    geometry::{Position, Vec3f, Volume},
};

pub type Entity = GenerationalIndex;
pub type EntityIter<'map> = GenerationalIndexIter<'map>;
pub type ComponentVec<T> = GenerationalIndexVec<T>;

pub struct EntitySystem {
    entity_allocator: GenerationalIndexAllocator,

    // Components
    pub positions: ComponentVec<Position>,
    pub velocities: ComponentVec<Vec3f>,
    pub colliders: ComponentVec<Volume>,
    pub cameras: ComponentVec<Camera>,

    // State
    pub selected_camera: Entity,
}

impl EntitySystem {
    pub fn new() -> Self {
        Self {
            entity_allocator: GenerationalIndexAllocator::new(),

            positions: ComponentVec::new(),
            velocities: ComponentVec::new(),
            colliders: ComponentVec::new(),
            cameras: ComponentVec::new(),

            selected_camera: Entity::none(),
        }
    }

    pub fn iter(&self) -> EntityIter {
        self.entity_allocator.iter()
    }

    pub fn get_selected_camera(&self) -> Option<&Camera> {
        self.cameras.get(self.selected_camera)
    }

    pub fn get_selected_camera_mut(&mut self) -> Option<&mut Camera> {
        self.cameras.get_mut(self.selected_camera)
    }

    pub fn kill(&mut self, entity: Entity) -> bool {
        self.entity_allocator.deallocate(entity)
    }

    pub fn create(&mut self) -> Entity {
        self.entity_allocator.allocate()
    }
}

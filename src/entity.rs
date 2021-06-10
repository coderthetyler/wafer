use crate::{
    camera::Camera,
    draw::DrawComponent,
    generation::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexVec},
    geometry::Position,
    input::InputComponent,
};

pub type Entity = GenerationalIndex;
pub type EntityMap<T> = GenerationalIndexVec<T>;

pub struct EntitySystem {
    entity_allocator: GenerationalIndexAllocator,

    // Components
    pub positions: EntityMap<Position>,
    pub draws: EntityMap<DrawComponent>,
    pub inputs: EntityMap<InputComponent>,
    pub cameras: EntityMap<Box<dyn Camera>>,

    // State
    pub selected_camera: Entity,
}

impl EntitySystem {
    pub fn new() -> Self {
        Self {
            entity_allocator: GenerationalIndexAllocator::new(),

            positions: EntityMap::new(),
            draws: EntityMap::new(),
            inputs: EntityMap::new(),
            cameras: EntityMap::new(),

            selected_camera: Entity::none(),
        }
    }

    pub fn get_selected_camera(&self) -> Option<&Box<dyn Camera>> {
        self.cameras.get(self.selected_camera)
    }

    pub fn kill(&mut self, entity: Entity) -> bool {
        self.entity_allocator.deallocate(entity)
    }

    pub fn create(&mut self) -> Entity {
        self.entity_allocator.allocate()
    }
}

// TODO gonna want a way to quickly query all entities that have some subset of components

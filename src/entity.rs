use crate::{
    camera::Camera,
    generation::{GenerationalIndex, GenerationalIndexPool, GenerationalIndexVec},
    geometry::{Position, Rotation, Volume},
    movement::{Spin, Velocity},
};

pub type Entity = GenerationalIndex;
pub type ComponentVec<T> = GenerationalIndexVec<T>;

pub struct EntityPool {
    pub pool: GenerationalIndexPool,

    // Components
    pub position: ComponentVec<Position>,
    pub velocity: ComponentVec<Velocity>,
    pub rotation: ComponentVec<Rotation>,
    pub spin: ComponentVec<Spin>,
    pub colliders: ComponentVec<Volume>,
    pub camera: ComponentVec<Camera>,
}

impl EntityPool {
    pub fn new() -> Self {
        Self {
            pool: GenerationalIndexPool::new(),

            position: ComponentVec::new(),
            velocity: ComponentVec::new(),
            rotation: ComponentVec::new(),
            spin: ComponentVec::new(),
            colliders: ComponentVec::new(),
            camera: ComponentVec::new(),
        }
    }
}

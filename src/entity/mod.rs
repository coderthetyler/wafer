mod delta;

use crate::{
    camera::Camera,
    puppet::Puppet,
    types::{GenerationalIndex, GenerationalIndexPool, GenerationalIndexVec},
    types::{Position, Rotation, Spin, Velocity, Volume},
};

pub use delta::EntityDelta;

pub type Entity = GenerationalIndex;
pub type EntityPool = GenerationalIndexPool;
pub type ComponentVec<T> = GenerationalIndexVec<T>;

pub struct Ecs {
    pub pool: EntityPool,
    pub comps: EntityComponents,
}

impl Ecs {
    pub fn new() -> Self {
        Self {
            pool: EntityPool::new(),
            comps: EntityComponents::new(),
        }
    }
}

pub struct EntityComponents {
    pub position: ComponentVec<Position>,
    pub velocity: ComponentVec<Velocity>,
    pub rotation: ComponentVec<Rotation>,
    pub spin: ComponentVec<Spin>,
    pub volumes: ComponentVec<Volume>,
    pub camera: ComponentVec<Camera>,
    pub puppet: ComponentVec<Puppet>,
}

impl EntityComponents {
    fn new() -> Self {
        Self {
            position: ComponentVec::new(),
            velocity: ComponentVec::new(),
            rotation: ComponentVec::new(),
            spin: ComponentVec::new(),
            volumes: ComponentVec::new(),
            camera: ComponentVec::new(),
            puppet: ComponentVec::new(),
        }
    }
}

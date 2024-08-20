use crate::cell::ArefCell;
use crate::ptr::StrongPtr;

use super::game_object::GameObject;
use super::game_object::GameObjectStrongPtr;
use super::game_object::GameObjectWeakPtr;
use super::id::GameObjectHandle;
use super::id::GameObjectId;
use super::id::GameObjectKind;

const DEFAULT_MOVABLES: usize = 1024;
const DEFAULT_STATIC_CHUNKS: usize = 8;
const DEFAULT_STATICS_PER_CHUNK: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneError {
    GameObjectIsDestroyed,
    ScaleMustBePositive,
    CircularHierarchy,
    IndexOutOfBounds,
    OutOfMemory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SceneCreateInfo {
    pub movables: usize,
    pub static_chunks: usize,
    pub statics_per_chunk: usize,
}

pub struct Scene {
    pub movables: Vec<GameObjectStrongPtr>,
    pub statics: Vec<Vec<GameObjectStrongPtr>>,
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SceneError::GameObjectIsDestroyed => write!(f, "game object was destroyed"),
            SceneError::ScaleMustBePositive => write!(f, "scale must be larger than 0"),
            SceneError::CircularHierarchy => {
                write!(f, "operation would have caused a circular hierarchy")
            }
            SceneError::IndexOutOfBounds => write!(f, "index was out of bounds"),
            SceneError::OutOfMemory => write!(f, "out of memory"),
        }
    }
}

pub type SceneResult<T> = Result<T, SceneError>;

impl std::error::Error for SceneError {}

impl Default for SceneCreateInfo {
    fn default() -> Self {
        Self {
            movables: DEFAULT_MOVABLES,
            static_chunks: DEFAULT_STATIC_CHUNKS,
            statics_per_chunk: DEFAULT_STATICS_PER_CHUNK,
        }
    }
}

impl Scene {
    pub fn new(info: SceneCreateInfo) -> Self {
        let mut movables = Vec::with_capacity(info.movables);
        for i in 0..info.movables {
            let handle = GameObjectHandle {
                id: GameObjectId {
                    kind: GameObjectKind::Movable,
                    index: i,
                },
                generation: 0,
            };

            let game_object = GameObject::new(handle, false);
            let ptr = StrongPtr::new(ArefCell::new(game_object));
            movables.push(ptr);
        }

        let mut statics = Vec::with_capacity(info.static_chunks);
        for i in 0..info.static_chunks {
            let mut chunk = Vec::with_capacity(info.statics_per_chunk);
            for j in 0..info.statics_per_chunk {
                let handle = GameObjectHandle {
                    id: GameObjectId {
                        kind: GameObjectKind::Static { chunk: i },
                        index: j,
                    },
                    generation: 0,
                };

                let game_object = GameObject::new(handle, false);
                let ptr = StrongPtr::new(ArefCell::new(game_object));
                chunk.push(ptr);
            }

            statics.push(chunk);
        }

        Self { movables, statics }
    }

    pub fn resolve(&self, handle: GameObjectHandle) -> SceneResult<GameObjectWeakPtr> {
        let ptr = match handle.id.kind {
            GameObjectKind::Movable => &self.movables[handle.id.index],
            GameObjectKind::Static { chunk } => &self.statics[chunk][handle.id.index],
        };

        let aref = ptr.borrow();

        let is_alive = aref.is_alive();
        let generation_matches = aref.handle().generation == handle.generation;

        if is_alive && generation_matches {
            Ok(ptr.to_weak())
        } else {
            Err(SceneError::GameObjectIsDestroyed)
        }
    }

    pub fn count_available_game_objects(&self, kind: GameObjectKind) -> usize {
        let chunk = match kind {
            GameObjectKind::Movable => &self.movables,
            GameObjectKind::Static { chunk } => &self.statics[chunk],
        };

        chunk.iter().filter(|x| x.borrow().is_alive()).count()
    }
}

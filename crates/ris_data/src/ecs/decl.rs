use super::components::mesh::MeshComponent;
use super::game_object::GameObject;
use super::handle::ComponentHandle;
use super::handle::DynComponentHandle;
use super::handle::DynHandle;
use super::handle::GenericHandle;
use super::handle::Handle;
use super::id::Component;
use super::id::EcsObject;
use super::scene::Scene;
use super::script::DynScriptComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcsTypeId {
    GameObject,
    MeshComponent,
    ScriptComponent,
}

declare::object!(GameObjectHandle, GameObject, EcsTypeId::GameObject,);
declare::component!(MeshComponentHandle, MeshComponent, EcsTypeId::MeshComponent,);
declare::component!(
    DynScriptComponentHandle,
    DynScriptComponent,
    EcsTypeId::ScriptComponent,
);

mod declare {
    macro_rules! object {
        (
            $handle_name:ident,
            $handle_type:ident,
            $ecs_type_id:expr $(,)?
        ) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $handle_name(pub GenericHandle<$handle_type>);

            impl std::ops::Deref for $handle_name {
                type Target = GenericHandle<$handle_type>;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for $handle_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl Handle for $handle_name {
                fn ecs_type_id() -> EcsTypeId {
                    $ecs_type_id
                }

                fn to_dyn(self) -> DynHandle {
                    self.0.into()
                }
            }

            impl From<GenericHandle<$handle_type>> for $handle_name {
                fn from(value: GenericHandle<$handle_type>) -> Self {
                    Self(value)
                }
            }

            impl From<$handle_name> for GenericHandle<$handle_type> {
                fn from(value: $handle_name) -> Self {
                    value.0
                }
            }

            impl EcsObject for $handle_type {
                fn ecs_type_id() -> EcsTypeId {
                    $ecs_type_id
                }
            }

            impl $handle_name {
                pub fn null() -> Self {
                    let handle = GenericHandle::null();
                    Self(handle)
                }

                pub fn is_alive(self, scene: &Scene) -> bool {
                    self.0.is_alive(scene)
                }
            }
        };
    }

    macro_rules! component {
        (
            $handle_name:ident,
            $handle_type:ident,
            $ecs_type_id:expr $(,)?
        ) => {
            declare::object!($handle_name, $handle_type, $ecs_type_id);

            impl ComponentHandle for $handle_name {
                fn to_dyn_component(self) -> DynComponentHandle {
                    self.0.into()
                }
            }
        };
    }

    pub(crate) use component;
    pub(crate) use object;
}

use std::any::TypeId;

pub trait Event: core::fmt::Debug {
    fn event_id(&self) -> TypeId;
}

pub mod math;
use std::marker::PhantomData;

pub mod body;
pub mod common;
pub mod debug_draw;
pub mod shapes;
pub mod world;

/// Used to mark our handles as !Send and !Sync for some attempt at thread safety.
type PhantomNoSend = PhantomData<*mut ()>;

pub use body::{Body, BodyDef};
pub use math::Vec2;
pub use shapes::{Shape, ShapeDef};
pub use world::{World, WorldDef};

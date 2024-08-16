mod box2d3;

use crate::renderer::Renderer;

use super::Vec2;

#[derive(Debug)]
pub struct UnsupportedError(&'static str);

static ENGINE_CTORS: &[(&str, fn() -> Box<dyn Engine>)] =
    &[("box2d3", || Box::new(box2d3::Engine::new()))];

pub fn init_engine(name: &str) -> Option<Box<dyn Engine>> {
    for (n, f) in ENGINE_CTORS {
        if *n == name {
            return Some(f());
        }
    }
    None
}

pub trait Engine {
    fn add_body(&mut self, def: BodyDef) -> Result<(), UnsupportedError>;

    fn step(&self, dt: f32, steps: u32);

    fn draw(&self, render: &mut Renderer);
}

pub struct Polygon {
    vertices: Vec<Vec2>,
    radius: f32,
}

pub struct Circle {
    radius: f32,
    offset: Vec2,
}

pub enum ShapeDef {
    Polygon(Polygon),
    Circle,
}

enum BodyKind {
    Static,
    Dynamic,
    Kinematic,
}

pub struct BodyDef {
    kind: BodyKind,
    shapes: Vec<ShapeDef>,
    position: Vec2,
    linear_velocity: Vec2,
    angular_velocity: f32,
}

impl BodyDef {
    pub fn new(position: Vec2, shapes: Vec<ShapeDef>) -> Self {
        Self {
            kind: BodyKind::Dynamic,
            shapes,
            position,
            linear_velocity: Vec2::ZERO,
            angular_velocity: 0.0,
        }
    }

    pub fn set_static(mut self) -> Self {
        self.kind = BodyKind::Static;
        self
    }
}

impl Polygon {
    pub fn new_box(w: f32, h: f32) -> Self {
        Self {
            vertices: vec![
                Vec2::new(w / 2.0, h / 2.0),
                Vec2::new(-w / 2.0, h / 2.0),
                Vec2::new(w / 2.0, -h / 2.0),
                Vec2::new(-w / 2.0, -h / 2.0),
            ],
            radius: 0.0,
        }
    }

    pub fn offset(mut self, offset: Vec2) -> Self {
        for vert in &mut self.vertices {
            *vert += offset;
        }
        self
    }
}

impl From<Polygon> for ShapeDef {
    fn from(inner_shape: Polygon) -> Self {
        ShapeDef::Polygon(inner_shape)
    }
}

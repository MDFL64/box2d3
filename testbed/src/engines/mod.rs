mod box2d3;
mod wrapped2d;

use crate::renderer::Renderer;

use super::Vec2;

#[derive(Debug)]
pub struct UnsupportedError(&'static str);

pub static ENGINES: &[(&str, fn() -> Box<dyn Engine>)] = &[
    ("Box2D 3.0.0 (box2d3)", || Box::new(box2d3::Engine::new())),
    ("Box2D 2.3.1 (wrapped2d)", || {
        Box::new(wrapped2d::Engine::new())
    }),
];

pub trait Engine {
    fn add_body(&mut self, def: BodyDef) -> Result<(), UnsupportedError>;

    fn step(&mut self, dt: f32, steps: u32);

    fn draw(&mut self, render: &mut Renderer);
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
    Circle(Circle),
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
    friction: f32,
    restitution: f32,
}

impl BodyDef {
    pub fn new(position: Vec2, shapes: Vec<ShapeDef>) -> Self {
        Self {
            kind: BodyKind::Dynamic,
            shapes,
            position,
            linear_velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            friction: 0.5,
            restitution: 0.2,
        }
    }

    pub fn set_static(mut self) -> Self {
        self.kind = BodyKind::Static;
        self
    }

    pub fn friction(mut self, x: f32) -> Self {
        self.friction = x;
        self
    }

    pub fn restitution(mut self, x: f32) -> Self {
        self.restitution = x;
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

    pub fn rotate(mut self, degrees: f32) -> Self {
        let rad = -degrees.to_radians();
        let s = rad.sin();
        let c = rad.cos();
        for vert in &mut self.vertices {
            let x = vert.x * c - vert.y * s;
            let y = vert.x * s + vert.y * c;
            *vert = Vec2 { x, y };
        }
        self
    }
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle {
            radius,
            offset: Vec2::ZERO,
        }
    }
}

impl From<Polygon> for ShapeDef {
    fn from(inner_shape: Polygon) -> Self {
        ShapeDef::Polygon(inner_shape)
    }
}

impl From<Circle> for ShapeDef {
    fn from(inner_shape: Circle) -> Self {
        ShapeDef::Circle(inner_shape)
    }
}

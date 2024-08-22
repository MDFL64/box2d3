use box2d3::common::HexColor;

use crate::renderer::Renderer;

use super::UnsupportedError;

use wrapped2d::b2::{self, DrawFlags};
use wrapped2d::user_data::NoUserData;

pub struct Engine {
    world: b2::World<NoUserData>,
}

impl Engine {
    pub fn new() -> Self {
        let world = b2::World::new(&b2::Vec2 { x: 0.0, y: -10.0 });
        Self { world }
    }
}

fn convert_vec2(v: box2d3::Vec2) -> b2::Vec2 {
    b2::Vec2 { x: v.x, y: v.y }
}

fn convert_vec2_back(v: b2::Vec2) -> box2d3::Vec2 {
    box2d3::Vec2 { x: v.x, y: v.y }
}

fn convert_color(c: &b2::Color) -> HexColor {
    HexColor::new_from_floats(c.r, c.g, c.b)
}

impl super::Engine for Engine {
    fn add_body(&mut self, def: super::BodyDef) -> Result<(), UnsupportedError> {
        let mut b2d_def = b2::BodyDef::new();
        b2d_def.position = convert_vec2(def.position);
        b2d_def.linear_velocity = convert_vec2(def.linear_velocity);
        b2d_def.angular_velocity = def.angular_velocity;
        b2d_def.body_type = match def.kind {
            super::BodyKind::Dynamic => b2::BodyType::Dynamic,
            super::BodyKind::Static => b2::BodyType::Static,
            super::BodyKind::Kinematic => b2::BodyType::Kinematic,
        };

        let new_body = self.world.create_body(&b2d_def);
        let mut body = self.world.body_mut(new_body);
        for shape in def.shapes {
            match shape {
                super::ShapeDef::Polygon(polygon) => {
                    let verts: Vec<_> =
                        polygon.vertices.iter().copied().map(convert_vec2).collect();
                    let polygon = b2::PolygonShape::new_with(&verts);

                    let fh = body.create_fast_fixture(&polygon, 1.0);
                    let mut fixture = body.fixture_mut(fh);
                    fixture.set_friction(def.friction);
                    fixture.set_restitution(def.restitution);
                }
                super::ShapeDef::Circle => {
                    return Err(UnsupportedError("circle shapes"));
                }
            }
        }

        Ok(())
    }

    fn step(&mut self, dt: f32, steps: u32) {
        self.world.step(dt, steps as i32, steps as i32 / 2);
    }

    fn draw(&mut self, render: &mut Renderer) {
        self.world
            .draw_debug_data(&mut DebugDrawer { render }, DrawFlags::DRAW_SHAPE);
    }
}

struct DebugDrawer<'a> {
    render: &'a mut Renderer,
}

impl<'a> b2::Draw for DebugDrawer<'a> {
    fn draw_polygon(&mut self, vertices: &[b2::Vec2], color: &b2::Color) {
        println!("poly");
    }

    fn draw_solid_polygon(&mut self, vertices: &[b2::Vec2], color: &b2::Color) {
        let points: Vec<_> = vertices.iter().copied().map(convert_vec2_back).collect();

        self.render.draw_polygon(&points, convert_color(color));
    }

    fn draw_circle(&mut self, center: &b2::Vec2, radius: f32, color: &b2::Color) {
        println!("circle");
    }

    fn draw_solid_circle(
        &mut self,
        center: &b2::Vec2,
        radius: f32,
        axis: &b2::Vec2,
        color: &b2::Color,
    ) {
        println!("solid circle");
    }

    fn draw_segment(&mut self, p1: &b2::Vec2, p2: &b2::Vec2, color: &b2::Color) {
        println!("segment");
    }

    fn draw_transform(&mut self, xf: &b2::Transform) {
        println!("xform");
    }
}

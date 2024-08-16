/*use macroquad::{
    color::Color,
    models::draw_mesh,
    shapes::{draw_poly, draw_triangle},
};*/

use box2d3::common::HexColor;

use crate::renderer::Renderer;
use crate::Color;

use super::UnsupportedError;

pub struct Engine {
    world: box2d3::World,
}

impl Engine {
    pub fn new() -> Self {
        let world_def = box2d3::WorldDef::default();
        let world = box2d3::World::new(&world_def);

        Self { world }
    }
}

impl super::Engine for Engine {
    fn add_body(&mut self, def: super::BodyDef) -> Result<(), UnsupportedError> {
        let mut b2d_def = box2d3::BodyDef::default();
        b2d_def.position = def.position;
        b2d_def.linear_velocity = def.linear_velocity;
        b2d_def.angular_velocity = def.angular_velocity;
        b2d_def.kind = match def.kind {
            super::BodyKind::Dynamic => box2d3::body::BodyKind::Dynamic,
            super::BodyKind::Static => box2d3::body::BodyKind::Static,
            super::BodyKind::Kinematic => box2d3::body::BodyKind::Kinematic,
        };

        let new_body = self.world.create_body(&b2d_def);
        for shape in def.shapes {
            match shape {
                super::ShapeDef::Polygon(polygon) => {
                    let hull = box2d3::shapes::Hull::compute(&polygon.vertices);

                    let shape_def = box2d3::ShapeDef::default();
                    let polygon = box2d3::shapes::Polygon::new(&hull, polygon.radius);

                    new_body.create_shape_polygon(&shape_def, &polygon);
                }
                super::ShapeDef::Circle => {
                    return Err(UnsupportedError("circle shapes"));
                }
            }
        }

        Ok(())
    }

    fn step(&self, dt: f32, steps: u32) {
        self.world.step(dt, steps);
    }

    fn draw(&self, render: &mut Renderer) {
        fn convert_color(color: HexColor) -> Color {
            let n = color.to_uint();
            let r = n >> 16;
            let g = n >> 8;
            let b = n >> 0;
            Color::RGB(r as u8, g as u8, b as u8)
        }

        let draw_opts = box2d3::debug_draw::DebugDraw::<Renderer> {
            draw_polygon: |_, _, _, _| println!("draw_polygon"),
            draw_solid_polygon: |xform, verts, vert_count, radius, color, render| {
                let render: &mut Renderer = unsafe { std::mem::transmute(render) };

                let vert_count = vert_count as usize;
                let mut vert_buffer = [super::Vec2::ZERO; 8];
                assert!(vert_count < vert_buffer.len());

                unsafe {
                    for i in 0..vert_count {
                        let v = verts.add(i).read();
                        vert_buffer[i] = &xform * v;
                    }
                }

                render.draw_polygon(&vert_buffer[..(vert_count as usize)], convert_color(color));
            },
            draw_circle: |_, _, _, _| println!("draw_circle"),
            draw_solid_circle: |_, _, _, _| println!("draw_solid_circle"),
            draw_capsule: |_, _, _, _, _| println!("draw_capsule"),
            draw_solid_capsule: |_, _, _, _, _| println!("draw_solid_capsule"),
            draw_segment: |_, _, _, _| println!("draw_segment"),
            draw_transform: |_, _| println!("draw_transform"),
            draw_point: |_, _, _, _| println!("draw_point"),
            draw_string: |_, _, _| println!("draw_string"),
            drawing_bounds: box2d3::math::AABB {
                lower_bound: box2d3::Vec2 { x: 0.0, y: 0.0 },
                upper_bound: box2d3::Vec2 { x: 0.0, y: 0.0 },
            },
            use_drawing_bounds: false,
            draw_shapes: true,
            draw_joints: false,
            draw_joint_extras: false,
            draw_aabbs: false,
            draw_mass: false,
            draw_contacts: false,
            draw_graph_colors: false,
            draw_contact_normals: false,
            draw_contact_impulses: false,
            draw_friction_impulses: false,
            context: render,
        };

        self.world.debug_draw(&draw_opts);
    }
}

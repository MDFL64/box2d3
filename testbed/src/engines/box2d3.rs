use macroquad::{color::Color, models::draw_mesh, shapes::{draw_poly, draw_triangle}};

use super::UnsupportedError;

pub struct Engine {
    world: box2d3::World
}

impl Engine {
    pub fn new() -> Self {
        let world_def = box2d3::WorldDef::default();
        let world = box2d3::World::new(&world_def);

        Self {
            world
        }
    }
}

fn convert_vec2(v: super::Vec2) -> box2d3::Vec2 {
    box2d3::Vec2{x: v.x, y: v.y}
}

fn convert_vec2_back(v: box2d3::Vec2) -> super::Vec2 {
    super::Vec2{x: v.x, y: v.y}
}

impl super::Engine for Engine {
    fn add_body(&mut self, def: super::BodyDef) -> Result<(),UnsupportedError> {
        let mut b2d_def = box2d3::BodyDef::default();
        b2d_def.position = convert_vec2(def.position);
        b2d_def.linear_velocity = convert_vec2(def.linear_velocity);
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

                    let hull_verts: Vec<_> = polygon.vertices.iter().copied().map(convert_vec2).collect();
                    let hull = box2d3::shapes::Hull::compute(&hull_verts);

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

    fn draw(&self) {
        let draw_opts = box2d3::debug_draw::DebugDraw::<()> {
            draw_polygon: |_, _, _, _| {
                println!("draw_polygon")
            },
            draw_solid_polygon: |xform, verts, vert_count, radius, color, draw| {
                let vert_count = vert_count as usize;
                let mut vert_buffer = [super::Vec2::ZERO;8];
                assert!(vert_count < vert_buffer.len());

                unsafe {
                    for i in 0..vert_count {
                        let v = verts.add(i).read();
                        vert_buffer[i] = convert_vec2_back(&xform * v);
                    }

                    let color = Color::from_hex(color.to_uint());
                    let base = vert_buffer[0];

                    for i in 1..vert_count-1 {
                        draw_triangle(base,vert_buffer[i],vert_buffer[i+1],color);
                    }
                }
            },
            draw_circle: |_, _, _, _| {
                println!("draw_circle")
            },
            draw_solid_circle: |_, _, _, _| {
                println!("draw_solid_circle")
            },
            draw_capsule: |_, _, _, _, _| {
                println!("draw_capsule")
            },
            draw_solid_capsule: |_, _, _, _, _| {
                println!("draw_solid_capsule")
            },
            draw_segment: |_, _, _, _| {
                println!("draw_segment")
            },
            draw_transform: |_, _| {
                println!("draw_transform")
            },
            draw_point: |_, _, _, _| {
                println!("draw_point")
            },
            draw_string: |_, _, _| {
                println!("draw_string")
            },
            drawing_bounds: box2d3::math::AABB {
                lower_bound: box2d3::Vec2{x: 0.0, y: 0.0},
                upper_bound: box2d3::Vec2{x: 0.0, y: 0.0},
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
            context: std::ptr::null_mut(),
        };

        self.world.debug_draw(&draw_opts);
    }
}

use crate::engines::{BodyDef, Circle, Engine, Polygon};
use crate::Vec2;

pub static TESTS: &[(&str, fn(&mut dyn Engine))] = &[
    ("Bench: Skinny Towers", test_skinny_towers),
    ("Test: Material Properties", test_mat_props),
];

pub fn test_skinny_towers(engine: &mut dyn Engine) {
    let size = 50;
    {
        let offset = size as f32 / 2.0;

        for x in 0..size {
            for y in 0..size {
                engine
                    .add_body(BodyDef::new(
                        Vec2::new((x as f32 - offset) * 2.0 + 1.0, y as f32 - offset + 20.0),
                        vec![Polygon::new_box(1.0, 1.0).into()],
                    ))
                    .unwrap();
            }
        }
    }

    // ground
    engine
        .add_body(
            BodyDef::new(
                Vec2::new(0.0, 0.0),
                vec![Polygon::new_box(100.0, 50.0)
                    .offset(Vec2::new(0.0, -50.0))
                    .into()],
            )
            .set_static(),
        )
        .unwrap();
}

pub fn test_mat_props(engine: &mut dyn Engine) {
    // ground
    for i in 0..10 {
        engine
            .add_body(
                BodyDef::new(
                    Vec2::new(0.0, 0.0),
                    vec![Polygon::new_box(40.0, 1.0)
                        .offset(Vec2::new(0.0, -100.0))
                        .rotate(i as f32 * 5.0)
                        .offset(Vec2::new(60.0, 80.0))
                        .into()],
                )
                .set_static(),
            )
            .unwrap();
    }
    engine
        .add_body(
            BodyDef::new(
                Vec2::new(0.0, 0.0),
                vec![Polygon::new_box(40.0, 1.0)
                    .offset(Vec2::new(-50.0, -40.0))
                    .into()],
            )
            .set_static(),
        )
        .unwrap();

    // friction boxes
    for i in 0..6 {
        let offset = (i as f32) * 4.5;
        engine
            .add_body(
                BodyDef::new(
                    Vec2::new(-offset, offset),
                    vec![Polygon::new_box(4.0, 1.0).into()],
                )
                .friction((i as f32) * 0.2),
            )
            .unwrap();
    }

    // bouncy circles
    for i in 0..6 {
        let offset = (i as f32) * 4.5;
        engine
            .add_body(
                BodyDef::new(Vec2::new(offset - 60.0, 0.0), vec![Circle::new(2.0).into()])
                    .restitution((i as f32) * 0.2),
            )
            .unwrap();
    }
}

use crate::engines::{BodyDef, Engine, Polygon};
use crate::Vec2;

pub fn start_test(engine: &mut dyn Engine) {
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

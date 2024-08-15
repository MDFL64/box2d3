use std::time::Instant;

use engines::{init_engine, BodyDef, Polygon};
use macroquad::prelude::*;

mod engines;

const START_PAUSED: bool = false;
const DELTA_TIME: f32 = 1.0 / 60.0;
const STEPS: u32 = 5;
const VIEW_SIZE: f32 = 60.0;

#[macroquad::main("Test Bed")]
async fn main() {
    let mut running = START_PAUSED;

    let mut engine = init_engine("box2d3").unwrap();

    let size = 50;
    {
        let offset = size as f32 / 2.0;

        for x in 0..size {
            for y in 0..size {
                engine
                    .add_body(BodyDef::new(
                        Vec2::new((x as f32 - offset) * 2.0, y as f32 - offset + 20.0),
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

    loop {
        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }

        clear_background(BLACK);

        set_world_camera();
        let sim_time = if running {
            let sim_start = Instant::now();
            engine.step(DELTA_TIME, STEPS);
            Some(sim_start.elapsed())
        } else {
            None
        };
        engine.draw();

        set_default_camera();
        if let Some(sim_time) = sim_time {
            draw_text(
                &format!("physics update: {:?}", sim_time),
                10.0,
                30.0,
                32.0,
                WHITE,
            );
        }

        next_frame().await
    }
}

fn set_world_camera() {
    let aspect = screen_width() / screen_height();
    let (w, h) = if aspect > 1.0 {
        (VIEW_SIZE * aspect, VIEW_SIZE)
    } else {
        (VIEW_SIZE, VIEW_SIZE / aspect)
    };

    let camera = Camera2D::from_display_rect(Rect::new(-w / 2.0, -h / 2.0, w, h));
    set_camera(&camera);
}

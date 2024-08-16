use std::cell::{OnceCell, RefCell};
use std::{sync::Mutex, time::Instant};

use engines::{init_engine, BodyDef, Engine, Polygon};

// use box2d3 vectors for config
use box2d3::Vec2;
use renderer::{Color, Renderer};
use sdl2::render::Canvas;
use sdl2::video::Window;

mod engines;
mod renderer;

const START_PAUSED: bool = false;
const DELTA_TIME: f32 = 1.0 / 60.0;
const STEPS: u32 = 5;
const VIEW_SIZE: f32 = 60.0;
fn main() {
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

    let render = Renderer::new();

    run_loop(State {
        render,
        engine,
        running: !START_PAUSED,
    });
}

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::new(None);
}

struct State {
    running: bool,
    engine: Box<dyn Engine>,
    render: Renderer,
}

fn run_loop(state: State) {
    STATE.with(|cell| {
        *cell.borrow_mut() = Some(state);
    });

    unsafe {
        emscripten_set_main_loop(loop_helper, 0, 0);
    }
}

fn loop_inner(state: &mut State) {
    let sim_time = if state.running {
        let t = get_time_ms();
        state.engine.step(DELTA_TIME, STEPS);
        Some(get_time_ms() - t)
    } else {
        None
    };
    println!("{:?}", sim_time);

    state.render.clear(Color::RGB(10, 10, 10));

    state.engine.draw(&mut state.render);

    state.render.present();
}

extern "C" fn loop_helper() {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        loop_inner(state.as_mut().unwrap());
    });
}

extern "C" {
    fn emscripten_set_main_loop(func: extern "C" fn(), fps: u32, simulate_infinite_loop: u32);
}

/*fn set_world_camera() {
    let aspect = screen_width() / screen_height();
    let (w, h) = if aspect > 1.0 {
        (VIEW_SIZE * aspect, VIEW_SIZE)
    } else {
        (VIEW_SIZE, VIEW_SIZE / aspect)
    };

    let camera = Camera2D::from_display_rect(Rect::new(-w / 2.0, -h / 2.0, w, h));
    set_camera(&camera);
}*/

pub fn get_time_ms() -> f64 {
    extern "C" {
        fn perf_now() -> f64;
    }
    return unsafe { perf_now() };
}

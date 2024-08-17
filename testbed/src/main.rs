use std::cell::RefCell;
use std::time::Instant;

use engines::{init_engine, BodyDef, Engine, Polygon};

// use box2d3 vectors for config
use box2d3::Vec2;
use renderer::{Color, Renderer};

mod engines;
mod renderer;

const START_PAUSED: bool = false;
const DELTA_TIME: f32 = 1.0 / 60.0;
const STEPS: u32 = 5;

fn main() {
    let mut engine = init_engine("box2d3").unwrap();

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

#[cfg(target_arch = "wasm32")]
fn run_loop(state: State) {
    STATE.with(|cell| {
        *cell.borrow_mut() = Some(state);
    });

    extern "C" fn loop_helper() {
        STATE.with(|cell| {
            let mut state = cell.borrow_mut();
            loop_inner(state.as_mut().unwrap());
        });
    }

    extern "C" {
        fn emscripten_set_main_loop(func: extern "C" fn(), fps: u32, simulate_infinite_loop: u32);
    }
    unsafe {
        emscripten_set_main_loop(loop_helper, 0, 0);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run_loop(mut state: State) {
    loop {
        loop_inner(&mut state);
    }
}

fn loop_inner(state: &mut State) {
    state.render.poll_events();

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

    state.render.draw_ui();

    state.render.present();
}

#[cfg(target_arch = "wasm32")]
pub fn get_time_ms() -> f64 {
    extern "C" {
        fn perf_now() -> f64;
    }
    unsafe { perf_now() }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_time_ms() -> f64 {
    use std::sync::OnceLock;
    static START: OnceLock<Instant> = OnceLock::new();

    let start = START.get_or_init(|| Instant::now());
    start.elapsed().as_secs_f64() * 1000.0
}

use std::cell::RefCell;
use std::time::Instant;

use engines::{BodyDef, Engine, Polygon, ENGINES};

// use box2d3 vectors for config
use box2d3::{common::HexColor, Vec2};
use renderer::Renderer;
use tests::TESTS;

mod engines;
mod tests;

mod renderer;

const START_PAUSED: bool = false;
const DELTA_TIME: f32 = 1.0 / 60.0;
const STEPS: u32 = 5;

fn main() {
    let engine_index = 0;
    let test_index = 0;

    let mut engine = ENGINES[engine_index].1();
    TESTS[test_index].1(engine.as_mut());

    let render = Renderer::new();

    run_loop(State {
        engine_index,
        test_index,
        render,
        engine,
        running: !START_PAUSED,
        perf_info: Default::default(),
        stop_step: 1000,
    });
}

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::new(None);
}

struct State {
    running: bool,
    engine: Box<dyn Engine>,
    engine_index: usize,
    test_index: usize,
    render: Renderer,
    perf_info: PerfInfo,
    stop_step: i32,
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

    if state.running {
        let t = get_time_ms();
        state.engine.step(DELTA_TIME, STEPS);
        let step_time = get_time_ms() - t;

        let f = 0.1;
        state.perf_info.step_time = state.perf_info.step_time * (1.0 - f) + step_time * f;
        state.perf_info.step_sum += step_time;
        state.perf_info.step_count += 1;

        if state.perf_info.step_count >= state.stop_step as usize {
            state.running = false;
        }
    } else {
        state.perf_info.step_time = 0.0;
    }

    state.render.clear(HexColor::new(0x111111));

    state.engine.draw(&mut state.render);

    state.render.draw_buffered_shapes(0.01);

    let reset = state.render.draw_ui(
        &mut state.engine_index,
        &mut state.test_index,
        &state.perf_info,
        &mut state.stop_step,
    );

    state.render.present();

    if reset {
        reset_state(state);
    }
}

fn reset_state(state: &mut State) {
    let mut new_engine = ENGINES[state.engine_index].1();
    TESTS[state.test_index].1(new_engine.as_mut());

    state.engine = new_engine;
    state.perf_info = Default::default();
    state.running = true;
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

#[derive(Default)]
struct PerfInfo {
    step_time: f64,
    step_sum: f64,
    step_count: usize,
}

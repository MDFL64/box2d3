use box2d3::Vec2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::EventPump;
use sdl2::{render::Canvas, video::Window};

use glow::HasContext;

pub use sdl2::pixels::Color;

pub struct Renderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    imgui: imgui::Context,
    imgui_platform: imgui_sdl2_support::SdlPlatform,
    imgui_render: imgui_glow_renderer::AutoRenderer,
    // if we don't safe a reference to this, imgui will break
    _gl_context: sdl2::video::GLContext,
    window_w: u32,
    window_h: u32,
    scale: f32,
    buffer_x: Vec<i16>,
    buffer_y: Vec<i16>,
}

fn glow_context(window: &sdl2::video::Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

impl Renderer {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        {
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0);
        }

        let w = 1600;
        let h = 800;

        let window = video_subsystem
            .window("Test Bed", w, h)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window
            .gl_create_context()
            .expect("Couldn't create GL context");
        window.gl_make_current(&gl_context).unwrap();

        let mut imgui = imgui::Context::create();
        let imgui_platform = imgui_sdl2_support::SdlPlatform::init(&mut imgui);

        let gl = glow_context(&window);
        let imgui_render = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui).unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Renderer {
            canvas,
            event_pump,
            imgui,
            imgui_platform,
            imgui_render,
            _gl_context: gl_context,
            window_w: w,
            window_h: h,
            scale: 10.0,
            buffer_x: vec![],
            buffer_y: vec![],
        }
    }

    pub fn poll_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            self.imgui_platform.handle_event(&mut self.imgui, &event);
            match event {
                Event::Quit { .. } => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }

    pub fn clear(&mut self, color: Color) {
        //self.canvas.set_draw_color(color);
        //self.canvas.clear();

        let gl = self.imgui_render.gl_context();
        unsafe {
            let r = color.r as f32 / 255.0;
            let g = color.g as f32 / 255.0;
            let b = color.b as f32 / 255.0;
            let a = color.a as f32 / 255.0;
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        // update window size
        //let (w, h) = self.canvas.window().size();
        //self.window_w = w;
        //self.window_h = h;
    }

    pub fn present(&mut self) {
        self.canvas.window().gl_swap_window();
    }

    fn pos_to_screen(&self, pos: Vec2) -> (i16, i16) {
        let x = (pos.x * self.scale) as i16 + (self.window_w / 2) as i16;
        let y = (pos.y * -self.scale) as i16 + (self.window_h / 2) as i16;
        return (x, y);
    }

    fn magnitude_to_screen(&self, n: f32) -> i16 {
        (n * self.scale) as i16
    }

    pub fn draw_circle(&mut self, pos: Vec2, radius: f32, color: Color) {
        let (x, y) = self.pos_to_screen(pos);
        let rad = self.magnitude_to_screen(radius);
        self.canvas.filled_circle(x, y, rad, color).unwrap();
    }

    pub fn draw_polygon(&mut self, points: &[Vec2], color: Color) {
        /*self.buffer_x.resize(points.len(), 0);
        self.buffer_y.resize(points.len(), 0);

        for (i, p) in points.iter().enumerate() {
            let (x, y) = self.pos_to_screen(*p);

            self.buffer_x[i] = x;
            self.buffer_y[i] = y;
        }

        self.canvas
            .filled_polygon(&self.buffer_x, &self.buffer_y, color)
            .unwrap();*/
    }

    pub fn draw_ui(&mut self) {
        self.imgui_platform
            .prepare_frame(&mut self.imgui, self.canvas.window(), &self.event_pump);

        let ui = self.imgui.frame();
        ui.show_demo_window(&mut true);

        self.imgui_render.render(self.imgui.render()).unwrap();
    }
}

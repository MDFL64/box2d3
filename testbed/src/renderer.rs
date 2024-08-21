use box2d3::common::HexColor;
use box2d3::Vec2;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::video::Window;

use glow::{HasContext, NativeBuffer, NativeProgram};

pub struct Renderer {
    window: Window,
    event_pump: EventPump,
    imgui: imgui::Context,
    imgui_platform: imgui_sdl2_support::SdlPlatform,
    imgui_render: imgui_glow_renderer::AutoRenderer,
    // if we don't save a reference to this, imgui will break
    _gl_context: sdl2::video::GLContext,
    draw_program: NativeProgram,
    draw_buffer: NativeBuffer
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
        // gl setup
        let draw_buffer;
        let draw_program;
        unsafe {
            let data: [f32;9] = [
                -0.5, -0.5, 0.0,
                0.5, -0.5, 0.0,
                0.0,  0.5, 0.0
            ];
            let data_bytes: [u8;36] = std::mem::transmute(data);

            draw_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(draw_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &data_bytes, glow::STATIC_DRAW);

            let shader_vertex = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(shader_vertex, include_str!("vertex.glsl"));
            gl.compile_shader(shader_vertex);

            let shader_fragment = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(shader_fragment, include_str!("fragment.glsl"));
            gl.compile_shader(shader_fragment);

            draw_program = gl.create_program().unwrap();
            gl.attach_shader(draw_program, shader_vertex);
            gl.attach_shader(draw_program, shader_fragment);
            gl.link_program(draw_program);

            let link_status = gl.get_program_link_status(draw_program);
            if !link_status {
                let log = gl.get_program_info_log(draw_program);
                println!("{}",log);
                panic!("shader compile failed");
            }
        }

        let imgui_render = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui).unwrap();

        //let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Renderer {
            window,
            event_pump,
            imgui,
            imgui_platform,
            imgui_render,
            _gl_context: gl_context,
            draw_buffer,
            draw_program
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

    pub fn clear(&mut self, color: HexColor) {
        //self.canvas.set_draw_color(color);
        //self.canvas.clear();

        let gl = self.imgui_render.gl_context();
        unsafe {
            let [r,g,b] = color.to_floats();
            gl.clear_color(r, g, b, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        // update window size
        //let (w, h) = self.canvas.window().size();
        //self.window_w = w;
        //self.window_h = h;
    }

    pub fn present(&mut self) {
        self.window.gl_swap_window();
    }

    fn pos_to_screen(&self, pos: Vec2) -> (i16, i16) {
        /*let x = (pos.x * self.scale) as i16 + (self.window_w / 2) as i16;
        let y = (pos.y * -self.scale) as i16 + (self.window_h / 2) as i16;
        return (x, y);*/
        (0,0)
    }

    fn magnitude_to_screen(&self, n: f32) -> i16 {
        //(n * self.scale) as i16
        0
    }

    pub fn draw_circle(&mut self, pos: Vec2, radius: f32, color: HexColor) {
        /*let (x, y) = self.pos_to_screen(pos);
        let rad = self.magnitude_to_screen(radius);
        self.canvas.filled_circle(x, y, rad, color).unwrap();*/
    }

    pub fn draw_polygon(&mut self, points: &[Vec2], color: HexColor) {
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

    /// Draw shape geometry buffered by other calls
    pub fn draw_buffered_shapes(&self) {
        let gl = self.imgui_render.gl_context();
        let (w,h) = self.window.drawable_size();
        let aspect = w as f32 / h as f32;
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.draw_buffer));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);
            gl.use_program(Some(self.draw_program));
            let scale_loc = gl.get_uniform_location(self.draw_program, "scale").unwrap();
            gl.uniform_2_f32(Some(&scale_loc), 0.1, 0.1 * aspect);

            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    pub fn draw_ui(&mut self) {
        self.imgui_platform
            .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        let ui = self.imgui.frame();
        ui.show_demo_window(&mut true);

        self.imgui_render.render(self.imgui.render()).unwrap();
    }
}

use box2d3::Vec2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{render::Canvas, video::Window};

pub use sdl2::pixels::Color;

pub struct Renderer {
    canvas: Canvas<Window>,
    window_w: u32,
    window_h: u32,
    scale: f32,
    buffer_x: Vec<i16>,
    buffer_y: Vec<i16>,
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
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Renderer {
            canvas,
            window_w: w,
            window_h: h,
            scale: 10.0,
            buffer_x: vec![],
            buffer_y: vec![],
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();

        // update window size
        let (w, h) = self.canvas.window().size();
        self.window_w = w;
        self.window_h = h;
    }

    pub fn present(&mut self) {
        self.canvas.present();
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
        self.buffer_x.resize(points.len(), 0);
        self.buffer_y.resize(points.len(), 0);

        for (i, p) in points.iter().enumerate() {
            let (x, y) = self.pos_to_screen(*p);

            self.buffer_x[i] = x;
            self.buffer_y[i] = y;
        }

        self.canvas
            .filled_polygon(&self.buffer_x, &self.buffer_y, color)
            .unwrap();
    }
}

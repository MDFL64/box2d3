#[repr(C)]
#[derive(Clone, Debug)]
pub struct Filter {
    pub category_bits: u32,
    pub mask_bits: u32,
    pub group_index: i32,
}

#[repr(transparent)]
pub struct HexColor(u32);

impl HexColor {
    pub fn new(hex: u32) -> Self {
        Self(hex)
    }

    pub fn new_from_floats(r: f32, g: f32, b: f32) -> Self {
        let r = (r * 255.0) as u32;
        let g = (g * 255.0) as u32;
        let b = (b * 255.0) as u32;

        Self((r << 16) | (g << 8) | (b << 0))
    }

    pub fn to_uint(&self) -> u32 {
        self.0
    }

    pub fn to_floats(&self) -> [f32; 3] {
        let r = ((self.0 >> 16) & 0xFF) as f32 / 255.0;
        let g = ((self.0 >> 8) & 0xFF) as f32 / 255.0;
        let b = ((self.0 >> 0) & 0xFF) as f32 / 255.0;

        [r, g, b]
    }
}

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

    pub fn to_uint(&self) -> u32 {
        self.0
    }

    pub fn to_floats(&self) -> [f32;3] {
        let r = ((self.0 >> 16) & 0xFF) as f32 / 255.0;
        let g = ((self.0 >> 8) & 0xFF) as f32 / 255.0;
        let b = ((self.0 >> 0) & 0xFF) as f32 / 255.0;

        [r,g,b]
    }
}

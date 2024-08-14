#[repr(C)]
#[derive(Clone, Debug)]
pub struct Filter {
    pub category_bits: u16,
    pub mask_bits: u16,
    pub group_index: i16,
}

#[repr(transparent)]
pub struct HexColor(u32);

impl HexColor {
    pub fn to_uint(&self) -> u32 {
        self.0
    }
}

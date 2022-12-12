#[derive(Debug, Default, Clone, Copy)]
pub struct MagicEntry {
    pub mask: u64,
    pub magic: u64,
    pub shift: u32,
    pub size: usize,
}

impl MagicEntry {
    pub fn new(mask: u64, magic: u64, shift: u32, size: usize) -> Self {
        Self {
            mask,
            magic,
            shift,
            size,
        }
    }
}

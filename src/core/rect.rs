#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPoint {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TRect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl TRect {
    pub fn contains(&self, pt: TPoint) -> bool {
        pt.x >= self.x && pt.x < self.x + self.width &&
        pt.y >= self.y && pt.y < self.y + self.height
    }
}

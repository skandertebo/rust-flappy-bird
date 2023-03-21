pub struct Entity {
    pub x: f32,
    pub y: f32,
}

impl Entity {
    pub fn new(x: f32, y: f32) -> Entity {
        Entity { x, y }
    }
}

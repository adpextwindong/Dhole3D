use renderer::vector::Vec2 as Vec2;

#[derive(Copy, Clone)]
pub struct Player {
    ///Their position in the world
    pub pos: Vec2<f32>,
    ///Direction their facing
    pub dir: Vec2<f32>,
}

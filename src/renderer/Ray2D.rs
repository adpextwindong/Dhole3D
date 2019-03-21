use renderer::vector::Vec2;

/// Represents a ray for raycasting
#[derive(Copy, Clone, Debug)]
pub struct Ray2D {
    /// b in y = mx + b
    pub initial_position: Vec2<f32>,
    /// m in y = mx + b
    pub dir: Vec2<f32>
}

//TODO this is dumb and should be refactored
impl Ray2D {
    pub fn new(dir: Vec2<f32>, pos: Vec2<f32>) -> Ray2D {
        let norm_dir = dir.normalized();
        Ray2D {
            initial_position: pos,
            dir : norm_dir,
        }
    }

    pub fn get_dir(self) -> Vec2<f32> {
        self.dir
    }
}
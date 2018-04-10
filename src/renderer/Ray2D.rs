use renderer::vector::Vec2;

/// Represents a ray for raycasting
#[derive(Debug)]
pub struct Ray2D {
    /// b in y = mx + b
    pub y_intercept: f32,
    /// m in y = mx + b
    pub dir: Vec2<f32>
}

impl Ray2D {
    pub fn new(dir: Vec2<f32>, y_pos: f32) -> Ray2D {
        Ray2D {
            y_intercept: y_pos,
            dir,
        }
    }
    pub fn at(&self, xtarg: f32) -> f32 {
        (xtarg * self.get_slope()) + self.y_intercept
    }

    pub fn get_y_intercept(self) -> f32 {
        self.y_intercept
    }

    pub fn get_slope(self) -> f32 {
        self.dir.slope()
    }

    pub fn get_dir(self) -> Vec2<f32> {
        self.dir
    }
}
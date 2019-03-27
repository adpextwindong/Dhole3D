use renderer::vector::Vec2;

/// Represents a ray for raycasting
#[derive(Copy, Clone, Debug)]
pub struct Ray2D {
    pub dir: Vec2<f32>
}

//TODO this is dumb and should be refactored
impl Ray2D {
    pub fn new(player_direction: Vec2<f32>, plane_direction: Vec2<f32>, cameraX : f32) -> Ray2D {
        Ray2D {
            dir : Vec2{
                x: player_direction.x + plane_direction.x * cameraX,
                y: player_direction.y + plane_direction.y * cameraX,
            },
        }
    }

    pub fn get_dir(self) -> Vec2<f32> {
        self.dir
    }
}
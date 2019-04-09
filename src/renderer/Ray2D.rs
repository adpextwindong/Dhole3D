use renderer::vector::Vec2;
use world::WORLD_CELL_SIZE;

/// Represents a ray for raycasting
#[derive(Copy, Clone, Debug)]
pub struct Ray2D {
    pub dir: Vec2<f32>,
    pub ray_number: usize
}

//TODO this is dumb and should be refactored
impl Ray2D {
    pub fn new(player_direction: Vec2<f32>, plane_direction: Vec2<f32>, cameraX : f32, ray_number :usize) -> Ray2D {
        Ray2D {
            dir : Vec2{
                // FIXME?
                x: player_direction.x + plane_direction.x * cameraX,
                y: player_direction.y + plane_direction.y * cameraX,
            }.normalized(),
            ray_number
        }
    }

    pub fn gen_dda_steps(self) -> (f32,f32) {
        // X + 1 , A + ystep
        let mut scaled_dir_steps = self.dir.normalized().scale(WORLD_CELL_SIZE as f32);

        if scaled_dir_steps.x.is_infinite(){
            scaled_dir_steps.x = WORLD_CELL_SIZE as f32;
        }

        if scaled_dir_steps.y.is_infinite(){
            scaled_dir_steps.y = WORLD_CELL_SIZE as f32;
        }

        (scaled_dir_steps.x, scaled_dir_steps.y)
    }

    pub fn get_dir(self) -> Vec2<f32> {
        self.dir
    }
}
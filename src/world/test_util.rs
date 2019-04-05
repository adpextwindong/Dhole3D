use renderer::vector::Vec2;

use super::player::Player;
use super::Wall;

use super::gen_blank_world;
use super::WORLD_CELL_SIZE;
use super::WORLD_SIZE_X;
use super::WORLD_SIZE_Y;
use world::wall::GREEN;
use world::wall::BLUE;
use world::wall::RED;
use world::GameState;
use renderer::vector::rotate_counter_clockwise;

use SCREEN_SIZE_X;
use world::DebugWindowFlags;
use renderer::vector::rotate_clockwise;

use std::f32;




pub fn generate_test_world() -> GameState{

    let mut theworld = gen_blank_world(WORLD_SIZE_X, WORLD_SIZE_Y);

    let p: Player = Player {
        pos: Vec2 {
            x: 5.0 * WORLD_CELL_SIZE as f32,
            y: 5.0 * WORLD_CELL_SIZE as f32,
        },
        dir: Vec2 {
            x: 0.0,//f32::consts::FRAC_PI_2,
            y: 1.0//f32::consts::FRAC_PI_2,
        },
    };

    p.dir.normalize();
    assert!(!(p.dir.x == 0.0 && p.dir.y == 0.0));
    //Dir not equal to null vector

    let red_wall = Wall {
        full: true,
        color: RED,
    };
    let blue_wall = Wall {
        full: true,
        color: BLUE,
    };
    let green_wall = Wall {
        full: true,
        color: GREEN,
    };

    for i in 0..WORLD_SIZE_X as usize {
        theworld[0][i] = red_wall;
        theworld[WORLD_SIZE_Y as usize - 1 as usize][i] = green_wall;

        theworld[i][0] = blue_wall;
        theworld[i][WORLD_SIZE_X as usize - 1 as usize] = green_wall;
    }
    //NOW Test up down and left right

    theworld[0][4] = Wall {
        full: true,
        color: BLUE,
    };

//    theworld[5][2] = Wall {
//        full: true,
//        color: BLUE,
//    };
//    theworld[5][4] = Wall {
//        full: true,
//        color: GREEN,
//    };



    GameState{
        the_world: theworld,
        p,
        camera_plane: rotate_clockwise(p.dir,f32::consts::FRAC_PI_4),
    }
}
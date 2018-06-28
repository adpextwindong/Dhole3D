extern crate sdl2;
extern crate sdl2_window;
extern crate shader_version;
extern crate window;
extern crate num_traits;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::f32;
use std::time::{Duration, Instant};
use std::io::{self, Read};

mod renderer;
use renderer::vector::Vec2 as Vec2;
use renderer::vector::{rotate_clockwise, rotate_counter_clockwise};
use renderer::ray2D::Ray2D;

mod world;
use world::wall::Wall as Wall;
use world::wall::NULL_COLOR as NULL_COLOR;
use world::wall::FLOOR_GREY as FLOOR_GREY;
use world::wall::RED as RED;
use world::wall::GREEN as GREEN;

use world::player::Player as Player;


use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::Window;
use sdl2::render::Canvas;
//use window::WindowSettings;


//Types TODO
//Player obj
//Map type
//World map var
//Wall Type

// TODO https://www.scratchapixel.com/




//WORLD CONSTANTS
//TODO make this flexible for loading seperate worlds
const WORLD_SIZE_X: usize = 10;
const WORLD_SIZE_Y: usize = 10;

const WORLD_CELL_SIZE: u32 = 10; // 10 Meters?

//TODO make this a startup option or something
const SCREEN_SIZE_X: u32 = 800;
const SCREEN_SIZE_Y: u32 = 600;

//TODO finish this
//TODO Add filepath option or make an INTO/FROM for a load source

fn gen_blank_world(x: usize, y: usize) -> Vec<Vec<Wall>> {
    let mut ret = Vec::new();
    let reg_wall = Wall {
        full: false,
        color: NULL_COLOR,
    };
    for _ in 0..y {
        let mut w = Vec::<Wall>::with_capacity(x as usize);
        for _ in 0..x {
            w.push(reg_wall);
        }
        ret.push(w);
    }


    return ret;
}

//TODO make these colors world constants
//TODO_FAR add support for different colored ceilings
//      and actually make them world surfaces once we get to true'r 3d
fn draw_ceiling(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    //FIX ME This expects to be ran first
    canvas.clear();
}

fn draw_floor(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(FLOOR_GREY);
    let pos_y = SCREEN_SIZE_Y as i32 / 2;
    let size_y = SCREEN_SIZE_Y as i32 / 2;
    canvas
        .fill_rect(Rect::new(0, pos_y, SCREEN_SIZE_X, size_y as u32))
        .unwrap();
}

fn debug_print_player(p: Player) {
    println!("POS: {} {} DIR: {} {}", p.pos.x, p.pos.y, p.dir.x, p.dir.y);
}




fn find_next_cell_boundary(line_pos: f32, positive: bool) -> i32 {
    // TODO Make tests for this
    // |   * |  positive gives 64
    // |_*___|  negative gives 0
    // 0 ..  64
    if positive {
        WORLD_CELL_SIZE as i32 * ((line_pos.floor() as i32 / WORLD_CELL_SIZE as i32) + 1)
    } else {
        WORLD_CELL_SIZE as i32 * ((line_pos.floor() as i32 / WORLD_CELL_SIZE as i32) - 1)
    }
}

fn out_of_world_bounds(pos: Vec2<f32>) -> bool {
    if pos.x == std::f32::INFINITY || pos.y == std::f32::INFINITY || pos.x == -std::f32::INFINITY || pos.y == -std::f32::INFINITY {
        return true;
    }else{
        return (pos.x >= (WORLD_SIZE_X as f32 * WORLD_CELL_SIZE as f32)) ||
            (pos.y >= (WORLD_SIZE_Y as f32 * WORLD_CELL_SIZE as f32))
    }

}

fn get_world_cell_at_vec2_pos(pos: Vec2<f32>, w: &Vec<Vec<Wall>>) -> Wall {
    let x: usize = (pos.x.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;
    let y: usize = (pos.y.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;
    w[x][y]
}

fn debug_print_world(w: &Vec<Vec<Wall>>, pos : Vec2<f32>) {
    let x: usize = (pos.x.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;
    let y: usize = (pos.y.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;

    for i in 0..WORLD_SIZE_Y {
        for j in 0..WORLD_SIZE_X {
            if x == j && y == i {
                print!("p");
            }else if w[i][j].full {
                print!("1");
            }else {
                print!("0");
            }
        }
        println!();
    }
}
//
//fn check_wall_at_vec2_pos(pos : Vec2,w : &Vec<Vec<Wall>>) -> bool{
//    // TODO REFACTOR INTO WORLD CLASS? ASSERT THAT THESE ARE POSITIVE
//    // FLOOR??
//    let x : usize = (pos.x.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;
//    let y : usize = (pos.y.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;
//
//    w[x][y].full
//}

fn draw_col(buffer: &mut [u8], pitch: usize, x: usize, color: Color) {
    for y in 0..SCREEN_SIZE_Y as usize {
        let offset = y * pitch + x * 3;
        buffer[offset] = color.r as u8;
        buffer[offset + 1] = color.g as u8;
        buffer[offset + 2] = color.b as u8;
    }
}

fn find_axis_intersections(position : Vec2<f32>, ray: Ray2D) -> (Vec2<f32>, Vec2<f32>){
    let a_x = find_next_cell_boundary(position.x, true);
    let b_y = find_next_cell_boundary(position.y, true);

    let x_axis_inter: Vec2<f32> = Vec2::<f32> {
        x: a_x as f32,
        y: ray.at(a_x as f32),
    };
    let y_axis_inter: Vec2<f32> = Vec2::<f32> {
        x: (b_y as f32 - ray.get_y_intercept()) / ray.get_slope(),
        y: b_y as f32,
    };

    (x_axis_inter, y_axis_inter)
}

fn advance_x_intersection(x_axis_intersection : Vec2<f32>, y_step : f32) -> Vec2<f32> {
    Vec2 {
        x: x_axis_intersection.x + WORLD_CELL_SIZE as f32,
        y: x_axis_intersection.y + y_step,
    }
}

fn advance_y_intersection(y_axis_intersection : Vec2<f32>, x_step : f32) -> Vec2<f32> {
    Vec2 {
        x: y_axis_intersection.x + x_step,
        y: y_axis_intersection.y + WORLD_CELL_SIZE as f32,
    }
}

//TODO HEIGHT SCALING
///DDA Algorithm using initial x and y axis intersections
fn find_wall_and_distance(theworld: &Vec<Vec<Wall>>, p : &Player, ray : Ray2D) -> Option<(Wall, f32)> {
    let xstep = if p.dir.x.is_sign_positive(){
        (1.0 / ray.dir.x).abs() * WORLD_CELL_SIZE as f32
    }else{
        -1.0 * (1.0 / ray.dir.x).abs() * WORLD_CELL_SIZE as f32
    };

    let ystep = if p.dir.y.is_sign_positive() {
        (1.0 / ray.dir.y).abs() * WORLD_CELL_SIZE as f32
    }else {
        -1.0 * (1.0 / ray.dir.y).abs() * WORLD_CELL_SIZE as f32
    };

    let (mut x_axis_intersection,mut y_axis_intersection) = find_axis_intersections(p.pos, ray);

    //println!("\nSTEPS {:?} {:?}", xstep, ystep);
    //TODO FIX STEP == INF CASE
    //println!("PLAYER POS: {:?} RAY DIRECTION: {:?}", p.pos, ray.dir);
    'finding_wall: loop {
        let x_dir_oob: bool = out_of_world_bounds(x_axis_intersection);
        let y_dir_oob: bool = out_of_world_bounds(y_axis_intersection);

        let dist_x_inter = x_axis_intersection.dist(&p.pos);
        let dist_y_inter = y_axis_intersection.dist(&p.pos);

        if x_dir_oob && y_dir_oob { //The ray has hit out of bounds
            return None;
        }else if dist_x_inter < dist_y_inter && !x_dir_oob { //Check X intersection on grid as its closer
            //println!("X walk");
            let potential_wall : Wall = get_world_cell_at_vec2_pos(x_axis_intersection, &theworld);
            if potential_wall.full {
                //println!("WALL FOUND!!");
                return Some((potential_wall, dist_x_inter))
            }else{
                x_axis_intersection = advance_x_intersection(x_axis_intersection, ystep);
            }
        }else if !y_dir_oob { //Check Y intersection on grid as its closer
            //println!("Y walk");
            let potential_wall : Wall = get_world_cell_at_vec2_pos(y_axis_intersection, &theworld);
            if potential_wall.full {
                //println!("WALL FOUND!!");
                return Some((potential_wall, dist_y_inter))
            }else{
                y_axis_intersection = advance_y_intersection(x_axis_intersection, xstep);
            }
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", SCREEN_SIZE_X, SCREEN_SIZE_Y)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let debug_window = video_subsystem
        .window("rust-sdl2 demo: Debug Window", SCREEN_SIZE_X, SCREEN_SIZE_Y)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_SIZE_X, SCREEN_SIZE_Y)
        .unwrap();

    let mut theworld = gen_blank_world(WORLD_SIZE_X, WORLD_SIZE_Y);

    let p: Player = Player {
        pos: Vec2 {
            x: 2.0 * WORLD_CELL_SIZE as f32,
            y: 2.0 * WORLD_CELL_SIZE as f32,
        },
        dir: Vec2 {
            x: f32::consts::FRAC_PI_2,
            y: f32::consts::FRAC_PI_2,
        },
    };

    let red_wall = Wall {
        full: true,
        color: RED,
    };
    for i in 0..WORLD_SIZE_X as usize {
        theworld[0][i] = red_wall;
        theworld[WORLD_SIZE_Y as usize - 1 as usize][i] = red_wall;

        theworld[i][0] = red_wall;
        theworld[i][WORLD_SIZE_X as usize - 1 as usize] = red_wall;
    }
    theworld[5][5] = Wall {
        full: true,
        color: GREEN,
    };;
    theworld[5][2] = Wall {
        full: true,
        color: GREEN,
    };;


    debug_print_world(&theworld, p.pos);

    // TODO!! MAKE THIS MORE UNIT TESTABLE
    // TODO!! Find some way to edit the tileset easier

    //TODO_FAR Move this to a seperate renderer file that takes the world ref
    //This function renders the raycast pixels to a pixel buffer. To be used with sdl_texture
    //TODO FINISH IT
    let render_statics = |buffer: &mut [u8], pitch: usize| {
        let world: &Vec<Vec<Wall>> = &theworld;
        let p: &Player = &p;

        let fov = f32::consts::FRAC_PI_2;
        let mut ray_curr_dir = rotate_counter_clockwise(p.dir, fov / 2.0);

        let delta_theta_y = fov / SCREEN_SIZE_Y as f32;

        //  Check nearest wall adjacent to current position, scans next needbe
        //  Use distance for wall scaling
        //  figure out fish eye projection issue

        // Cast Ray
        'raycasting: for y in 0..SCREEN_SIZE_Y as usize {
            //println!("ITER {}", y);
            let ray: Ray2D = Ray2D::new(ray_curr_dir, p.pos.y);
            //println!("NEW RAY: {:?}",ray);
            let possible_wall : Option<(Wall, f32)> = find_wall_and_distance(world, p, ray);
            //TODO draw wall with height scaling
            if let Some((sampled_wall, _dist)) = possible_wall {
                draw_col(buffer, pitch, y, sampled_wall.color);
            }

            ray_curr_dir = rotate_clockwise(ray_curr_dir, delta_theta_y);
        }
        //let mut stdin = io::stdin();
        //let _ = stdin.read(&mut [0u8]).unwrap();
    };


    let mut delta: f64 = 0.0;
    let mut i = 0;
    'running: loop {
        let last_frame_instant = Instant::now();
        //Draw floor

        let mut event_pump = sdl_context.event_pump().unwrap();
        draw_ceiling(&mut canvas);
        draw_floor(&mut canvas);
        //TODO Finish the statics renderer
        //Draw statics texture
        texture.with_lock(None, &render_statics).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        //Present Frame
        canvas.present();

        let frame_duration = last_frame_instant.elapsed();
        delta = frame_duration.as_secs() as f64 + frame_duration.subsec_nanos() as f64 * 1e-9;

        //println!("{}", delta);


        //debug_print_player(p);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
        // TODO Work on gameloop once renderer is up
    }


    println!("Frame Delta : {}", delta);
}

// NOTES
// CURRENT : TODO gamestate -> pixel array (everything rendered to the window context texture)
//           TODO Get raycaster to work on a collumn level then 2D stage
//                Once simple colors are handled we should move to each wall having bitmap surfaces

// Asset handling https://rust-sdl2.github.io/rust-sdl2/sdl2/image/index.html
// ?Depth buffer
// TODO Move graphics code to renderer module
// TODO Add headbob (I guess their has to be a basic player height) to make moving around seem real
// TODO_FAR Gamestate handler, actual game once initial graphics are up

// TODO work on documentation of how the engine works so its easy to come back to
// world is 2d matrix of walls for now
// world -> pixel_array

// #Nice things
// TODO tileset editor to create json verson of maps
// TODO serialize maps to json
// TODO load maps from json via file or text entry box

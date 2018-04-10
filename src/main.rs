

extern crate sdl2;
extern crate sdl2_window;
extern crate shader_version;
extern crate window;
extern crate num_traits;

mod renderer;
use renderer::vector::Vec2 as Vec2;
use renderer::vector::{rotate_clockwise, rotate_counter_clockwise};

use renderer::ray2D::Ray2D;



use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

//use sdl2::surface::Surface;

use sdl2::video::Window;
use sdl2::render::Canvas;
//use sdl2::rect;

//use window::WindowSettings;

//TODO_FAR opengl?
//use shader_version::OpenGL;

use std::f32;
use std::time::{Duration, Instant};
use std::io;

//Types TODO
//2D Vector
//Player obj
//Map type
//World map var
//Wall Type

//?Far compression of world to file
//See how much this struct takes up in mem and alignment etc...
#[derive(Copy, Clone)]
pub struct Wall {
    full: bool,
    color: Color,
}


// TODO https://www.scratchapixel.com/


#[derive(Copy, Clone)]
struct Player {
    pos: Vec2<f32>, //Their position in the world
    dir: Vec2<f32>, //Direction their facing
}

const NULL_COLOR: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
const NULL_WALL: Wall = Wall {
    full: false,
    color: NULL_COLOR,
};

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};

const FLOOR_GREY: Color = Color {
    r: 128,
    g: 128,
    b: 128,
    a: 255,
};
const CEILING_BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

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
    (pos.x <= (WORLD_SIZE_X as f32 * WORLD_CELL_SIZE as f32)) &&
        (pos.y <= (WORLD_SIZE_Y as f32 * WORLD_CELL_SIZE as f32))
}

fn get_wall_at_vec2_pos(pos: Vec2<f32>, w: &Vec<Vec<Wall>>) -> Wall {
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

fn draw_col(buffer: &mut [u8], pitch: usize) {

    //        for y in 0..SCREEN_SIZE_Y as usize {
    //            for x in 0..SCREEN_SIZE_X as usize {
    //                let offset = y * pitch + x * 3;
    //                buffer[offset] = test_color.r as u8;
    //                buffer[offset + 1] = test_color.g as u8;
    //                buffer[offset + 2] = test_color.b as u8;
    //            }
    //        }
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

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_SIZE_X, SCREEN_SIZE_Y)
        .unwrap();

    let mut w = gen_blank_world(WORLD_SIZE_X, WORLD_SIZE_Y);

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
        w[0][i] = red_wall;
        w[WORLD_SIZE_Y as usize - 1 as usize][i] = red_wall;

        w[i][0] = red_wall;
        w[i][WORLD_SIZE_X as usize - 1 as usize] = red_wall;
    }

    w[5][5] = red_wall;

    debug_print_world(&w, p.pos);

    //TODO_FAR Move this to a seperate renderer file that takes the world ref
    //This function renders the raycast pixels to a pixel buffer. To be used with sdl_texture
    //TODO FINISH IT
    let render_statics = |buffer: &mut [u8], pitch: usize| {
        let world: &Vec<Vec<Wall>> = &w;
        let p: &Player = &p;

        //TODO Remove this shit once I got all the ray stuff working
        let mut test_color: Color = RED;
        let fov = f32::consts::FRAC_PI_2;
        let left_most_dir = rotate_counter_clockwise(p.dir, fov / 2.0);

        //let delta_theta_x = fov / SCREEN_SIZE_X as f32;
        let delta_theta_y = fov / SCREEN_SIZE_Y as f32;

        //TODO do the rotation of the player dir vector by FRAC_PI_4
        let mut curr_dir = left_most_dir;
        //Start from player position, use the ray to check


        //Use dir for x value and p.y as y intercept for rayline
        //Scan through static walls from (p.x , p.y) in that direction
        //Have cases for directions of on unit circle quadrants
        //  Dictates 2D array access
        //
        //  Check nearest wall adjacent to current position, scans next needbe
        //  Use distance for wall scaling
        //  figure out fish eye projection issue

        // TODO SPLIT INTO QUADRANTS

        //       | /
        //  ______/___
        //      /|
        //    /  |
        //  x    |
        // 0 .. 64 .. 128

        // apply symmetry across slope > 1 and slope < 1

        // TODO!! MAKE THIS MORE UNIT TESTABLE
        // TODO!! Find some way to edit the tileset easier

        // Cast Ray
        'raycasting: for y in 0..SCREEN_SIZE_Y as usize {
            //TODO shoot ray using curr_dir
            let ray: Ray2D = Ray2D::new(curr_dir, p.pos.y);

            if p.dir.x.is_sign_positive() {
                if p.dir.y.is_sign_positive() {

                    //?Refactor and use p.dir.x.sign_positive directly???
                    //??? hoist
                    let a_x = find_next_cell_boundary(p.pos.x, true);
                    let b_y = find_next_cell_boundary(p.pos.y, true);

                    let mut x_axis_inter: Vec2<f32> = Vec2::<f32> {
                        x: a_x as f32,
                        y: ray.at(a_x as f32),
                    };
                    let mut y_axis_inter: Vec2<f32> = Vec2::<f32> {
                        x: (b_y as f32 - ray.get_y_intercept()) / ray.get_slope(),
                        y: b_y as f32,
                    };
                    //debug_print_player()
                    //println!("INTIAL X:{} Y:{}", y_axis_inter.x, y_axis_inter.y);

                    let xstep = (1.0 / ray.dir.x).abs() * WORLD_CELL_SIZE as f32;
                    let ystep = (1.0 / ray.dir.y).abs() * WORLD_CELL_SIZE as f32;


                    let draw_color = GREEN;
                    let draw_X = y;

                    for draw_col in 0..SCREEN_SIZE_X as u8 {
                        let offset = draw_X * pitch + draw_col as usize * 3;
                        buffer[offset] = draw_color.r as u8;
                        buffer[offset + 1] = draw_color.g as u8;
                        buffer[offset + 2] = draw_color.b as u8;
                    }

                    'finding_wall: loop {
                        let mut x_dir_oob: bool = out_of_world_bounds(x_axis_inter);
                        let mut y_dir_oob: bool = out_of_world_bounds(y_axis_inter);

                        println!("Seeable in X dir {} Y dir {}", x_dir_oob, y_dir_oob);

                        let dist_x = x_axis_inter.dist(&p.pos);
                        let dist_y = y_axis_inter.dist(&p.pos);
                        //fix yintercept invalidation on update
                        if !x_dir_oob && !y_dir_oob {
                            if dist_x < dist_y && !y_dir_oob{
                                //check x dir first then y
                                let potential_wall = get_wall_at_vec2_pos(x_axis_inter, &w);
                                if potential_wall.full {
                                    //Draw collumn TODO HEIGHT SCALING
                                    println!("WALL FOUND!!");

                                    break 'finding_wall;
                                } else {
                                    //Advance to next x inter
                                    x_axis_inter = Vec2 {
                                        x: x_axis_inter.x + WORLD_CELL_SIZE as f32,
                                        y: x_axis_inter.y + ystep,
                                    };
                                    // Next intersection is out of bounds ?Switch
                                    if out_of_world_bounds(x_axis_inter) {
                                        x_dir_oob = true;
                                        if y_dir_oob {
                                            continue 'raycasting;
                                        }
                                        println!("Keep going x");
                                        break 'finding_wall;
                                    }
                                }
                            } else if !x_dir_oob {
                                //println!("X:{} Y:{}", y_axis_inter.x, y_axis_inter.y);
                                println!("WALL FOUND!!");

                                let potential_wall = get_wall_at_vec2_pos(y_axis_inter, &w);
                                if potential_wall.full {
                                    let draw_color = potential_wall.color;
                                    //draw_col

                                    break 'finding_wall;
                                } else {
                                    //Advance to next y inter
                                    y_axis_inter = Vec2 {
                                        x: y_axis_inter.x + xstep,
                                        y: x_axis_inter.y + WORLD_CELL_SIZE as f32,
                                    };

                                    if out_of_world_bounds(y_axis_inter) {
                                        y_dir_oob = true;
                                        if x_dir_oob {
                                            continue 'raycasting;
                                        }
                                        println!("Keep going Y");
                                        break 'finding_wall;
                                    }
                                }
                            }
                        }else {
                            break 'finding_wall;
                        }

                    }

                    //bounce between x axis steps and y axis steps
                    // process intersections in order of distance


                }
            }

            //let curr_w :Wall = world[y][i];


            curr_dir = rotate_clockwise(curr_dir, delta_theta_y);
        }
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
        //Draw texture
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

//TODO Add docstrings to things


//TODO ADD Debug derives



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

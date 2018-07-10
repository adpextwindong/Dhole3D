extern crate sdl2;
extern crate sdl2_window;
extern crate shader_version;
extern crate window;
extern crate num_traits;

//extern crate serde_derive;

//extern crate serde;
//extern crate serde_json;
// TODO Map serialization

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
use world::wall::BLUE as BLUE;

use world::player::Player as Player;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;



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

const FOV: f32 = f32::consts::FRAC_PI_2;
const MOVE_RATE : f32 = 5.0;
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

fn debug_draw_world(canvas: &mut Canvas<Window>, w : &Vec<Vec<Wall>>) {
    canvas.set_draw_color(Color{
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    });
    //TODO maybe need scaling/moving around for bigger maps but we can refactor that later.
    let rect_size = SCREEN_SIZE_Y / WORLD_SIZE_Y as u32;
    let mut recs = Vec::<sdl2::rect::Rect>::with_capacity(WORLD_SIZE_X * WORLD_SIZE_Y as usize);
    for x in 0..WORLD_SIZE_X as i32{
        for y in 0..WORLD_SIZE_Y as i32{
            let frame = sdl2::rect::Rect::new(x * rect_size as i32, y * rect_size as i32, rect_size, rect_size);

            recs.push(frame);
        }
    }

    canvas.draw_rects(&recs);
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
    if pos.x >= 0.0 && pos.y >= 0.0{
        if pos.x == std::f32::INFINITY || pos.y == std::f32::INFINITY {
            return true;
        }else{
            return (pos.x >= (WORLD_SIZE_X as f32 * WORLD_CELL_SIZE as f32)) ||
                (pos.y >= (WORLD_SIZE_Y as f32 * WORLD_CELL_SIZE as f32))
        }
    }else{
        return true;
    }

}

fn get_world_cell_at_vec2_pos(pos: Vec2<f32>, w: &Vec<Vec<Wall>>) -> Wall {
    //println!("GET_WORLD POS {:?}",pos);
    let x: usize = (pos.x.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;
    let y: usize = (pos.y.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;
    w[x][y]
}

fn debug_print_world(w: &Vec<Vec<Wall>>, p: &Player) {
    let x: usize = (p.pos.x.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;
    let y: usize = (p.pos.y.floor() as i32 / WORLD_CELL_SIZE as i32) as usize;

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

fn draw_col(buffer: &mut [u8], pitch: usize, x: usize, color: Color, dist: f32) {
    //println!("SCALING BY DIST {:?}", dist);
    //let h = SCREEN_SIZE_Y as f32 * dist; //This dist will have to be normalized for fix eye
    //let col_start = h /2.0;
    //let col_end = SCREEN_SIZE_Y as f32 - (h / 2.0);

    for y in 0  .. SCREEN_SIZE_Y as usize {
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

fn advance_x_intersection(x_axis_intersection : &Vec2<f32>, y_step : f32) -> Vec2<f32> {
    Vec2 {
        x: x_axis_intersection.x + WORLD_CELL_SIZE as f32,
        y: x_axis_intersection.y + y_step,
    }
}

//TODO clean this up
fn advance_y_intersection(y_axis_intersection : &Vec2<f32>, x_step : f32) -> Vec2<f32> {
    Vec2 {
        x: y_axis_intersection.x + x_step,
        y: y_axis_intersection.y + WORLD_CELL_SIZE as f32,
    }
}

fn gen_dda_steps(p : &Player, ray : &Ray2D) -> (f32,f32) {
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

    (xstep, ystep)
}

fn x_walk(theworld : &Vec<Vec<Wall>>, xstep : f32, dist_x_inter : Vec2<f32>, x_axis_intersection: &mut Vec2<f32>, ystep: f32) -> Option<(Wall,Vec2<f32>)>{
    //println!("X walk, xstep:{:?} x_dist:{:?}", xstep, dist_x_inter);
    let potential_wall: Wall = get_world_cell_at_vec2_pos(x_axis_intersection.to_owned(), &theworld);
    if potential_wall.full {
        //println!("WALL FOUND!!");
        return Some((potential_wall, dist_x_inter))
    } else {
        *x_axis_intersection = advance_x_intersection(x_axis_intersection, ystep);
        return None;
    }
}
fn y_walk(theworld : &Vec<Vec<Wall>>, ystep : f32, dist_y_inter : Vec2<f32>, y_axis_intersection: &mut Vec2<f32>, xstep: f32) -> Option<(Wall,Vec2<f32>)>{
    //println!("Y walk, ystep:{:?} y_dist:{:?}", ystep, dist_y_inter);
    let potential_wall : Wall = get_world_cell_at_vec2_pos(y_axis_intersection.to_owned(), theworld);
    if potential_wall.full {
        //println!("WALL FOUND!!");
        return Some((potential_wall, dist_y_inter));
    }else{
        //TODO clean this up
        *y_axis_intersection = advance_y_intersection(y_axis_intersection, xstep);
        return None;
    }
}

//TODO CURRENT, HEIGHT SCALING
///DDA Algorithm using initial x and y axis intersections
fn
find_wall_and_distance(theworld: &Vec<Vec<Wall>>, p : &Player, ray : Ray2D) -> Option<(Wall, Vec2<f32>)> {
    let (xstep, ystep) = gen_dda_steps(p,&ray);
    let (mut x_axis_intersection,mut y_axis_intersection) = find_axis_intersections(p.pos, ray);

//    debug_print_world(theworld, p.pos);
//    println!("RAY: {:?}",ray);
//    println!("STEPS X: {:?}, Y: {:?}", xstep, ystep);
//    println!("\nSTART STEPS {:?} {:?}", xstep, ystep); //TODO FIX STEP == INF CASE
    //println!("PLAYER POS: {:?} RAY DIRECTION: {:?}", p.pos, ray.dir);

    'finding_wall: loop {
        let x_dir_oob: bool = out_of_world_bounds(x_axis_intersection);
        let y_dir_oob: bool = out_of_world_bounds(y_axis_intersection);

        let dist_x_inter = x_axis_intersection.diff(&p.pos);
        let dist_y_inter = y_axis_intersection.diff(&p.pos);

        //let dist_x_inter = x_axis_intersection.dist(&p.pos);
        //let dist_y_inter = y_axis_intersection.dist(&p.pos);

        let walk_res;

        if x_dir_oob && y_dir_oob { //The ray has hit out of bounds
            //println!("Done, returning None");
            return None;
        }else if x_dir_oob {
            //println!("FIRST PATH WALK");
            walk_res = y_walk(theworld, ystep, dist_y_inter, &mut y_axis_intersection, xstep);
        }else if y_dir_oob {
            //println!("SECOND PATH WALK");
            walk_res = x_walk(theworld, xstep, dist_x_inter, &mut x_axis_intersection, ystep);
        }else{
            if dist_x_inter.length() <= dist_y_inter.length()  {
                //println!("THIRD PATH WALK");
                walk_res = x_walk(theworld, xstep, dist_x_inter, &mut x_axis_intersection, ystep);
            }else{
                walk_res = y_walk(theworld, ystep, dist_y_inter, &mut y_axis_intersection, xstep);
            }
        }

        if let Some((wall, dist)) = walk_res {
            //println!("THE WALL IS FOUND");
            return Some((wall,dist));
        }

//        println!("Grab stdin");
//        let mut stdin = io::stdin();
//        let _ = stdin.read(&mut [0u8]).unwrap();
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
    let mut debug_canvas = debug_window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_SIZE_X, SCREEN_SIZE_Y)
        .unwrap();

    let mut theworld = gen_blank_world(WORLD_SIZE_X, WORLD_SIZE_Y);

    let mut p: Player = Player {
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
//
        theworld[i][0] = blue_wall;
        theworld[i][WORLD_SIZE_X as usize - 1 as usize] = green_wall;
    }
    //NOW Test up down and left right

//    theworld[5][2] = Wall {
//        full: true,
//        color: BLUE,
//    };
//    theworld[5][4] = Wall {
//        full: true,
//        color: GREEN,
//    };


    debug_print_world(&theworld, &p);


    //This function renders the raycast pixels to a pixel buffer. To be used with sdl_texture

    debug_print_player(p);
//    let mut stdin = io::stdin();
//    let _ = stdin.read(&mut [0u8]).unwrap();

    let mut delta: f64 = 0.0;

    'running: loop {
        canvas.clear();
        p.dir.normalize();
        let last_frame_instant = Instant::now();

        let mut event_pump = sdl_context.event_pump().unwrap();
        draw_ceiling(&mut canvas);
        draw_floor(&mut canvas);

        debug_draw_world(&mut debug_canvas, &theworld);

        //TODO Finish the statics renderer
        //Draw statics texture
        {
            let render_statics = |buffer: &mut [u8], pitch: usize| {
                let world: &Vec<Vec<Wall>> = &theworld;
                let p_copy: Player = p;


                let mut ray_curr_dir = rotate_counter_clockwise(p_copy.dir, FOV / 2.0);

                let delta_theta_y = FOV / SCREEN_SIZE_X as f32;

                'raycasting: for mut y in 0..SCREEN_SIZE_X as usize {
                    let ray: Ray2D = Ray2D::new(ray_curr_dir, p_copy.pos.y);
                    let possible_wall : Option<(Wall, Vec2<f32>)> = find_wall_and_distance(world, &p_copy, ray);

                    if let Some((sampled_wall, dist)) = possible_wall {

                        let ang = ray.dir.norm().angle();
                        let fixed_dist = (dist.x * f32::cos(ang)) + (dist.y * f32::sin(ang));

                        draw_col(buffer, pitch, y, sampled_wall.color, fixed_dist);

                    }
                    ray_curr_dir = rotate_clockwise(ray_curr_dir, delta_theta_y);
                }
            };
            texture.with_lock(None, &render_statics).unwrap();
        }
        canvas.copy(&texture, None, None).unwrap();


        //Present Frame
        canvas.present();
        debug_canvas.present();

//        let frame_duration = last_frame_instant.elapsed();
//        delta = frame_duration.as_secs() as f64 + frame_duration.subsec_nanos() as f64 * 1e-9;
//        println!("{}", delta);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |

                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                Event::KeyDown {keycode : Some(Keycode::W) ,.. } =>{
                    if p.pos.x < (WORLD_SIZE_X as u32 * WORLD_CELL_SIZE- 1) as f32 {
                        p.pos.x += 1.0 * MOVE_RATE;
                    }
                    //println!("Move right");
                    debug_print_world(&theworld, &p);
                },
                Event::KeyDown {keycode : Some(Keycode::S) ,.. } =>{
                    if p.pos.x > (1) as f32 {
                        p.pos.x -= 1.0 * MOVE_RATE;
                    }
                    //println!("Move left");
                    debug_print_world(&theworld, &p);
                },
                Event::KeyDown {keycode : Some(Keycode::A) ,.. } =>{
//                    if p.pos.y < (WORLD_SIZE_Y as u32 * WORLD_CELL_SIZE - 1) as f32 {
//                        p.pos.y += 1.0;
//                    }
                    //println!("Move up");
                    p.dir = rotate_counter_clockwise(p.dir, FOV / 15.0);

                    debug_print_world(&theworld, &p);
                },
                Event::KeyDown {keycode : Some(Keycode::D) ,.. } =>{
//                    if p.pos.y > (1) as f32 {
//                        p.pos.y -= 1.0;
//                    }
                    p.dir = rotate_clockwise(p.dir, FOV / 15.0);
                    //println!("Move down");
                    debug_print_world(&theworld, &p);
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        // TODO Work on gameloop once renderer is up
        // The rest of the game loop goes here...
    }

//    println!("Frame Delta : {}", delta);
}

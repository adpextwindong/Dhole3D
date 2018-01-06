extern crate sdl2;
extern crate sdl2_window;
extern crate shader_version;
extern crate window;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

//use sdl2_window::Sdl2Window;
use sdl2::surface;
use sdl2::video::Window;
use sdl2::render::Canvas;
//use sdl2::rect;

//use window::WindowSettings;

//TODO_FAR opengl?
//use shader_version::OpenGL;

use std::f32;
use std::time::{Duration, Instant};



//Types TODO
//2D Vector
//Player obj
//Map type
//World map var
//Wall Type

//?Far compression of world to file
//See how much this struct takes up in mem and alignment etc...
#[derive(Copy, Clone)]
pub struct  Wall{
    full : bool,
    color : Color
}
//????
pub struct MapGrip {
    walls : [[Wall; 2]; 2]
}

#[derive(Copy, Clone)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
struct Player {
    pos : Vec2, //Their position in the world
    dir : Vec2  //Direction their facing
}

const NULL_COLOR : Color = Color{r:0,g:0,b:0,a: 0};
const NULL_WALL : Wall = Wall{full: false,  color: NULL_COLOR};

const RED : Color = Color{r: 255, g:0 ,b: 0, a: 255};
const GREEN : Color = Color{r: 0, g:255 ,b: 0, a: 255};
const BLUE : Color = Color{r: 0, g:0 ,b: 255, a: 255};

const FLOOR_GREY : Color = Color{r : 128, g: 128, b: 128, a: 255};
const CEILING_BLACK : Color = Color{r : 0, g: 0, b: 0, a: 255};

//WORLD CONSTANTS
//TODO make this flexible for loading seperate worlds
const WORLD_SIZE_X : u32 = 10;
const WORLD_SIZE_Y : u32 = 10;

const WORLD_CELL_SIZE : u32 = 10; // 10 Meters?

//TODO make this a startup option or something
const SCREEN_SIZE_X : u32 = 800;
const SCREEN_SIZE_Y : u32 = 600;

//TODO finish this
//TODO Add filepath option or make an INTO/FROM for a load source
fn gen_blank_world(x: u32, y:u32) -> Vec<Vec<Wall>> {
    let mut ret = Vec::new();
    let reg_wall = Wall {full:true, color: NULL_COLOR};
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
fn draw_ceiling(canvas : &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    //FIX ME This expects to be ran first
    canvas.clear();
}

fn draw_floor(canvas : &mut Canvas<Window>) {
    canvas.set_draw_color(FLOOR_GREY);
    let pos_y =  SCREEN_SIZE_Y as i32 /2;
    let size_y = SCREEN_SIZE_Y as i32 /2;
    canvas.fill_rect(Rect::new(0, pos_y, SCREEN_SIZE_X, size_y as u32)).unwrap();
}

fn debug_print_player(p: Player){
    println!("POS: {} {} DIR: {} {}", p.pos.x, p.pos.y, p.dir.x, p.dir.y);
}
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo: Video", SCREEN_SIZE_X, SCREEN_SIZE_Y)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, SCREEN_SIZE_X, SCREEN_SIZE_Y).unwrap();

    let mut w = gen_blank_world(WORLD_SIZE_X, WORLD_SIZE_Y);

    w[5][5] = Wall{full: true, color: RED};

    let p : Player = Player{pos: Vec2{x : 0.0 ,
                                      y : (WORLD_SIZE_Y * WORLD_CELL_SIZE) as f32 },
                            dir: Vec2{x : f32::consts::FRAC_PI_2,
                                      y : f32::consts::FRAC_PI_2} };

    //TODO_FAR Move this to a seperate renderer file that takes the world ref
    //This function renders the raycast pixels to a pixel buffer. To be used with sdl_texture
    //TODO FINISH IT
    let render_statics = |buffer: &mut [u8], pitch: usize| {
        let world : &Vec<Vec<Wall>> = &w;
        //println!("{}", pitch);
        for y in 0..SCREEN_SIZE_Y as usize {
            for x in 0..SCREEN_SIZE_X as usize {
                let offset = y*pitch + x*3;
                buffer[offset] = 255 as u8;
                buffer[offset + 1] = 0 as u8;
                buffer[offset + 2] = 0;
            }
        }
    };


    let mut delta : f64 = 0.0;
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
        delta = frame_duration.as_secs() as f64
                            + frame_duration.subsec_nanos() as f64 * 1e-9;

        //println!("{}", delta);


        //debug_print_player(p);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
        // TODO Work on gameloop once renderer is up
    }


    println!("{}", delta);
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


#![allow(unused_imports)]
#![allow(unused_mut)]

extern crate sdl2;
extern crate sdl2_window;
extern crate shader_version;
extern crate window;
extern crate num_traits;

use std::time::{Duration};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod world;
mod renderer;
mod debug_window;

use world::test_util::generate_test_world;
use world::WORLD_SIZE_X;
use world::WORLD_CELL_SIZE;
use world::player::MOVE_RATE;
use renderer::FOV;
use renderer::vector::rotate_clockwise;
use renderer::vector::rotate_counter_clockwise;

//TODO make this a startup option or something
const SCREEN_SIZE_X: u32 = 800;
const SCREEN_SIZE_Y: u32 = 600;

/*
pub fn event_pump_handler(){
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |

            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

            Event::KeyDown {keycode : Some(Keycode::W) ,.. } =>{
                if p.pos.x < (WORLD_SIZE_X as u32 * WORLD_CELL_SIZE - 1) as f32 {
                    p.pos.x += 1.0 * MOVE_RATE;
                }
                //println!("Move right");
                debug_window::debug_print_world(&theworld, &p);
            },
            Event::KeyDown {keycode : Some(Keycode::S) ,.. } =>{
                if p.pos.x > (1) as f32 {
                    p.pos.x -= 1.0 * MOVE_RATE;
                }
                //println!("Move left");
                debug_window::debug_print_world(&theworld, &p);
            },
            Event::KeyDown {keycode : Some(Keycode::A) ,.. } =>{
//                    if p.pos.y < (WORLD_SIZE_Y as u32 * WORLD_CELL_SIZE - 1) as f32 {
//                        p.pos.y += 1.0;
//                    }
                //println!("Move up");
                p.dir = rotate_counter_clockwise(p.dir, FOV / 15.0);

                debug_window::debug_print_world(&theworld, &p);
            },
            Event::KeyDown {keycode : Some(Keycode::D) ,.. } =>{
//                    if p.pos.y > (1) as f32 {
//                        p.pos.y -= 1.0;
//                    }
                p.dir = rotate_clockwise(p.dir, FOV / 15.0);
                //println!("Move down");
                debug_window::debug_print_world(&theworld, &p);
            },
            _ => {}
        }
    }
}
*/

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
    let texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_SIZE_X, SCREEN_SIZE_Y)
        .unwrap();

    let (theworld,mut p) = generate_test_world();

    let mut debug_canvas = debug_window.into_canvas().build().unwrap();
    debug_window::debug_print_world(&theworld, &p);
    debug_window::debug_print_player(p);

    'running: loop {
        //let mut event_pump = sdl_context.event_pump().unwrap();
        p.dir.normalize();
        debug_window::debug_draw_world(&mut debug_canvas, &theworld, &p);

        //========
        //RENDERER
        {
            let mut game_renderer = renderer::renderer::new(&mut canvas, &mut texture);
            game_renderer.draw_frame(&theworld, &p);
        }
        //RENDERER
        //========

        debug_canvas.present();


        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        // TODO Work on gameloop once renderer is up
        // The rest of the game loop goes here...
    }
}

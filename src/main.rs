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


mod world;
mod renderer;
mod debug_window;
mod keyhandler;

use world::test_util::generate_test_world;
use world::WORLD_SIZE_X;
use world::WORLD_CELL_SIZE;
use world::player::MOVE_RATE;
use renderer::FOV;
use renderer::vector::rotate_clockwise;
use renderer::vector::rotate_counter_clockwise;
use keyhandler::handle_events;

//TODO make this a startup option or something
const SCREEN_SIZE_X: u32 = 800;
const SCREEN_SIZE_Y: u32 = 600;

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

    let (theworld,mut p) = generate_test_world();

    let mut debug_canvas = debug_window.into_canvas().build().unwrap();
//    debug_window::debug_print_world(&theworld, &p);
    debug_window::debug_print_player(p);

    'running: loop {
        let mut event_pump = sdl_context.event_pump().unwrap();
        p.dir.normalize();
        debug_window::debug_draw_world(&mut debug_canvas, &theworld, &p);
        debug_window::debug_print_player(p);
        //========
        //RENDERER
        {
            let mut game_renderer = renderer::renderer::new(&mut canvas);
            game_renderer.draw_frame(&mut texture, &theworld, &p);
        }
        //RENDERER
        //========

        if let Some(bad_event) = handle_events(event_pump, &theworld, &mut p) {
            print!("Recieved event: {:?}. Shutting down.", bad_event);
            break 'running;
        }

        debug_canvas.present();




        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        // TODO Work on gameloop once renderer is up
        // The rest of the game loop goes here...
    }
}

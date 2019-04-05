#![allow(unused_imports)]
#![allow(unused_mut)]

extern crate sdl2;
extern crate sdl2_window;
extern crate shader_version;
extern crate window;
extern crate num_traits;
extern crate graphics;
extern crate serde;
extern crate serde_json;

use std::time::{Duration};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

mod world;
mod renderer;
mod debug_window;
mod keyhandler;

use world::test_util::generate_test_world;
use world::WORLD_SIZE_X;
use world::WORLD_CELL_SIZE;
use world::player::MOVE_RATE;
use world::GameState;
use renderer::FOV;
use renderer::vector::rotate_clockwise;
use renderer::vector::rotate_counter_clockwise;
use keyhandler::handle_events;

use keyhandler::KeyhandlerEvent::*;
use world::player::Player;

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

    let mut gs : GameState = generate_test_world();

    let mut debug_canvas = debug_window.into_canvas().build().unwrap();
//    debug_window::debug_print_world(&theworld, &p);
//    debug_window::debug_print_player(gs.p);

    let mut key_update = true;
    let mut debug_on : bool = false;

    //FIXME add saving player state to file to make debugging easier
    {
        let player = match OpenOptions::new().read(true).open("save.json"){
            Err(e) => {
                gs.p
            },
            Ok(file) => {
                let save_reader = BufReader::new(file);
                serde_json::from_reader(save_reader).unwrap_or(gs.p)
            }
        };
        gs.p = player;
    }


    'running: loop {
        let mut event_pump = sdl_context.event_pump().unwrap();


        //========
        //RENDERER
        {
            let mut game_renderer = renderer::renderer::new(&mut canvas);
            //fixme mutability of GS
            let last_frame_info = game_renderer.draw_frame(&mut debug_canvas, &mut texture,  &mut gs, key_update && debug_on);
        }
        //RENDERER
        //========

        if gs.dflags.distsView == false {
            debug_window::debug_draw_world(&mut debug_canvas, &gs);
        }

        if gs.dflags.inspect_ray.is_some(){
            gs.dflags.inspect_ray = None;
            //gs.dflags.inspect_ray_info = None;
            debug_on = false;
        }

        if let Some(event) = handle_events(event_pump, &mut gs, &mut debug_on) {
            match event {
                keyhandler::KeyhandlerEvent::EngineKeyKill => {
                    print!("\nReceived event: {:?}. Saving player state to disk. Shutting down.", event);
                    {
                        let mut save_file = OpenOptions::new().create(true).write(true).truncate(true).open("save.json").unwrap();
                        //Fixme unwrap
                        save_file.write(serde_json::to_string(&gs.p).unwrap().as_bytes());
                    }

                    break 'running;

                },
                EngineKeyGSUpdate => {
                    gs.dflags.inspect_ray_info = None;
                    key_update = true;
                    //debug_window::debug_print_player(p);
                },
                EngineKeyIdle => {key_update = false;},
            }

        }

        debug_canvas.present();




        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        // TODO Work on gameloop once renderer is up
        // The rest of the game loop goes here...
    }
}

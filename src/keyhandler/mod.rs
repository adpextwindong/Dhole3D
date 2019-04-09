
use std::time::SystemTime;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

use world::WORLD_CELL_SIZE;
use world::WORLD_SIZE_X;
use world::WORLD_SIZE_Y;
use world::player::Player;
use world::wall::Wall;

use renderer::FOV;
use renderer::vector::{rotate_counter_clockwise, rotate_clockwise};

use std::io;
use std::io::BufRead;

use world::player::MOVE_RATE;
use world::GameState;
use world::DebugWindowFlags;

const TURN_RESOLUTION : f32 = 30.0;

#[derive(Debug)]
pub enum KeyhandlerEvent {
    EngineKeyIdle,
    EngineKeyGSUpdate,
    EngineKeyKill
}

pub fn handle_events(mut event_pump :  EventPump,gs : &mut GameState, debug_on : &mut bool, dflags : &mut DebugWindowFlags) -> Option<KeyhandlerEvent> {

    //filter events for kill commands, then process gs_update keys
    //TODO look into how this pump iter really works
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return Some(KeyhandlerEvent::EngineKeyKill),

            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Some(KeyhandlerEvent::EngineKeyKill),

            Event::KeyDown { keycode, .. } => {
                handle_keydowns(keycode, gs, debug_on, dflags);
                return Some(KeyhandlerEvent::EngineKeyGSUpdate);
            },

            _ => return Some(KeyhandlerEvent::EngineKeyIdle)
        }
    }
    None
}



fn handle_keydowns(keydown : Option<Keycode>,gs : &mut GameState, debug_on : &mut bool, dflags : &mut DebugWindowFlags) {
    if let Some(keycode) = keydown{
        match keycode{
            Keycode::W => {
                let pos_delta = gs.p.dir.scale(MOVE_RATE);
                gs.move_player(pos_delta);
            },
            Keycode::S =>{
                let pos_delta = gs.p.dir.scale( -1.0 * MOVE_RATE);
                gs.move_player( pos_delta);
            },
            Keycode::A =>{
                gs.p.dir = rotate_counter_clockwise(gs.p.dir, FOV / TURN_RESOLUTION);
                gs.camera_plane = rotate_counter_clockwise(gs.camera_plane, FOV / TURN_RESOLUTION);
            },
            Keycode::D =>{

                gs.p.dir = rotate_clockwise(gs.p.dir, FOV / TURN_RESOLUTION);
                gs.camera_plane = rotate_clockwise(gs.camera_plane, FOV / TURN_RESOLUTION);
            },
            Keycode::O =>{
                *debug_on^= true;
            },
            Keycode::L =>{
                dflags.distsView ^= true;
            }
            Keycode::I =>{
                println!("Enter ray to inspect: ");
                {
                    let mut buffer = String::new();
                    let stdin = io::stdin();
                    let mut handle = stdin.lock();

                    match handle.read_line(&mut buffer){
                        Ok(_) => {
                            match usize::from_str_radix(&buffer.trim(), 10){
                                Ok(inspect_ray_i) => {
                                    println!("Inspecting {:?}", inspect_ray_i);
                                    dflags.inspect_ray = Some(inspect_ray_i);
                                },
                                e =>{
                                    dflags.inspect_ray = None;
                                }
                            }
                        },
                        e =>{
                        }
                    }
                }
                println!("Ok uhh");
            }
            _ => {
                //Unused key for now
            }
        }
    }
}
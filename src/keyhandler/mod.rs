
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

use world::player::MOVE_RATE;
use world::GameState;

const TURN_RESOLUTION : f32 = 30.0;

#[derive(Debug)]
pub enum KeyhandlerEvent {
    EngineKeyIdle,
    EngineKeyGSUpdate,
    EngineKeyKill
}

pub fn handle_events(mut event_pump :  EventPump,gs : &mut GameState, debug_on : &mut bool) -> Option<KeyhandlerEvent> {

    //filter events for kill commands, then process gs_update keys
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return Some(KeyhandlerEvent::EngineKeyKill),

            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Some(KeyhandlerEvent::EngineKeyKill),

            Event::KeyDown { keycode, .. } => {
                handle_keydowns(keycode, gs, debug_on);
                return Some(KeyhandlerEvent::EngineKeyGSUpdate);
            },

            _ => return Some(KeyhandlerEvent::EngineKeyIdle)
        }
    }
    None
}



fn handle_keydowns(keydown : Option<Keycode>,gs : &mut GameState, debug_on : &mut bool) {
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

            },
            Keycode::D =>{

                gs.p.dir = rotate_clockwise(gs.p.dir, FOV / TURN_RESOLUTION);
            },
            Keycode::O =>{
                *debug_on^= true;
            },
            _ => {
                //Unused key for now
            }
        }
    }
}
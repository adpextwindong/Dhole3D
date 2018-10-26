
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
const TURN_RESOLUTION : f32 = 30.0;

pub fn handle_events(mut event_pump :  EventPump,w: &Vec<Vec<Wall>>, p : &mut Player) -> Option<Event> {

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {},

            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Some(Event::Quit{
                timestamp: 0,
            }),

            Event::KeyDown {keycode, ..} => {
                handle_keydowns(keycode, w, p);
            },

            _ => {}
        }
    }
    None
}



fn handle_keydowns(keydown : Option<Keycode>, w: &Vec<Vec<Wall>>, p : &mut Player) {
    if let Some(keycode) = keydown{
        match keycode{
            Keycode::W => {
                let pos_delta = p.dir.scale(MOVE_RATE);
                p.move_player(w, pos_delta);
            },
            Keycode::S =>{
                let pos_delta = p.dir.scale( -1.0 * MOVE_RATE);
                p.move_player(w, pos_delta);
            },
            Keycode::A =>{
                p.dir = rotate_counter_clockwise(p.dir, FOV / TURN_RESOLUTION);

            },
            Keycode::D =>{
                p.dir = rotate_clockwise(p.dir, FOV / TURN_RESOLUTION);
            },
            _ => {
                //Unused key for now
            }
        }
    }
}
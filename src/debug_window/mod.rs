use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::f32;

use world::wall::Wall;
use world::player::Player;
use SCREEN_SIZE_Y;
use world::WORLD_SIZE_Y;
use world::WORLD_SIZE_X;
use world::WORLD_CELL_SIZE;
use world::GameState;
use renderer::vector::Vec2;
use renderer::vector::{rotate_clockwise, rotate_counter_clockwise};

const RECT_SIZE: u32 = SCREEN_SIZE_Y / WORLD_SIZE_Y as u32;

pub fn debug_draw_dists(canvas: &mut Canvas<Window>, dists_and_colors : &Vec<(f32, Color)>) {
    let max : f32 = dists_and_colors.into_iter().map(|(d,c) | d).fold(0.0f32, |mut max, &val| {
        if val > max {
            max = val;
        }
        max
    });
    canvas.set_draw_color(Color{
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    });
    canvas.fill_rect(None).unwrap(); //Reset canvas to gray

    for (x, (dist,color)) in dists_and_colors.iter().enumerate(){
        canvas.set_draw_color(*color);

        canvas.fill_rect(
            Rect::new(
                x as i32,
                0,
                1,
                ((dist/max) * SCREEN_SIZE_Y as f32) as u32
            )
        ).unwrap();

    }
}

//TODO maybe need scaling/moving around for bigger maps but we can refactor that later.
pub fn debug_draw_world(canvas: &mut Canvas<Window>, gs : &GameState) {
        if gs.dflags.distsView == false{
            canvas.set_draw_color(Color{
                r: 128,
                g: 128,
                b: 128,
                a: 255,
            });
            canvas.fill_rect(None).unwrap(); //Reset canvas to gray

            draw_cells(canvas, &gs.the_world);

            canvas.set_draw_color(Color{
                r: 255,
                g: 165,
                b: 0,
                a: 255,
            });

            draw_player(canvas, &gs.p);
        }
}

fn draw_player(canvas: &mut Canvas<Window>, p: &Player){

    let pos_debug = wc2screen_coords(p.pos);

    canvas.set_draw_color(Color{
        r: 255,
        g: 165,
        b: 0,
        a: 255,
    });
    let p_draw_radius = 6;
    canvas.fill_rect(
        Rect::new(
            pos_debug.x as i32 - p_draw_radius/2,
            pos_debug.y as i32 - p_draw_radius/2,
            p_draw_radius as u32,
            p_draw_radius as u32
        )
    ).unwrap();

    let line_scale = 1000.0;

    let arrow_head_pos = Vec2{
        x: pos_debug.x + p.dir.normalized().scale(line_scale).x,
        y: pos_debug.y - p.dir.normalized().scale(line_scale).y
    };

    canvas.set_draw_color(Color{
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    });

    canvas.draw_line(
        Point::new(pos_debug.x as i32, pos_debug.y as i32),
        Point::new(arrow_head_pos.x as i32, arrow_head_pos.y as i32),
    ).unwrap();
}
fn draw_cells(canvas: &mut Canvas<Window>, w : &Vec<Vec<Wall>>) {
    for x in 0..WORLD_SIZE_X as i32{
        for y in 0..WORLD_SIZE_Y as i32{
            let y_pos = SCREEN_SIZE_Y as i32 - ((y + 1) * RECT_SIZE as i32);
            let x_pos = x * RECT_SIZE as i32;
            let grid_content = Rect::new(x_pos, y_pos, RECT_SIZE - 1, RECT_SIZE - 1);

            canvas.set_draw_color(w[x as usize][y  as usize].color);
            canvas.fill_rect(grid_content).unwrap();
        }
    }
}

fn wc2screen_coords(pos :Vec2<f32>) -> Vec2<f32>{
    let scale = RECT_SIZE as f32 / WORLD_CELL_SIZE as f32;
    Vec2{
        x: pos.x * scale,
        y: SCREEN_SIZE_Y as f32 - (pos.y * scale)
    }
}

pub fn debug_print_player(p: Player) {
    println!("POS: {} {} DIR: {} {}", p.pos.x, p.pos.y, p.dir.x, p.dir.y);
}

pub fn debug_print_world(w: &Vec<Vec<Wall>>, p: &Player) {
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
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use world::wall::Wall;
use world::player::Player;
use SCREEN_SIZE_Y;
use world::WORLD_SIZE_Y;
use world::WORLD_SIZE_X;
use world::WORLD_CELL_SIZE;


//TODO maybe need scaling/moving around for bigger maps but we can refactor that later.
pub fn debug_draw_world(canvas: &mut Canvas<Window>, w : &Vec<Vec<Wall>>, p: &Player) {
    canvas.set_draw_color(Color{
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    });
    canvas.fill_rect(None).unwrap();

    let rect_size = SCREEN_SIZE_Y / WORLD_SIZE_Y as u32;
    let mut recs = Vec::<Rect>::with_capacity(WORLD_SIZE_X * WORLD_SIZE_Y as usize);
    for x in 0..WORLD_SIZE_X as i32{
        for y in 0..WORLD_SIZE_Y as i32{
            let frame = Rect::new(x * rect_size as i32, y * rect_size as i32, rect_size, rect_size);

            recs.push(frame);
        }
    }
    canvas.draw_rects(&recs).unwrap();

    for x in 0..WORLD_SIZE_X as i32{
        for y in 0..WORLD_SIZE_Y as i32{
            let grid_content = Rect::new((x * rect_size as i32) + 1 as i32, (y * rect_size as i32) + 1 as i32, rect_size - 2, rect_size - 2);
            canvas.set_draw_color(w[x as usize][y as usize].color);
            canvas.fill_rect(grid_content).unwrap();
        }
    }

    canvas.set_draw_color(Color{
        r: 255,
        g: 165,
        b: 0,
        a: 255,
    });
    canvas.fill_rect(
        Rect::new(
            ((p.pos.x / WORLD_CELL_SIZE as f32 ) as u32 * rect_size) as i32,
            ((p.pos.y / WORLD_CELL_SIZE as f32 ) as u32 * rect_size) as i32,
            rect_size / 3,
            rect_size / 3
        )
    ).unwrap();
//    canvas.circle(
//        ((p.pos.x / WORLD_SIZE_X as f32) as u32 * rect_size) as i16,
//        ((p.pos.y / WORLD_SIZE_Y as f32) as u32 * rect_size) as i16,
//        (rect_size / 2) as i16,
//        Color{
//            r: 255,
//            g: 165,
//            b: 0,
//            a: 255,
//        }
//    ).unwrap();

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
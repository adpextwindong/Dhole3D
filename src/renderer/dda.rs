use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::io::StdoutLock;

use renderer::vector::Vec2;
use renderer::ray2D::Ray2D;
use world::out_of_world_bounds;
use world::wall::Wall;
use world::player::Player;
use world::WORLD_CELL_SIZE;
use world::find_next_cell_boundary;
use world::GameState as GameState;

fn find_axis_intersections(position : Vec2<f32>, ray: Ray2D, xstep : f32, ystep : f32) -> (Vec2<f32>, Vec2<f32>){
    let a_x : i32 = if xstep == 0.0{
        position.x.round() as i32
    }else{
        find_next_cell_boundary(position.x, ray.dir.x.is_sign_positive())
    };

    let b_y: i32 = if ystep == 0.0{
        position.y.floor() as i32
    }else{
        find_next_cell_boundary(position.y, ray.dir.y.is_sign_positive())
    };

    let x_axis_inter: Vec2<f32> = Vec2::<f32> {
        x: a_x as f32,
        y: position.y + (((a_x as f32 - position.x)/ray.dir.x)*ray.dir.y)
    };
    let y_axis_inter: Vec2<f32> = Vec2::<f32> {
        x: position.x + (((b_y as f32 - position.y)/ray.dir.y)*ray.dir.x),
        y: b_y as f32,
    };

    (x_axis_inter, y_axis_inter)
}

fn advance_x_intersection(x_axis_intersection : &Vec2<f32>, x_step : f32, y_step : f32) -> Vec2<f32> {
    let advance = if x_step.is_sign_positive(){
        WORLD_CELL_SIZE as i32
    }else{
        -1 * WORLD_CELL_SIZE as i32
    };
    Vec2 {
        x: x_axis_intersection.x + advance as f32,
        y: x_axis_intersection.y + y_step,
    }
}

fn advance_y_intersection(y_axis_intersection : &Vec2<f32>, x_step : f32, y_step : f32) -> Vec2<f32> {
    let advance = if y_step.is_sign_positive(){
        WORLD_CELL_SIZE as i32
    }else{
        -1 * WORLD_CELL_SIZE as i32
    };
    Vec2 {
        x: y_axis_intersection.x + x_step,
        y: y_axis_intersection.y + advance as f32,
    }
}

fn gen_dda_steps(ray : Ray2D) -> (f32,f32) {
    // X + 1 , A + ystep

    let scaled_dir_steps = ray.dir.normalized().scale(WORLD_CELL_SIZE as f32);
    (scaled_dir_steps.x, scaled_dir_steps.y)
}

fn check_over_y_step(gs : &GameState, xstep : f32,ystep: f32, intersection : Vec2<f32>, changed_frame : bool) -> Option<(Wall, Vec2<f32>)>{
    let grab_cell_pos = if ystep.is_sign_negative(){

        Vec2{
            x: intersection.x ,
            //chuck it over to the next spot
            y: if intersection.y - 0.5> 0.0 {
                intersection.y - 0.5
            }else{
                0.0
            },

        }
    }else{
        Vec2{
            x: intersection.x ,
            y: intersection.y //+ 0.5 //chuck it over to the next spot
        }
    };
    let potential_wall: Wall = gs.get_world_cell_at_vec2_pos(grab_cell_pos,  changed_frame);
    if potential_wall.full {
        return Some((potential_wall, grab_cell_pos))
    }else{
        return None;
    }
}

fn check_over_x_step(gs : &GameState, xstep : f32,ystep: f32, intersection : Vec2<f32>, changed_frame : bool) -> Option<(Wall, Vec2<f32>)>{
    let grab_cell_pos = if xstep.is_sign_negative(){
        Vec2{
            x: if intersection.x - 0.5 > 0.0 {
                intersection.x - 0.5
            }else{
                0.0
            },
            y: intersection.y  //chuck it over to the next spot
        }
    }else{
        Vec2{
            x: intersection.x ,//+ 0.5,
            y: intersection.y  //chuck it over to the next spot
        }
    };
    let potential_wall: Wall = gs.get_world_cell_at_vec2_pos(grab_cell_pos,  changed_frame);
    if potential_wall.full {
        return Some((potential_wall, grab_cell_pos))
    }else{
        return None;
    }
}

#[allow(unused_variables)]
fn x_walk(gs : &GameState, xstep : f32, x_axis_intersection: &mut Vec2<f32>, ystep: f32, changed_frame : bool, buf_handle : &mut BufWriter<StdoutLock>) -> Option<(Wall,Vec2<f32>)>{
    let potential_wall: Wall = gs.get_world_cell_at_vec2_pos(x_axis_intersection.to_owned(), changed_frame);
    if potential_wall.full {

        if changed_frame{
            writeln!(buf_handle, "X WALL FOUND at {:?}, {:?}", x_axis_intersection.x, x_axis_intersection.y);
        }
        //dirty
        return Some((potential_wall, *x_axis_intersection))
    } else {
        *x_axis_intersection = advance_x_intersection(x_axis_intersection, xstep, ystep);
        return None;
    }
}
#[allow(unused_variables)]
fn y_walk(gs : &GameState, ystep : f32, y_axis_intersection: &mut Vec2<f32>, xstep: f32, changed_frame : bool, buf_handle : &mut BufWriter<StdoutLock>) -> Option<(Wall,Vec2<f32>)>{
    let potential_wall : Wall = gs.get_world_cell_at_vec2_pos(y_axis_intersection.to_owned(),  changed_frame);
    if potential_wall.full {

        if changed_frame{
            writeln!(buf_handle,"Y WALL FOUND at {:?}, {:?}", y_axis_intersection.x, y_axis_intersection.y);
        }
        //dirty
        return Some((potential_wall, *y_axis_intersection));
    }else{
        //TODO clean this up
        *y_axis_intersection = advance_y_intersection(y_axis_intersection, xstep, ystep);
        return None;
    }
}

//TODO CURRENT, HEIGHT SCALING
///DDA Algorithm using initial x and y axis intersections
pub fn find_wall_and_distance(gs : &GameState, ray : Ray2D, changed_frame : bool) -> Option<(Wall, Vec2<f32>)> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut buf_handle = io::BufWriter::new(handle);

    let (mut xstep,mut ystep) = gen_dda_steps(ray);

    if xstep.is_infinite(){
        xstep = WORLD_CELL_SIZE as f32;
    }

    if ystep.is_infinite(){
        ystep = WORLD_CELL_SIZE as f32;
    }

    if changed_frame{
        writeln!(buf_handle,"xstep: {:?}, ystep: {:?}", xstep, ystep);
    }

    let (mut x_axis_intersection,mut y_axis_intersection) = find_axis_intersections(gs.p.pos, ray, xstep, ystep);

    if changed_frame {
        writeln!(buf_handle,"\n*********\nRAY: {:?}",ray);
        writeln!(buf_handle,"STEPS X: {:?}, Y: {:?}", xstep, ystep);
        writeln!(buf_handle,"\nSTART STEPS {:?} {:?}", xstep, ystep); //TODO FIX STEP == INF CASE
        writeln!(buf_handle,"PLAYER POS: {:?} RAY DIRECTION: {:?}", gs.p.pos, ray.dir);
        writeln!(buf_handle,"Initial intersections: X:{:#?} \n Y:{:#?}\n", x_axis_intersection, y_axis_intersection);
    }


    'finding_wall: loop {
        let x_dir_oob: bool = out_of_world_bounds(x_axis_intersection);
        let y_dir_oob: bool = out_of_world_bounds(y_axis_intersection);


        let dist_x_inter = x_axis_intersection.diff(&gs.p.pos).dist_rayfishfix(ray);
        let dist_y_inter = y_axis_intersection.diff(&gs.p.pos).dist_rayfishfix(ray);

//        let dist_x_inter = x_axis_intersection.diff(&gs.p.pos).length(); //.dist_rayfishfix(ray);
//        let dist_y_inter = y_axis_intersection.diff(&gs.p.pos).length(); //.dist_rayfishfix(ray);

        let walk_res;

        if x_dir_oob && y_dir_oob { //The ray has hit out of bounds
            //println!("Done, returning None");
            return None;
        }else if x_dir_oob || xstep == 0.0 {
            if changed_frame{
                writeln!(buf_handle,"FIRST PATH WALK");
            }

            walk_res = y_walk(gs, ystep, &mut y_axis_intersection, xstep, changed_frame, &mut buf_handle);
        }else if y_dir_oob || ystep == 0.0 {
            if changed_frame{
                writeln!(buf_handle,"SECOND PATH WALK");
            }
            walk_res = x_walk(gs, xstep, &mut x_axis_intersection, ystep, changed_frame, &mut buf_handle);
        }else{
            if changed_frame {
                write!(buf_handle, "X inter {:?}, Y inter {:?}, X dist {:?}, Y dist {:?}\t\t==>", x_axis_intersection, y_axis_intersection, dist_x_inter, dist_y_inter);
            }

            if dist_x_inter <= dist_y_inter  {
                if let Some((wall, dist)) = check_over_x_step(gs, xstep,ystep, x_axis_intersection,changed_frame){
                    if changed_frame{
                        writeln!(buf_handle,"THE WALL IS FOUND at x inter {:?}", x_axis_intersection);
                    }
                    return Some((wall,dist));
                }
                walk_res = x_walk(gs, xstep, &mut x_axis_intersection, ystep, changed_frame,&mut buf_handle);
                if changed_frame{
                    writeln!(buf_handle,"THIRD PATH WALK X_WALK new x inter x: {:?} y: {:?}", x_axis_intersection.x, x_axis_intersection.y);

                }
            }else{
                if let Some((wall, dist)) = check_over_y_step(gs, xstep,ystep, y_axis_intersection,changed_frame){
                    if changed_frame{
                        writeln!(buf_handle,"THE WALL IS FOUND at y inter {:?}", y_axis_intersection);
                    }
                    return Some((wall,dist));
                }

                walk_res = y_walk(gs, ystep, &mut y_axis_intersection, xstep, changed_frame, &mut buf_handle);
                if changed_frame{
                    writeln!(buf_handle,"THIRD PATH WALK Y_WALK new y inter x: {:?} y: {:?}", y_axis_intersection.x, y_axis_intersection.y,);
                }

            }
        }

        if let Some((wall, dist)) = walk_res {
            if changed_frame{
                writeln!(buf_handle,"THE WALL IS FOUND end of loop??");
            }
            return Some((wall,dist));
        }


    }
}

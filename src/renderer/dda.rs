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
use world::DebugWindowFlags;

pub struct caster<'a>{
    gs : &'a GameState,
    dflags : &'a DebugWindowFlags,
    changed_frame : bool,

    ray : Ray2D,
    xstep : f32,
    ystep : f32,
}

impl<'a> caster<'a> {
    pub fn new(gs : &'a GameState, ray : Ray2D, changed_frame : bool, dflags : &'a DebugWindowFlags) -> caster<'a>{
        let (dx,dy) = ray.gen_dda_steps();

        caster{
            gs,
            dflags,
            changed_frame,
            ray,
            xstep : dx,
            ystep : dy,
        }
    }

    //todo make sure this is correct
    fn find_axis_intersections(&self, position : Vec2<f32>) -> (Vec2<f32>, Vec2<f32>){
        let a_x : i32 = if self.xstep == 0.0{
            position.x.round() as i32
        }else{
            find_next_cell_boundary(position.x, self.ray.dir.x.is_sign_positive())
        };

        let b_y: i32 = if self.ystep == 0.0{
            position.y.floor() as i32
        }else{
            find_next_cell_boundary(position.y, self.ray.dir.y.is_sign_positive())
        };

        let x_axis_inter: Vec2<f32> = Vec2::<f32> {
            x: a_x as f32,
            y: position.y + (((a_x as f32 - position.x)/self.ray.dir.x)*self.ray.dir.y)
        };
        let y_axis_inter: Vec2<f32> = Vec2::<f32> {
            x: position.x + (((b_y as f32 - position.y)/self.ray.dir.y)*self.ray.dir.x),
            y: b_y as f32,
        };

        (x_axis_inter, y_axis_inter)
    }

    fn y_walk(&self, y_axis_intersection: &mut Vec2<f32>, buf_handle : &mut BufWriter<StdoutLock>) -> Option<(Wall,Vec2<f32>)>{
        let grab_cell_pos = if self.ystep.is_sign_negative(){

            Vec2{
                x: y_axis_intersection.x ,
                //chuck it over to the next spot
                y: if y_axis_intersection.y - 0.5> 0.0 {
                    y_axis_intersection.y - 0.5
                }else{
                    0.0
                },

            }
        }else{
            Vec2{
                x: y_axis_intersection.x ,
                y: y_axis_intersection.y //+ 0.5 //chuck it over to the next spot
            }
        };

        let potential_wall : Wall = self.gs.get_world_cell_at_vec2_pos(grab_cell_pos,  self.changed_frame);
        if potential_wall.full {

            if self.changed_frame{
                writeln!(buf_handle,"Ray{:?}\t{:?} Y WALL FOUND at {:?}, {:?}", self.ray.ray_number, potential_wall.color, y_axis_intersection.x, y_axis_intersection.y);
            }
            //dirty
            return Some((potential_wall, *y_axis_intersection));
        }else{
            //TODO clean this up
            *y_axis_intersection = advance_y_intersection(y_axis_intersection, self.xstep, self.ystep);
            return None;
        }
    }

    fn x_walk(&self, x_axis_intersection: &mut Vec2<f32>, buf_handle : &mut BufWriter<StdoutLock>) -> Option<(Wall,Vec2<f32>)>{
        let grab_cell_pos = if self.xstep.is_sign_negative(){
            Vec2{
                x: if x_axis_intersection.x - 0.5 > 0.0 { //deal with rounding up on borders of cells
                    x_axis_intersection.x - 0.5
                }else{
                    0.0
                },
                y: x_axis_intersection.y  //chuck it over to the next spot
            }
        }else{
            Vec2{
                x: x_axis_intersection.x ,//+ 0.5,
                y: x_axis_intersection.y  //chuck it over to the next spot
            }
        };

        let potential_wall: Wall = self.gs.get_world_cell_at_vec2_pos(grab_cell_pos, self.changed_frame);
        if potential_wall.full {

            if self.changed_frame{
                writeln!(buf_handle, "Ray {:?}\t{:?} X WALL FOUND at {:?}, {:?}", self.ray.ray_number, potential_wall.color, x_axis_intersection.x, x_axis_intersection.y);
            }
            //dirty
            return Some((potential_wall, *x_axis_intersection))
        } else {
            *x_axis_intersection = advance_x_intersection(x_axis_intersection, self.xstep, self.ystep);
            return None;
        }
    }

    //TODO CURRENT, HEIGHT SCALING
    ///DDA Algorithm using initial x and y axis intersections
    pub fn find_wall_and_distance(self) -> Option<(Wall, Vec2<f32>)> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let mut buf_handle = io::BufWriter::new(handle);

//        if self.changed_frame && self.dflags.inspect_ray.is_some(){
//            writeln!(buf_handle,"xstep: {:?}, ystep: {:?}", self.xstep, self.ystep);
//        }

        let (mut x_axis_intersection,mut y_axis_intersection) = self.find_axis_intersections(self.gs.p.pos);

//        if self.changed_frame && self.dflags.inspect_ray.is_some(){
//            writeln!(buf_handle,"\n*********\nRAY: {:?}",self.ray);
//            writeln!(buf_handle,"STEPS X: {:?}, Y: {:?}", self.xstep, self.ystep);
//            writeln!(buf_handle,"\nSTART STEPS {:?} {:?}", self.xstep, self.ystep); //TODO FIX STEP == INF CASE
//            writeln!(buf_handle,"PLAYER POS: {:?} RAY DIRECTION: {:?}", self.gs.p.pos, self.ray.dir);
//            writeln!(buf_handle,"Initial intersections: X:{:#?} \n Y:{:#?}\n", x_axis_intersection, y_axis_intersection);
//        }

        'finding_wall: loop {
            let x_dir_oob: bool = out_of_world_bounds(x_axis_intersection);
            let y_dir_oob: bool = out_of_world_bounds(y_axis_intersection);

            let dist_x_inter = x_axis_intersection.diff(&self.gs.p.pos).dist_rayfishfix(self.ray);
            let dist_y_inter = y_axis_intersection.diff(&self.gs.p.pos).dist_rayfishfix(self.ray);

            let mut walk_res;

            if x_dir_oob && y_dir_oob { //The ray has hit out of bounds
                return None;
            }else if x_dir_oob || self.xstep == 0.0 {
                walk_res = self.y_walk(&mut y_axis_intersection,  &mut buf_handle);
            }else if y_dir_oob || self.ystep == 0.0 {
                walk_res = self.x_walk( &mut x_axis_intersection, &mut buf_handle);
            }else{
                if dist_x_inter <= dist_y_inter  {
                    walk_res = self.x_walk(&mut x_axis_intersection,&mut buf_handle);
                }else{
                    walk_res = self.y_walk(&mut y_axis_intersection, &mut buf_handle);
                }
            }

            if let Some((wall, dist)) = walk_res {
                return Some((wall,dist));
            }
        }
    }
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




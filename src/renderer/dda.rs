use renderer::vector::Vec2;
use renderer::ray2D::Ray2D;
use world::out_of_world_bounds;
use world::wall::Wall;
use world::player::Player;
use world::get_world_cell_at_vec2_pos;
use world::WORLD_CELL_SIZE;
use world::find_next_cell_boundary;


fn find_axis_intersections(position : Vec2<f32>, ray: Ray2D) -> (Vec2<f32>, Vec2<f32>){
    let a_x = find_next_cell_boundary(position.x, ray.dir.x.is_sign_positive());
    let b_y = find_next_cell_boundary(position.y, ray.dir.y.is_sign_positive());

    let x_axis_inter: Vec2<f32> = Vec2::<f32> {
        x: a_x as f32,
        y: ray.at(a_x as f32),
    };
    let y_axis_inter: Vec2<f32> = Vec2::<f32> {
        x: (b_y as f32 - ray.get_y_intercept()) / ray.get_slope(),
        y: b_y as f32,
    };

    (x_axis_inter, y_axis_inter)
}

fn advance_x_intersection(x_axis_intersection : &Vec2<f32>, y_step : f32) -> Vec2<f32> {
    Vec2 {
        x: x_axis_intersection.x + WORLD_CELL_SIZE as f32,
        y: x_axis_intersection.y + y_step,
    }
}

//TODO clean this up
fn advance_y_intersection(y_axis_intersection : &Vec2<f32>, x_step : f32) -> Vec2<f32> {
    Vec2 {
        x: y_axis_intersection.x + x_step,
        y: y_axis_intersection.y + WORLD_CELL_SIZE as f32,
    }
}

fn gen_dda_steps(p : &Player, ray : &Ray2D) -> (f32,f32) {
    let xstep = if p.dir.x.is_sign_positive(){
        (1.0 / ray.dir.x).abs() * WORLD_CELL_SIZE as f32
    }else{
        -1.0 * (1.0 / ray.dir.x).abs() * WORLD_CELL_SIZE as f32
    };

    let ystep = if p.dir.y.is_sign_positive() {
        (1.0 / ray.dir.y).abs() * WORLD_CELL_SIZE as f32
    }else {
        -1.0 * (1.0 / ray.dir.y).abs() * WORLD_CELL_SIZE as f32
    };

    (xstep, ystep)
}
#[allow(unused_variables)]
fn x_walk(theworld : &Vec<Vec<Wall>>, xstep : f32, dist_x_inter : Vec2<f32>, x_axis_intersection: &mut Vec2<f32>, ystep: f32) -> Option<(Wall,Vec2<f32>)>{
    //println!("X walk, xstep:{:?} x_dist:{:?}", xstep, dist_x_inter);
    let potential_wall: Wall = get_world_cell_at_vec2_pos(x_axis_intersection.to_owned(), &theworld);
    if potential_wall.full {
        //println!("WALL FOUND!!");
        return Some((potential_wall, dist_x_inter))
    } else {
        *x_axis_intersection = advance_x_intersection(x_axis_intersection, ystep);
        return None;
    }
}
#[allow(unused_variables)]
fn y_walk(theworld : &Vec<Vec<Wall>>, ystep : f32, dist_y_inter : Vec2<f32>, y_axis_intersection: &mut Vec2<f32>, xstep: f32) -> Option<(Wall,Vec2<f32>)>{
    //println!("Y walk, ystep:{:?} y_dist:{:?}", ystep, dist_y_inter);
    let potential_wall : Wall = get_world_cell_at_vec2_pos(y_axis_intersection.to_owned(), theworld);
    if potential_wall.full {
        //println!("WALL FOUND!!");
        return Some((potential_wall, dist_y_inter));
    }else{
        //TODO clean this up
        *y_axis_intersection = advance_y_intersection(y_axis_intersection, xstep);
        return None;
    }
}

//TODO CURRENT, HEIGHT SCALING
///DDA Algorithm using initial x and y axis intersections
pub fn find_wall_and_distance(theworld: &Vec<Vec<Wall>>, p : &Player, ray : Ray2D) -> Option<(Wall, Vec2<f32>)> {
    let (xstep, ystep) = gen_dda_steps(p,&ray);
    let (mut x_axis_intersection,mut y_axis_intersection) = find_axis_intersections(p.pos, ray);

//    debug_print_world(theworld, p.pos);
//    println!("RAY: {:?}",ray);
//    println!("STEPS X: {:?}, Y: {:?}", xstep, ystep);
//    println!("\nSTART STEPS {:?} {:?}", xstep, ystep); //TODO FIX STEP == INF CASE
    //println!("PLAYER POS: {:?} RAY DIRECTION: {:?}", p.pos, ray.dir);

    'finding_wall: loop {
        let x_dir_oob: bool = out_of_world_bounds(x_axis_intersection);
        let y_dir_oob: bool = out_of_world_bounds(y_axis_intersection);

        let dist_x_inter = x_axis_intersection.diff(&p.pos);
        let dist_y_inter = y_axis_intersection.diff(&p.pos);

        //let dist_x_inter = x_axis_intersection.dist(&p.pos);
        //let dist_y_inter = y_axis_intersection.dist(&p.pos);

        let walk_res;

        if x_dir_oob && y_dir_oob { //The ray has hit out of bounds
            //println!("Done, returning None");
            return None;
        }else if x_dir_oob {
            //println!("FIRST PATH WALK");
            walk_res = y_walk(theworld, ystep, dist_y_inter, &mut y_axis_intersection, xstep);
        }else if y_dir_oob {
            //println!("SECOND PATH WALK");
            walk_res = x_walk(theworld, xstep, dist_x_inter, &mut x_axis_intersection, ystep);
        }else{
            if dist_x_inter.length() <= dist_y_inter.length()  {
                //println!("THIRD PATH WALK");
                walk_res = x_walk(theworld, xstep, dist_x_inter, &mut x_axis_intersection, ystep);
            }else{
                walk_res = y_walk(theworld, ystep, dist_y_inter, &mut y_axis_intersection, xstep);
            }
        }

        if let Some((wall, dist)) = walk_res {
            //println!("THE WALL IS FOUND");
            return Some((wall,dist));
        }

//        println!("Grab stdin");
//        let mut stdin = io::stdin();
//        let _ = stdin.read(&mut [0u8]).unwrap();
    }
}

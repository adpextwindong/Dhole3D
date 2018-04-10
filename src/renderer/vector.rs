use std::ops::{Div, Mul, Add};
use num_traits;

#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Div + Mul + Add> Vec2<T>{
    pub fn slope(self) -> <T as Div>::Output {
        self.y / self.x
    }
    /// Computes the distance between two vectors
    pub fn dist(&self, other: &Vec2<T>) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Copy + num_traits::float::Float
    {
        ((self.x * other.x) + (self.y * other.y)).sqrt()
    }

    pub fn get_x(self) -> T {
        self.y
    }

    pub fn get_y(self) -> T {
        self.y
    }
}

pub trait Vector {
    //fn norm(&self) -> Vec2;
    fn length(&self) -> f32;
}
impl Vector for Vec2<f32> {
    /// Returns the length of the vector
    fn length(&self) -> f32
    {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

//https://en.wikipedia.org/wiki/Rotation_matrix
pub fn rotate_counter_clockwise(v: Vec2<f32>, theta: f32) -> Vec2<f32> {
    return Vec2 {
        x: (v.x * theta.cos()) - (v.y * theta.sin()),
        y: (v.x * theta.sin()) + (v.y * theta.cos()),
    };
}

pub fn rotate_clockwise(v: Vec2<f32>, theta: f32) -> Vec2<f32> {
    return Vec2 {
        x: (v.x * theta.cos()) + (v.y * theta.sin()),
        y: (v.x * -theta.sin()) + (v.y * theta.cos()),
    };
}

pub fn norm<T: Div + Mul + Add>(v: Vec2<f32>) -> Vec2<f32>
{
    let l = v.length();
    return Vec2::<f32> { x: v.x / l, y: v.y };
}
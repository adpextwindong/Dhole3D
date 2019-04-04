use std::ops::{Add, Sub, Div};
use num_traits::Float as Float;
use std::ops::AddAssign;
use sdl2::rect::Point;
use renderer::ray2D::Ray2D;

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Vec2<T>{
    pub fn slope(self) -> <T as Div>::Output {
        self.y / self.x
    }
    /// Computes the distance between two vectors
    pub fn dist(&self, other: &Vec2<T>) -> T
    where
        T: Float
    {
        (((self.x - other.x).powi(2)) + ((self.y - other.y).powi(2))).sqrt()
    }

    pub fn get_x(self) -> T {
        self.y
    }

    pub fn get_y(self) -> T {
        self.y
    }

    pub fn length(self) -> T{
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn normalize(mut self) {
        let l = self.length();
        self.x = self.x / l;
        self.y = self.y / l;
    }

    pub fn normalized(self) -> Vec2<T>{
        let l = self.length();
        Vec2{
            x: self.x / l,
            y: self.y / l
        }
    }

    pub fn diff(&self, other: &Vec2<T>) -> Vec2<T>{
        Vec2{
            x: other.x - self.x,
            y: other.y - self.y
        }
    }

    pub fn scale(self, scale : T) -> Vec2<T> {
        Vec2{
            x: self.x * scale,
            y: self.y * scale
        }
    }
}

impl Vec2<f32>{
    pub fn angle(self) -> f32{
        f32::atan2(self.y,self.x)
    }

    pub fn dist_rayfishfix(self, ray : Ray2D) -> f32{
        let ang = ray.dir.normalized().angle();
        Vec2{
            x: (self.x * f32::cos(ang)) - (self.y * f32::sin(ang)),
            y: (self.x * f32::sin(ang)) + (self.y * f32::cos(ang))
        }.length()
    }
}

impl<T: Float> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> <Self as Add<Vec2<T>>>::Output {
        Vec2{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl<T: Float> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> <Self as Sub<Vec2<T>>>::Output {
        Vec2{
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}


impl<T: Float> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>){
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
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

#[cfg(test)]
mod tests{
    #![allow(dead_code)]
    #![allow(unused_imports)]
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(deprecated)]
    #![allow(unused_macros)]

    use super::*;
}
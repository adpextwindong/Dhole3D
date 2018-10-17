use std::ops::Div;
use num_traits::Float as Float;

#[derive(Copy, Clone, Debug)]
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
        ((self.x * other.x) + (self.y * other.y)).sqrt()
    }

    pub fn get_x(self) -> T {
        self.y
    }

    pub fn get_y(self) -> T {
        self.y
    }

    pub fn length(&self) -> T{
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn norm(self) -> Vec2<T> {
        let l = self.length();
        return Vec2::<T> { x: self.x / l, y: self.y / l};
    }
    pub fn normalize(mut self) {
        let l = self.length();
        self.x = self.x / l;
        self.y = self.y / l;
    }

    pub fn diff(&self, other: &Vec2<T>) -> Vec2<T>{
        Vec2{
            x: other.x - self.x,
            y: other.y - self.y
        }
    }

}
impl Vec2<f32>{
    pub fn angle(self) -> f32{
        f32::atan2(self.y,self.x)
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
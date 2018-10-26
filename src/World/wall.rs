use sdl2::pixels::Color;

// TODO Add surface/texutre part
#[derive(Copy, Clone, Debug)]
pub struct Wall {
    pub full: bool,
    pub color: Color,
}

pub const NULL_WALL: Wall = Wall {
    full: false,
    color: NULL_COLOR,
};


pub const NULL_COLOR: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};
pub const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};

pub const FLOOR_GREY: Color = Color {
    r: 128,
    g: 128,
    b: 128,
    a: 255,
};
pub const CEILING_BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
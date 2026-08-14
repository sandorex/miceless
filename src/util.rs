#![allow(unused)]

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, y
        }
    }

    pub fn offset(&self, x: i32, y: i32) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}

impl Into<sdl3::render::FPoint> for Point {
    fn into(self) -> sdl3::render::FPoint {
        sdl3::render::FPoint::new(self.x as f32, self.y as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction4 {
    Top,
    Bottom,
    Left,
    Right,
}

impl Into<Direction8> for Direction4 {
    fn into(self) -> Direction8 {
        match self {
            Self::Top => Direction8::Top,
            Self::Bottom => Direction8::Bottom,
            Self::Left => Direction8::Left,
            Self::Right => Direction8::Right,
        }
    }
}

/// Like `Direction` but with inbetween directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction8 {
    Top,
    TopRight,
    TopLeft,
    Bottom,
    BottomRight,
    BottomLeft,
    Left,
    Right,
    Center,
}

impl TryInto<Direction4> for Direction8 {
    type Error = ();

    fn try_into(self) -> Result<Direction4, Self::Error> {
        match self {
            Self::Top => Ok(Direction4::Top),
            Self::Bottom => Ok(Direction4::Bottom),
            Self::Left => Ok(Direction4::Left),
            Self::Right => Ok(Direction4::Right),

            _ => Err(())
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x, y, w, h
        }
    }

    pub fn new_centered(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self::new(x - w / 2, y - h / 2, w, h)
    }

    pub fn area(&self) -> u32 {
        self.w.unsigned_abs() * self.h.unsigned_abs()
    }

    pub fn center(&self) -> Point {
        Point::new(self.x + self.w / 2, self.y + self.h / 2)
    }

    pub fn offset_center(&self, x: i32, y: i32) -> Self {
        Self::new(self.x + x, self.y + y, self.w, self.h)
    }

    pub fn offset(&self, w: i32, h: i32) -> Self {
        Self::new(self.x + w / 2, self.y + h / 2, self.w - w / 2 * 2, self.h - h / 2 * 2)
    }

    /// Divides rect into a grid and returns the specified rect
    pub fn divide(&self, grid_x: i32, grid_y: i32, index_x: i32, index_y: i32) -> Self {
        let x = self.w / grid_x.abs();
        let y = self.h / grid_y.abs();

        Self::new(self.x + x * index_x, self.y + y * index_y, x, y)
    }

    /// Havles the rectangle in specified direction
    pub fn half(&self, direction: Direction4) -> Self {
        match direction {
            Direction4::Top => self.divide(1, 2, 0, 0),
            Direction4::Bottom => self.divide(1, 2, 0, 1),
            Direction4::Left => self.divide(2, 1, 0, 0),
            Direction4::Right => self.divide(2, 1, 1, 0),
        }
    }

    /// Divides rect into 3x3 grid and returns specified piece
    pub fn third(&self, direction: Direction8) -> Self {
        match direction {
            Direction8::Top => self.divide(3, 3, 1, 0),
            Direction8::TopLeft => self.divide(3, 3, 0, 0),
            Direction8::TopRight => self.divide(3, 3, 2, 0),

            Direction8::Bottom => self.divide(3, 3, 1, 2),
            Direction8::BottomLeft => self.divide(3, 3, 0, 2),
            Direction8::BottomRight => self.divide(3, 3, 2, 2),

            Direction8::Left => self.divide(3, 3, 0, 1),
            Direction8::Center => self.divide(3, 3, 1, 1),
            Direction8::Right => self.divide(3, 3, 2, 1),
        }
    }
}

impl Into<sdl3::rect::Rect> for Rect {
    fn into(self) -> sdl3::rect::Rect {
        // TODO double check this
        sdl3::rect::Rect::new(self.x, self.y, self.w.abs() as u32, self.h.abs() as u32)
    }
}

impl Into<sdl3::render::FRect> for Rect {
    fn into(self) -> sdl3::render::FRect {
        // TODO double check this
        sdl3::render::FRect::new(self.x as f32, self.y as f32, self.w.abs() as f32, self.h.abs() as f32)
    }
}

impl From<sdl3::rect::Rect> for Rect {
    fn from(value: sdl3::rect::Rect) -> Self {
        Self::new(value.x, value.y, value.w, value.h)
    }
}

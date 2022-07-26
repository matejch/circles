use std::fmt;
use web_sys::console;
use crate::ball::is_point_in_rect;
use crate::geometry::Cells::{BottomLeft, BottomRight, TopLeft, TopRight};
use crate::random::{random_range, random_sign, random_velocity};


pub enum Cells {
    TopLeft = 1,
    TopRight = 2,
    BottomLeft = 3,
    BottomRight = 4,
}


#[derive(Debug, Copy, Clone)]
pub struct RenderingRect {
    pub rect: Rect,
    pub many: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn intersects(&self, other: Rect) -> bool {
        let top_left = Point { x: self.x, y: self.y }.is_in_rect(&other);
        let top_right = Point { x: self.x + self.w, y: self.y }.is_in_rect(&other);
        let bottom_left = Point { x: self.x, y: self.y + self.h }.is_in_rect(&other);
        let bottom_right = Point { x: self.x + self.w, y: self.y + self.h }.is_in_rect(&other);

        if top_left || top_right || bottom_left || bottom_right {
            return true;
        }

        false
    }

    pub fn fits(&self, other: Rect) -> bool {
        let top_left = Point { x: self.x, y: self.y }.is_in_rect(&other);
        let top_right = Point { x: self.x + self.w, y: self.y }.is_in_rect(&other);
        let bottom_left = Point { x: self.x, y: self.y - self.h }.is_in_rect(&other);
        let bottom_right = Point { x: self.x + self.w, y: self.y - self.h }.is_in_rect(&other);

        if top_left && top_right && bottom_left && bottom_right {
            return true;
        }

        false
    }

    pub fn split_to_four_cells(&self) -> (Rect, Rect, Rect, Rect) {
        let new_w = self.w / 2.0;
        let new_h = self.h / 2.0;

        let top_left = Rect { x: self.x, y: self.y, w: new_w, h: new_h };
        let top_right = Rect { x: self.x + new_w, y: self.y, w: new_w, h: new_h };
        let bottom_left = Rect { x: self.x, y: self.y + new_h, w: new_w, h: new_h };
        let bottom_right = Rect { x: self.x + new_w, y: self.y + new_h, w: new_w, h: new_h };

        return (top_left, top_right, bottom_left, bottom_right);
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.w / 2.0,
            y: self.y + self.h / 2.0,
        }
    }

    pub fn where_is_point_relative_to_center(self, point: Point) -> Cells {
        let center = self.center();

        if center.x < point.x {
            if center.y < point.y {
                return BottomRight;
            } else {
                return TopRight;
            }
        } else if center.y < point.y {
            //console::log_1(&format!("where is point? {:#?} {:#?}, {:#?} ", self, point, center).into());

            return BottomLeft;
        } else {
            return TopLeft;
        }
    }

    pub fn top_left(&self) -> Rect {
        let new_w = self.w / 2.0;
        let new_h = self.h / 2.0;
        Rect { x: self.x, y: self.y, w: new_w, h: new_h }
    }

    pub fn top_right(&self) -> Rect {
        let new_w = self.w / 2.0;
        let new_h = self.h / 2.0;
        Rect { x: self.x + new_w, y: self.y, w: new_w, h: new_h }
    }

    pub fn bottom_left(&self) -> Rect {
        let new_w = self.w / 2.0;
        let new_h = self.h / 2.0;

        Rect { x: self.x, y: self.y + new_h, w: new_w, h: new_h }
    }

    pub fn bottom_right(&self) -> Rect {
        let new_w = self.w / 2.0;
        let new_h = self.h / 2.0;

        Rect { x: self.x + new_w, y: self.y + new_h, w: new_w, h: new_h }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.x, self.y)
    }
}

impl Point {
    pub fn random_point(width: usize, height: usize) -> Self {
        Self {
            x: random_range(width / 10, 9 * width / 10) as f64,
            y: random_range(width / 10, 9 * height / 10) as f64,

        }
    }
    pub fn random_velocity(min: f64, max: f64) -> Self {
        Self {
            x: random_sign() * random_velocity(min, max),
            y: random_sign() * random_velocity(min, max),
        }
    }

    pub fn is_in_rect(&self, rect: &Rect) -> bool {
        return is_point_in_rect(rect.x, rect.y, rect.w, rect.h, self.x, self.y);
    }

    fn is_point_in_rect(x: f64, y: f64, w: f64, h: f64, point_x: f64, point_y: f64) -> bool {
        if point_x > x
            && point_x < x + w
            && point_y > y
            && point_y < y + h {
            return true;
        }
        false
    }
}
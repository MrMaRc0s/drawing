use std::f64::consts::PI;

use raster::{Color, Image};
use rand::Rng;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

fn stroke_line(image: &mut Image, start: &Point, end: &Point, color: &Color) {
    let color = color.clone();
    let (mut x1, mut y1) = (start.x, start.y);
    let (mut x2, mut y2) = (end.x, end.y);

    let steep = (y2 - y1).abs() > (x2 - x1).abs();
    if steep {
        (x1, y1) = (y1, x1);
        (x2, y2) = (y2, x2);
    }

    if x1 > x2 {
        (x1, y1, x2, y2) = (x2, y2, x1, y1);
    }

    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let y_step = if y1 < y2 { 1 } else { -1 };
    let mut error = 2 * dy - dx;
    let mut y = y1;

    for x in x1..=x2 {
        if steep {
            image.display(y, x, color.clone());
        } else {
            image.display(x, y, color.clone());
        }
        if error > 0 {
            y += y_step;
            error -= 2 * dx;
        }
        error += 2 * dy;
    }
}

// --- Traits ---

/// Interface for objects that can be rendered to a canvas.
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

/// Interface for the drawing surface itself.
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// --- Point ---

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self { 
            x: rng.gen_range(0..width), 
            y: rng.gen_range(0..height) 
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        let c = self.color();
        image.display(self.x, self.y, c);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 0)
    }
}

// --- Line ---

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    #[allow(dead_code)]
    pub fn new(start: &Point, end: &Point) -> Self {
        Self { start: start.clone(), end: end.clone() }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self { 
            start: Point::random(width, height), 
            end: Point::random(width, height) 
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        stroke_line(image, &self.start, &self.end, &color);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }
}

// --- Rectangle ---

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    /// Normalizes points so top_left is always the minimum coordinates.
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);

        Self {
            top_left: Point::new(min_x, min_y),
            bottom_right: Point::new(max_x, max_y),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let c = self.color();
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);

        stroke_line(image, &self.top_left, &top_right, &c);
        stroke_line(image, &top_right, &self.bottom_right, &c);
        stroke_line(image, &self.bottom_right, &bottom_left, &c);
        stroke_line(image, &bottom_left, &self.top_left, &c);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }
}

// --- Triangle ---

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self { a: a.clone(), b: b.clone(), c: c.clone() }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let c = self.color();
        stroke_line(image, &self.a, &self.b, &c);
        stroke_line(image, &self.b, &self.c, &c);
        stroke_line(image, &self.c, &self.a, &c);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }
}

// --- Circle ---

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    #[allow(dead_code)]
    pub fn new(center: &Point, radius: i32) -> Self {
        Self { center: center.clone(), radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self { 
            center: Point::random(width, height), 
            radius: rng.gen_range(5..=width / 4) 
        }
    }
}

impl Drawable for Circle {
    /// Midpoint Circle Algorithm implementation.
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 1 - x;

        while x >= y {
            image.display(self.center.x + x, self.center.y + y, color.clone());
            image.display(self.center.x + y, self.center.y + x, color.clone());
            image.display(self.center.x - y, self.center.y + x, color.clone());
            image.display(self.center.x - x, self.center.y + y, color.clone());
            image.display(self.center.x - x, self.center.y - y, color.clone());
            image.display(self.center.x - y, self.center.y - x, color.clone());
            image.display(self.center.x + y, self.center.y - x, color.clone());
            image.display(self.center.x + x, self.center.y - y, color.clone());

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x) + 1;
            }
        }
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255))
    }
}

// --- Pentagon (bonus) ---

/// Regular pentagon in screen space (`center`, circumscribed `radius`).
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Pentagon {
    center: Point,
    radius: i32,
}

impl Pentagon {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: center.clone(),
            radius: radius.max(0),
        }
    }

    fn vertices(&self) -> [Point; 5] {
        let mut pts = core::array::from_fn(|_| Point::new(0, 0));
        let cx = self.center.x as f64;
        let cy = self.center.y as f64;
        let r = self.radius as f64;

        for i in 0..5 {
            let theta = -PI / 2.0 + (2.0 * PI * i as f64 / 5.0);
            let x = cx + r * theta.cos();
            let y = cy + r * theta.sin();
            pts[i] = Point::new(x.round() as i32, y.round() as i32);
        }
        pts
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let c = self.color();
        let v = self.vertices();
        for i in 0..5 {
            stroke_line(image, &v[i], &v[(i + 1) % 5], &c);
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 255)
    }
}

// --- Cube (bonus) ---

/// Axis-aligned cube in 3D, drawn as a wireframe with a simple isometric projection.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Cube {
    center: Point,
    half_edge: i32,
}

impl Cube {
    pub fn new(center: &Point, edge: i32) -> Self {
        let edge = edge.max(1);
        Self {
            center: center.clone(),
            half_edge: edge / 2,
        }
    }

    fn corner_model(i: u8, h: i32) -> (i32, i32, i32) {
        let p = |b: u8| if b == 0 { -h } else { h };
        let xb = i & 1;
        let yb = (i >> 1) & 1;
        let zb = (i >> 2) & 1;
        (p(xb), p(yb), p(zb))
    }

    fn project(&self, mx: i32, my: i32, mz: i32) -> Point {
        let xf = mx as f64;
        let yf = my as f64;
        let zf = mz as f64;
        let sx = self.center.x as f64 + xf - zf;
        let sy = self.center.y as f64 - yf - (xf + zf) * 0.5;
        Point::new(sx.round() as i32, sy.round() as i32)
    }

    fn screen_corner(&self, index: u8) -> Point {
        let (x, y, z) = Self::corner_model(index, self.half_edge);
        self.project(x, y, z)
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut Image) {
        let c = self.color();
        const EDGES: [(u8, u8); 12] = [
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 0),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
            (0, 4),
            (1, 5),
            (3, 7),
            (2, 6),
        ];
        for (a, b) in EDGES {
            let pa = self.screen_corner(a);
            let pb = self.screen_corner(b);
            stroke_line(image, &pa, &pb, &c);
        }
    }

    fn color(&self) -> Color {
        Color::rgb(255, 165, 0)
    }
}



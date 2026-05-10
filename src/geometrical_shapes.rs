use raster::{Color, Image};
use rand::Rng;

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
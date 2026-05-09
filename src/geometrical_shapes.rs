pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn clone(&self) -> Self {
        Self { x: self.x, y: self.y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self { x: rng.gen_range(0..width), y: rng.gen_range(0..height) }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, Color::Black);
    }
    
    fn color(&self) -> Color {
       let mut rng = rand::thread_rng();
       Color::Rgb(rng.gen_range(1..256), rng.gen_range(1..256), rng.gen_range(1..256))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Self { start: start.clone(), end: end.clone() }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self { start: Point::random(width, height), end: Point::random(width, height) }
    }
}

impl Drawable for Line {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::Rgb(rng.gen_range(1..256), rng.gen_range(1..256), rng.gen_range(1..256))
    }

    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let (mut x1, mut y1) = (self.start.x, self.start.y);
        let (mut x2, mut y2) = (self.end.x, self.end.y);

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
}
#[derive(Debug, Eq, PartialEq)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

#[derive(Debug, Eq, PartialEq)]
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
        Self { center: Point::random(width, height), radius: rng.gen_range(0..width) }
    }
}





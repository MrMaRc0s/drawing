mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    //Random Line
    gs::Line::random(image.width, image.height).draw(&mut image);

    // Random Point
    gs::Point::random(image.width, image.height).draw(&mut image);

    // Rectangle with reverse-order points
    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
    rectangle.draw(&mut image);

    // Triangle
    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    // FROM CLASSMATE: The circle testing loop
    // (Note: This assumes Circle::random is implemented in the shapes file)
    for _ in 0..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png"); 
    println!("Image generated: image.png");
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
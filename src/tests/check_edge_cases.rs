use super::*;
use raster::Image;

#[test]
fn line_with_same_start_and_end_draws_without_panicking() {
    let point = Point::new(69, 69);
    let line = Line::new(&point, &point);
    let mut image = Image::blank(169, 169);

    line.draw(&mut image);
}
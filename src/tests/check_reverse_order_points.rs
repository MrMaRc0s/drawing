use super::*;

#[test]
fn rectangle_normalizes_reverse_order_points() {
    let bottom_right_first = Rectangle::new(&Point::new(150, 300), &Point::new(50, 60));
    let bottom_left_first = Rectangle::new(&Point::new(50, 300), &Point::new(150, 60));

    assert_eq!(bottom_right_first.top_left, Point::new(50, 60));
    assert_eq!(bottom_right_first.bottom_right, Point::new(150, 300));
    assert_eq!(bottom_left_first.top_left, Point::new(50, 60));
    assert_eq!(bottom_left_first.bottom_right, Point::new(150, 300));
}
use super::*;

#[test]
fn random_point_stays_inside_image_dimensions() {
    let width = 69;
    let height = 69;

    for _ in 0..1_000 {
        let point = Point::random(width, height);

        assert!(point.x >= 0 && point.x < width);
        assert!(point.y >= 0 && point.y < height);
    }
}

#[test]
fn random_line_points_stay_inside_image_dimensions() {
    let width = 69;
    let height = 69;

    for _ in 0..1_000 {
        let line = Line::random(width, height);

        assert!(line.start.x >= 0 && line.start.x < width);
        assert!(line.start.y >= 0 && line.start.y < height);
        assert!(line.end.x >= 0 && line.end.x < width);
        assert!(line.end.y >= 0 && line.end.y < height);
    }
}

#[test]
fn random_circle_center_stays_inside_image_dimensions() {
    let width = 69;
    let height = 69;

    for _ in 0..1_000 {
        let circle = Circle::random(width, height);

        assert!(circle.center.x >= 0 && circle.center.x < width);
        assert!(circle.center.y >= 0 && circle.center.y < height);
    }
}
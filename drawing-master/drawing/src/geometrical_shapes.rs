use rand::Rng;
use raster::Color;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut dyn Displayable);
    fn color(&self) -> Color;
}

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut dyn Displayable) {
        let c = self.color();
        for dx in -1..=1 {
            for dy in -1..=1 {
                image.display(self.x + dx, self.y + dy, c.clone());
            }
        }
    }

    fn color(&self) -> Color {
        random_color()
    }
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Line { p1: *p1, p2: *p2 }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut dyn Displayable) {
        draw_line_with_color(&self.p1, &self.p2, image, self.color());
    }

    fn color(&self) -> Color {
        random_color()
    }
}

pub struct Rectangle {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            min_x: p1.x.min(p2.x),
            min_y: p1.y.min(p2.y),
            max_x: p1.x.max(p2.x),
            max_y: p1.y.max(p2.y),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let p1 = Point::new(self.min_x, self.min_y);
        let p2 = Point::new(self.max_x, self.min_y);
        let p3 = Point::new(self.max_x, self.max_y);
        let p4 = Point::new(self.min_x, self.max_y);

        let c = self.color();
        draw_line_with_color(&p1, &p2, image, c.clone());
        draw_line_with_color(&p2, &p3, image, c.clone());
        draw_line_with_color(&p3, &p4, image, c.clone());
        draw_line_with_color(&p4, &p1, image, c);
    }

    fn color(&self) -> Color {
        random_color()
    }
}

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle { p1: *p1, p2: *p2, p3: *p3 }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut dyn Displayable) {
        let c = self.color();
        draw_line_with_color(&self.p1, &self.p2, image, c.clone());
        draw_line_with_color(&self.p2, &self.p3, image, c.clone());
        draw_line_with_color(&self.p3, &self.p1, image, c);
    }

    fn color(&self) -> Color {
        random_color()
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle { center: *center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::random(width, height);
        let max_r = width.min(height) / 2; 
        let radius = rng.gen_range(5..=max_r);
        Circle { center, radius }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut dyn Displayable) {
        let xc = self.center.x;
        let yc = self.center.y;
        let r = self.radius;
        let c = self.color();

        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        display_circle_points(xc, yc, x, y, image, &c);
        while y >= x {
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
            display_circle_points(xc, yc, x, y, image, &c);
        }
    }

    fn color(&self) -> Color {
        random_color()
    }
}

fn draw_line_with_color(p1: &Point, p2: &Point, image: &mut dyn Displayable, color: Color) {
    let mut x = p1.x;
    let mut y = p1.y;
    let x2 = p2.x;
    let y2 = p2.y;

    let dx = (x2 - x).abs();
    let sx = if x < x2 { 1 } else { -1 };
    let dy = -(y2 - y).abs();
    let sy = if y < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        image.display(x, y, color.clone());
        if x == x2 && y == y2 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn display_circle_points(xc: i32, yc: i32, x: i32, y: i32, image: &mut dyn Displayable, color: &Color) {
    image.display(xc + x, yc + y, color.clone());
    image.display(xc - x, yc + y, color.clone());
    image.display(xc + x, yc - y, color.clone());
    image.display(xc - x, yc - y, color.clone());
    image.display(xc + y, yc + x, color.clone());
    image.display(xc - y, yc + x, color.clone());
    image.display(xc + y, yc - x, color.clone());
    image.display(xc - y, yc - x, color.clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct MockImage;
    impl Displayable for MockImage {
        fn display(&mut self, _x: i32, _y: i32, _color: Color) {}
    }

    #[test]
    fn test_random_point_within_bounds() {
        let width = 1000;
        let height = 800;
        for _ in 0..100 {
            let p = Point::random(width, height);
            assert!(p.x >= 0 && p.x < width, "X point is out of bounds");
            assert!(p.y >= 0 && p.y < height, "Y point is out of bounds");
        }
    }

    #[test]
    fn test_rectangle_initialization_reverse() {
        let p1 = Point::new(100, 200);
        let p2 = Point::new(50, 100);
        let rect = Rectangle::new(&p1, &p2);
        
        assert_eq!(rect.min_x, 50);
        assert_eq!(rect.max_x, 100);
        assert_eq!(rect.min_y, 100);
        assert_eq!(rect.max_y, 200);
    }

    #[test]
    fn test_line_edge_case_same_coordinate() {
        let p1 = Point::new(50, 50);
        let p2 = Point::new(50, 50);
        let line = Line::new(&p1, &p2);
        
        let mut mock = MockImage;
        line.draw(&mut mock); 
    }
}
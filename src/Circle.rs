pub mod Circle {
    use crate::Forces::Forces::Gravity;
    use crate::Rect::Rect::Rect;

    #[derive(Clone, Copy, Debug)]
    pub struct Circle {
        pub x: i16,
        pub y: i16,
        pub r: i16,
        pub vel: i16,
    }

    impl Circle {
        pub fn new(x: i16, y: i16, r: i16, vel: i16) -> Self {
            Self { x, y, r, vel }
        }
        pub fn detect_collision(&self, rect: &Rect) -> bool {
            // Calculate circle's bounding rectangle using its diameter
            let circle_diameter = (self.r * 2) as u32;
            let circle_rect = sdl2::rect::Rect::new(
                self.x as i32,
                self.y as i32,
                circle_diameter,
                circle_diameter,
            );

            // Check for overlap between the circle's bounding rectangle and the input rectangle
            let rect = rect.get_rect();
            let overlap_x =
                circle_rect.x < rect.x + rect.w && circle_rect.x + circle_rect.w > rect.x;
            let overlap_y =
                circle_rect.y < rect.y + rect.h && circle_rect.y + circle_rect.h > rect.y;

            overlap_x && overlap_y
        }
    }

    impl Gravity for Circle {
        fn fall(&mut self) {
            self.vel += 32;
            self.y += self.vel;
        }
    }
}

pub fn fill_circle(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    circle: &mut Circle::Circle,
) {
    let mut x = -circle.r; //This variable is the x coodinate that's on the left side of the radius.
    let mut y = 0; //This is the y value of the radius.
    let mut err = 2 - 2 * circle.r;
    while x < 0 {
        let points = [
            sdl2::rect::Point::new((circle.x - x).into(), (circle.y + y).into()),
            sdl2::rect::Point::new((circle.x - y).into(), (circle.y - x).into()),
            sdl2::rect::Point::new((circle.x + x).into(), (circle.y - y).into()),
            sdl2::rect::Point::new((circle.x + y).into(), (circle.y + x).into()),
        ];
        let r = err;
        if r < y {
            y += 1 * 2 + 1;
            err += y;
        }
        if r > x || err > y {
            x += 1 * 2 + 1;
            err += x;
        }
        let _ = points.iter().map(|point| canvas.draw_point(*point));
        for i in 0..4 {
            let mut min_x = 100;
            let mut max_x = 0;
            let _ = points.iter().for_each(|pt| {
                if pt.y == y.into() {
                    if min_x > points[i as usize].x {
                        min_x = points[i as usize].x;
                    }
                    if max_x < points[i as usize].x {
                        max_x = points[i as usize].x;
                    }
                }
                let _ = canvas.draw_line(points[i as usize], *pt);
            });
        }
    }
}

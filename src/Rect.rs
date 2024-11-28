pub mod Rect {
    use crate::Forces::Forces::Gravity;
    pub struct Rect {
        rect: sdl2::rect::Rect,
        vel: i32,
    }
    impl Rect {
        pub fn new(rect: sdl2::rect::Rect, vel: i32) -> Self {
            Self {
                rect: rect.clone(),
                vel,
            }
        }
        pub fn get_rect(&self) -> sdl2::rect::Rect {
            self.rect.clone()
        }
        pub fn get_all_points(&self) -> [[f64; 2]; 4] {
            [
                [
                    ((*self).rect.x as f64) + (*self).rect.w as f64,
                    (*self).rect.y as f64,
                ],
                [
                    ((*self).rect.x as f64) + (*self).rect.w as f64,
                    ((*self).rect.y as f64) + (*self).rect.h as f64,
                ],
                [
                    ((*self).rect.x as f64),
                    ((*self).rect.y as f64) + (*self).rect.h as f64,
                ],
                [(*self).rect.x as f64, (*self).rect.y as f64],
            ]
        }
        pub fn check_for_collision(&self, rect: &Rect) -> bool {
            // Helper: Vector subtraction
            fn subtract(p1: [f64; 2], p2: [f64; 2]) -> [f64; 2] {
                [p1[0] - p2[0], p1[1] - p2[1]]
            }

            // Helper: Perpendicular vector
            fn perpendicular(edge: [f64; 2]) -> [f64; 2] {
                [-edge[1], edge[0]]
            }

            // Helper: Normalize a vector
            fn normalize(v: [f64; 2]) -> [f64; 2] {
                let length = (v[0].powi(2) + v[1].powi(2)).sqrt();
                if length > 0.0 {
                    [v[0] / length, v[1] / length]
                } else {
                    [0.0, 0.0] // Handle zero-length vectors
                }
            }

            // Helper: Dot product
            fn dot(v1: [f64; 2], v2: [f64; 2]) -> f64 {
                v1[0] * v2[0] + v1[1] * v2[1]
            }

            // Helper: Project a shape onto an axis
            fn project(shape: &[[f64; 2]; 4], axis: [f64; 2]) -> (f64, f64) {
                let mut min = dot(shape[0], axis);
                let mut max = min;
                for &point in shape.iter().skip(1) {
                    let projection = dot(point, axis);
                    if projection < min {
                        min = projection;
                    }
                    if projection > max {
                        max = projection;
                    }
                }
                (min, max)
            }

            // Helper: Check if two projections overlap
            fn overlap(proj1: (f64, f64), proj2: (f64, f64)) -> bool {
                const EPSILON: f64 = 1e-9;
                proj1.1 + EPSILON >= proj2.0 && proj2.1 + EPSILON >= proj1.0
            }

            // Get the vertices of both rectangles as `f64` for precision
            let points1: [[f64; 2]; 4] = self.get_all_points().map(|p| [p[0] as f64, p[1] as f64]);
            let points2: [[f64; 2]; 4] = rect.get_all_points().map(|p| [p[0] as f64, p[1] as f64]);

            // Compute edges for both rectangles
            let edges1: Vec<[f64; 2]> = points1
                .iter()
                .zip(points1.iter().cycle().skip(1))
                .map(|(p1, p2)| subtract(*p2, *p1))
                .collect();
            let edges2: Vec<[f64; 2]> = points2
                .iter()
                .zip(points2.iter().cycle().skip(1))
                .map(|(p1, p2)| subtract(*p2, *p1))
                .collect();

            // Generate axes by normalizing perpendicular vectors of all edges
            let axes: Vec<[f64; 2]> = edges1
                .iter()
                .chain(edges2.iter())
                .map(|edge| normalize(perpendicular(*edge)))
                .collect();

            // Check projections on all axes
            for axis in axes {
                let proj1 = project(&points1, axis);
                let proj2 = project(&points2, axis);
                if !overlap(proj1, proj2) {
                    // Separating axis found, no collision
                    return false;
                }
            }

            // All projections overlap, collision detected
            true
        }

        pub fn aabb(&self, rect: sdl2::rect::Rect) -> bool {
            self.rect.x + self.rect.w > rect.x
                && self.rect.x < rect.x + rect.w
                && self.rect.y + self.rect.h > rect.y
                && self.rect.y < rect.y + rect.h
        }
    }

    impl Gravity for Rect {
        fn fall(&mut self) {
            self.vel += 32;
            self.rect.y += self.vel;
        }
    }
}

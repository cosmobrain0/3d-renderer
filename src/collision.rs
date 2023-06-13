pub mod collision {
    use crate::matrix::matrix::Vec2;

    pub trait CollisionTrait: std::fmt::Debug + Clone {
        fn collision(&self, other: &Collider, threshold: f32) -> CollisionResult;
        fn point_collision(&self, point: Vec2, threshold: f32) -> bool;
    }

    #[derive(Debug, Clone)]
    pub enum Collider {
        Line(LineCollider),
        Circle(CircleCollider),
        Triangle(TriangleCollider),
    }

    #[derive(Debug, Clone)]
    pub enum CollisionResult {
        None,
        One(Vec2),
        Many(Vec<Vec2>),
        Infinite,
    }
    impl CollisionResult {
        fn as_vec(&self) -> Option<Vec<Vec2>> {
            match &self {
                &CollisionResult::None => None,
                &CollisionResult::One(x) => Some(vec![*x]),
                &CollisionResult::Many(x) => Some(x.clone()),
                &CollisionResult::Infinite => None,
            }
        }
    }

    /// A line defined as `r = position + lambda direction`
    /// If `bounded` is true, then `lambda` must be between 0 and 1
    #[derive(Debug, Clone)]
    pub struct LineCollider {
        position: Vec2,
        direction: Vec2,
        bounded: bool,
    }
    impl LineCollider {
        pub fn new(position: Vec2, direction: Vec2, bounded: bool) -> Self {
            Self {
                position,
                direction,
                bounded,
            }
        }

        pub fn is_parallel(&self, other: &LineCollider) -> bool {
            (self.direction.y() == 0.0 && other.direction.y() == 0.0)
                || (self.direction.x() == 0.0 && other.direction.x() == 0.0)
                || (self.direction.x() / self.direction.y()
                    == other.direction.x() / other.direction.y())
        }

        /// A *bounded* line with endpoints `a` and `b`
        pub fn between_points(a: Vec2, b: Vec2) -> Self {
            Self {
                position: a,
                direction: b - a,
                bounded: true,
            }
        }

        /// An ***un**bounded* line which passes through `a` and `b`
        pub fn through_points(a: Vec2, b: Vec2) -> Self {
            Self {
                position: a,
                direction: b - a,
                bounded: false,
            }
        }
    }
    impl CollisionTrait for LineCollider {
        /// This currently doesn't consider wether or not the line is bounded
        fn point_collision(&self, point: Vec2, threshold: f32) -> bool {
            if self.direction.x() == 0.0 {
                (self.position.x() - point.x()).abs() <= threshold
            } else if self.direction.y() == 0.0 {
                (self.position.y() - point.y()).abs() <= threshold
            } else {
                let lambda_x = (point.x() - self.position.x()) / self.direction.x();
                let lambda_y = (point.y() - self.position.y()) / self.direction.y();
                (lambda_x - lambda_y).abs() / self.direction.length() <= threshold
            }
        }

        /// This currently doesn't consider wether or not the line is bounded
        fn collision(&self, other: &Collider, threshold: f32) -> CollisionResult {
            match other {
                Collider::Line(line) => {
                    if line.is_parallel(self) {
                        if line.point_collision(self.position, threshold) {
                            CollisionResult::Infinite
                        } else {
                            CollisionResult::None
                        }
                    } else {
                        // p1 + ad1 = p2 + bd2
                        // p1.x + a d1.x = p2.x + b d2.x
                        // a = (p2.x-p1.x)/d1.x + b d2.x/d1.x
                        // p1.y + a d1.y = p2.y + b d2.y
                        // p1.y + d1.y(p2.x-p1.x)/d1.x + b d2.x d1.y/d1.x = p2.y + b d2.y
                        // b (d2.x d1.y/d1.x - d2.y) = p2.y - p1.y - d1.y(p2.x-p1.x)/d1.x
                        let p1 = self.position;
                        let p2 = line.position;
                        let d1 = self.direction;
                        let d2 = line.direction;
                        let b = (p2.y() - p1.y() - d1.y() / d1.x() * (p2.x() - p1.x()))
                            / (d2.x() * d1.y() / d1.x() - d2.y());
                        CollisionResult::One(line.position + line.direction * b)
                    }
                }
                Collider::Circle(circle) => todo!(),
                Collider::Triangle(triangle) => todo!(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct CircleCollider {
        centre: Vec2,
        radius: f32,
    }
    impl CircleCollider {
        pub fn new(centre: Vec2, radius: f32) -> Self {
            Self { centre, radius }
        }
    }
    impl CollisionTrait for CircleCollider {
        fn collision(&self, other: &Collider, threshold: f32) -> CollisionResult {
            todo!()
        }

        fn point_collision(&self, point: Vec2, threshold: f32) -> bool {
            todo!()
        }
    }

    #[derive(Debug, Clone)]
    pub struct TriangleCollider {
        points: [Vec2; 3],
    }
    impl TriangleCollider {
        pub fn new(points: &[Vec2; 3]) -> Self {
            Self { points: *points }
        }
    }
    impl CollisionTrait for TriangleCollider {
        fn collision(&self, other: &Collider, threshold: f32) -> CollisionResult {
            todo!()
        }

        fn point_collision(&self, point: Vec2, threshold: f32) -> bool {
            todo!()
        }
    }
}

pub mod shapes {
    use crate::matrix::matrix::Vec3;

    pub struct Vertex {
        position: Vec3,
    }

    impl From<Vec3> for Vertex {
        fn from(value: Vec3) -> Self {
            Self { position: value }
        }
    }

    pub struct Triangle {
        points: [Vertex; 3],
    }

    impl Triangle {
        pub fn new(p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
            Self {
                points: [p1.into(), p2.into(), p3.into()],
            }
        }

        pub fn normal(&self) -> Vec3 {
            (self.points[1].position - self.points[0].position)
                .cross(&(self.points[2].position - self.points[0].position))
        }
    }
}

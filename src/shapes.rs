pub mod shapes {
    use ruscii::drawing::Pencil;

    use crate::matrix::matrix::{Matrix, Vec2, Vec3};

    #[derive(Clone, Copy, Debug)]
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

    pub struct Mesh {
        triangles: Vec<Triangle>,
        transformation: Transformation,
    }

    impl Mesh {
        pub fn new(triangles: Vec<Triangle>) -> Self {
            Self {
                triangles,
                transformation: Default::default(),
            }
        }

        pub fn draw(&self, pencil: &mut Pencil) {
            let points: Vec<_> = self
                .triangles
                .iter()
                .map(|x| x.points)
                .flat_map(|x| x.map(|x| x.position))
                .map(|x| self.transformation.transform(x))
                .map(|x| matrix![x.x(); x.y()])
                .map(|x| ruscii::spatial::Vec2 {
                    x: x.x() as i32,
                    y: x.y() as i32,
                })
                .collect::<Vec<ruscii::spatial::Vec2>>();
            let triangles: Vec<_> = points.chunks_exact(3).collect();
            for triangle in triangles {
                pencil
                    .draw_char('#', triangle[0])
                    .draw_char('#', triangle[1])
                    .draw_char('#', triangle[2]);
            }
        }
    }

    pub struct Transformation {
        translation: Vec3,
        rotation: Vec3,
        scale: Vec3,
    }

    impl Transformation {
        pub fn new(translation: Vec3, rotation: Vec3, scale: Vec3) -> Self {
            Self {
                translation,
                rotation,
                scale,
            }
        }

        pub fn translation(translation: Vec3) -> Self {
            Self {
                translation,
                rotation: matrix![0; 0; 0],
                scale: matrix![1; 1; 1],
            }
        }

        pub fn rotation(rotation: Vec3) -> Self {
            Self {
                translation: matrix![0; 0; 0],
                rotation,
                scale: matrix![1; 1; 1],
            }
        }

        pub fn scale(scale: Vec3) -> Self {
            Self {
                translation: matrix![0; 0; 0],
                rotation: matrix![0; 0; 0],
                scale,
            }
        }

        pub fn transform(&self, point: Vec3) -> Vec3 {
            (Transformation::from_rotation(self.rotation) * (point - self.translation))
                .component_mult(&self.scale)
                + self.translation
        }

        fn from_rotation(rotation: Vec3) -> Matrix<3, 3> {
            let [x, y, z] = [rotation.x(), rotation.y(), rotation.z()];

            let cos_x = x.cos();
            let cos_y = y.cos();
            let cos_z = z.cos();
            let sin_x = x.sin();
            let sin_y = y.sin();
            let sin_z = z.sin();

            let rx = matrix! {
                1, 0, 0;
                0, cos_x, -sin_x;
                0, sin_x, cos_x
            };
            let ry = matrix! {
                cos_y, 0, sin_y;
                0, 1, 0;
                -sin_y, 0, cos_y
            };
            let rz = matrix! {
                cos_z, -sin_z, 0;
                sin_z, cos_z, 0;
                0, 0, 1
            };

            rz * ry * rx
        }
    }

    impl Default for Transformation {
        fn default() -> Self {
            Self {
                translation: matrix![0; 0; 0],
                rotation: matrix![0; 0; 0],
                scale: matrix![1; 1; 1],
            }
        }
    }
}

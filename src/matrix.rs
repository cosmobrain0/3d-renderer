#[macro_export]
macro_rules! matrix {
    ($($($value: expr),+);*) => {
        crate::matrix::matrix::Matrix::new(
            [$([$($value as f32),*]),*]
        )
    };
}

pub mod matrix {
    use std::ops::{Add, Index, IndexMut, Mul, Sub};

    #[derive(Debug, Clone, Copy)]
    pub struct Matrix<const HEIGHT: usize, const WIDTH: usize> {
        values: [[f32; WIDTH]; HEIGHT],
    }

    impl<const WIDTH: usize, const HEIGHT: usize> Matrix<HEIGHT, WIDTH> {
        pub fn new(values: [[f32; WIDTH]; HEIGHT]) -> Self {
            Self { values }
        }

        pub fn from_slice(values: &[&[f32]]) -> Result<Self, ()> {
            if values.len() != HEIGHT {
                return Err(());
            }
            let mut result = [[0.0; WIDTH]; HEIGHT];
            for i in 0..HEIGHT {
                let row = values[i];
                if row.len() != WIDTH {
                    return Err(());
                } else {
                    for (j, &value) in row.iter().enumerate() {
                        result[i][j] = value;
                    }
                }
            }

            Ok(Self { values: result })
        }

        pub fn values(&self) -> &[[f32; WIDTH]; HEIGHT] {
            &self.values
        }

        pub fn component_mult(&self, rhs: &Matrix<HEIGHT, WIDTH>) -> Matrix<HEIGHT, WIDTH> {
            let mut values = [[0.0; WIDTH]; HEIGHT];
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    values[i][j] = self.values[i][j] * rhs.values[i][j];
                }
            }
            Matrix { values }
        }
    }

    /// Index as matrix[y][x]
    impl<const WIDTH: usize, const HEIGHT: usize> Index<usize> for Matrix<HEIGHT, WIDTH> {
        type Output = [f32; WIDTH];

        fn index(&self, index: usize) -> &Self::Output {
            &self.values[index]
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<usize> for Matrix<HEIGHT, WIDTH> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.values[index]
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize> Index<(usize, usize)> for Matrix<HEIGHT, WIDTH> {
        type Output = f32;
        fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
            &self.values[y][x]
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<(usize, usize)> for Matrix<HEIGHT, WIDTH> {
        fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
            &mut self.values[y][x]
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize> Add<Matrix<HEIGHT, WIDTH>> for Matrix<HEIGHT, WIDTH> {
        type Output = Matrix<HEIGHT, WIDTH>;

        fn add(self, rhs: Matrix<HEIGHT, WIDTH>) -> Self::Output {
            let values = self
                .values
                .iter()
                .enumerate()
                .map(|(x, column)| {
                    column
                        .iter()
                        .enumerate()
                        .map(|(y, value)| value + rhs[x][y])
                        .collect()
                })
                .collect::<Vec<Vec<f32>>>();
            let values = values.iter().map(|x| x.as_slice()).collect::<Vec<_>>();
            Matrix::from_slice(values.as_slice()).unwrap()
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize> Sub<Matrix<HEIGHT, WIDTH>> for Matrix<HEIGHT, WIDTH> {
        type Output = Matrix<HEIGHT, WIDTH>;

        fn sub(self, rhs: Matrix<HEIGHT, WIDTH>) -> Self::Output {
            let values = self
                .values
                .iter()
                .enumerate()
                .map(|(x, column)| {
                    column
                        .iter()
                        .enumerate()
                        .map(|(y, value)| value - rhs[x][y])
                        .collect()
                })
                .collect::<Vec<Vec<f32>>>();
            let values = values.iter().map(|x| x.as_slice()).collect::<Vec<_>>();
            Matrix::from_slice(values.as_slice()).unwrap()
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize> PartialEq<Matrix<HEIGHT, WIDTH>>
        for Matrix<HEIGHT, WIDTH>
    {
        fn eq(&self, other: &Matrix<HEIGHT, WIDTH>) -> bool {
            self.values.iter().enumerate().all(|(x, column)| {
                column
                    .iter()
                    .enumerate()
                    .all(|(y, &value)| value == other[x][y])
            })
        }
    }

    /// Allows for multiplying an MxN matrix and an NxP matrix (heightxwidth)
    impl<const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for Matrix<M, N> {
        type Output = Matrix<M, P>;

        /// TODO: make this more functional and pretty
        fn mul(self, rhs: Matrix<N, P>) -> Self::Output {
            let mut result = [[0.0; P]; M];
            for i in 0..M {
                for j in 0..P {
                    let mut sum = 0.0;
                    for k in 0..N {
                        sum += self[i][k] * rhs[k][j];
                    }
                    result[i][j] = sum;
                }
            }
            Matrix::new(result)
        }
    }

    impl<const WIDTH: usize, const HEIGHT: usize> Mul<f32> for Matrix<HEIGHT, WIDTH> {
        type Output = Matrix<HEIGHT, WIDTH>;

        fn mul(self, rhs: f32) -> Self::Output {
            let mut result = self.values;
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    result[i][j] *= rhs;
                }
            }
            Matrix::new(result)
        }
    }

    pub type Vector<const N: usize> = Matrix<N, 1>;
    pub type Vec2 = Vector<2>;
    pub type Vec3 = Vector<3>;
    pub type Vec4 = Vector<4>;

    impl<const N: usize> Vector<N> {
        pub fn as_slice(&self) -> [f32; N] {
            let mut result = [0.0; N];
            self.values
                .iter()
                .enumerate()
                .for_each(|(i, x)| result[i] = x[0]);
            result
        }

        pub fn dot(&self, rhs: &Vector<N>) -> f32 {
            self.values
                .iter()
                .enumerate()
                .fold(0.0, |acc, (i, x)| acc + x[0] * rhs[i][0])
        }

        pub fn length(&self) -> f32 {
            self.as_slice().iter().map(|x| x * x).sum::<f32>().sqrt()
        }

        pub fn sqr_length(&self) -> f32 {
            self.as_slice().iter().map(|x| x * x).sum()
        }
    }

    impl Vector<1> {
        pub fn x(&self) -> f32 {
            self[0][0]
        }
        pub fn set_x(&mut self, value: f32) {
            self[0][0] = value
        }
    }

    impl Vector<2> {
        pub fn x(&self) -> f32 {
            self[0][0]
        }
        pub fn y(&self) -> f32 {
            self[1][0]
        }

        pub fn set_x(&mut self, value: f32) {
            self[0][0] = value
        }
        pub fn set_y(&mut self, value: f32) {
            self[1][0] = value
        }
    }

    impl Vector<3> {
        pub fn x(&self) -> f32 {
            self[0][0]
        }
        pub fn y(&self) -> f32 {
            self[1][0]
        }
        pub fn z(&self) -> f32 {
            self[2][0]
        }

        pub fn set_x(&mut self, value: f32) {
            self[0][0] = value
        }
        pub fn set_y(&mut self, value: f32) {
            self[1][0] = value
        }
        pub fn set_z(&mut self, value: f32) {
            self[2][0] = value
        }
    }

    impl Vector<3> {
        pub fn cross(&self, rhs: &Vector<3>) -> Vector<3> {
            matrix! {
                self.y()*rhs.z() - self.z()*rhs.y();
                self.z()*rhs.x() - self.x()*rhs.z();
                self.x()*rhs.y() - self.y()*rhs.x()
            }
        }
    }
}

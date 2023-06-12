#[macro_use]
mod matrix;
mod shapes;

fn main() {
    println!("Hello World!!");
}

#[cfg(test)]
mod tests {
    use crate::matrix::matrix::*;

    #[test]
    fn matrix_equality() {
        vec![2, 3, 4];
        let mat1 = Matrix::<2, 2>::new([[1.0, 2.0], [3.0, 4.0]]);
        let mat2 = matrix! {1.0, 2.0; 3.0, 4.0};
        let mat3 = matrix![1, 2; 4, 3];
        assert_eq!(mat1, mat2);
        assert_ne!(mat2, mat3);
    }

    #[test]
    fn matrix_addition() {
        let mat1: Matrix<2, 3> = matrix! {
            1.0, 2.0, 3.0;
            4.0, 5.0, 6.0
        };
        let mat2: Matrix<2, 3> = matrix! {
            12.0, 13.0, 25.0;
            26.0, 14.0, 18.0
        };
        let result = mat1 + mat2;
        assert_eq!(
            result,
            matrix! {
                1.0+12.0, 2.0+13.0, 3.0+25.0;
                4.0+26.0, 5.0+14.0, 6.0+18.0
            },
        );
    }

    #[test]
    fn matrix_multiplication() {
        let mat1: Matrix<2, 3> = matrix! {
            1, 3, 4;
            5, 8, 2
        };
        let mat2: Matrix<3, 2> = matrix! {
            8, 7;
            3, 1;
            9, 2
        };
        assert_eq!(
            mat1 * mat2,
            matrix! {
                53, 18;
                82, 47
            },
        );
    }

    #[test]
    fn vector_to_slice() {
        let vector: Matrix<3, 1> = matrix! {
            1; 2; 5
        };
        assert_eq!(vector.as_slice(), [1.0, 2.0, 5.0]);
    }

    #[test]
    fn dot_product() {
        let v1: Vector<3> = matrix![1; 5; 3];
        let v2: Vector<3> = matrix![6; 3; 10];
        assert_eq!(v1.dot(&v2), 1.0 * 6.0 + 5.0 * 3.0 + 3.0 * 10.0);
    }

    #[test]
    fn cross_product() {
        assert_eq!(
            matrix![2; 3; 4].cross(&matrix![5; 6; 7]),
            matrix![-3; 6; -3]
        )
    }

    #[test]
    fn lengths() {
        let v1: Vec2 = matrix![3; 4];
        assert_eq!(v1.length(), 5.0);
        assert_eq!(v1.sqr_length(), 25.0);

        let v2: Vec3 = matrix![3; 4; 5];
        assert_eq!(v2.length(), 50.0f32.sqrt());
        assert_eq!(v2.sqr_length(), 50.0);
    }
}

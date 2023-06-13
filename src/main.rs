use ruscii::keyboard::Key;
use screen::screen::{GameState, Screen};

#[macro_use]
mod matrix;
mod collision;
mod screen;
mod shapes;

fn main() {
    let mut screen = Screen::new(State::new());
    screen.run();
}

struct State {}

impl State {
    fn new() -> Self {
        Self {}
    }
}

impl GameState for State {
    fn update(&mut self, app_state: &mut ruscii::app::State) {}

    fn draw(&self, pencil: &mut ruscii::drawing::Pencil, win_x: usize, win_y: usize) {
        pencil.draw_hline('#', ruscii::spatial::Vec2 { x: 5, y: 10 }, 8);
    }

    fn key_pressed(&mut self, key: Key, app_state: &mut ruscii::app::State) {}

    fn key_released(&mut self, key: Key, app_state: &mut ruscii::app::State) {
        match key {
            Key::Esc => app_state.stop(),
            _ => (),
        }
    }

    fn key_down(&mut self, key: Key, app_state: &mut ruscii::app::State) {}
}

#[cfg(test)]
mod tests {
    use crate::{
        collision::collision::{Collider, CollisionResult, CollisionTrait, LineCollider},
        matrix::matrix::*,
    };

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

    #[test]
    fn line_line_collision() {
        let line1 = LineCollider::new(matrix![0.99;1.2], matrix![2.5;1], false);
        let line2 = LineCollider::new(matrix![2.5; -1], matrix![1.1; -0.24], false);
        let line3 = LineCollider::new(
            matrix![0.99;1.2] + matrix![2.5;1] * 0.3,
            matrix![2.5; 1],
            false,
        );
        let line4 = LineCollider::new(matrix![0;1.2], matrix![2.5;1] * 0.2, false);

        let result = line1.collision(&Collider::Line(line2.clone()), 0.01);
        if let CollisionResult::One(x) = result {
            let distance = (x - matrix![-2.036; -0.0105]).length();
            println!("{}", distance);
            assert!(distance <= 0.02);
        } else {
            panic!(
                "{:#?} should have been a collision at (2.036, -0.0105)",
                result
            )
        }

        let result = line1.collision(&Collider::Line(line3.clone()), 0.01);
        if let CollisionResult::Infinite = result {
        } else {
            panic!("{:#?} should have been infinite collisions", result);
        }

        let result = line1.collision(&Collider::Line(line4.clone()), 0.01);
        if let CollisionResult::None = result {
        } else {
            panic!("{:#?} should have been no collisions", result);
        }

        assert!(line1.is_parallel(&line1));
        assert!(!line1.is_parallel(&line2));
    }
}

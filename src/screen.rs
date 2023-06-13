pub mod screen {
    use ruscii::{app::App, drawing::Pencil, keyboard::Key, terminal::Window};

    pub trait GameState {
        fn update(&mut self, app_state: &mut ruscii::app::State);
        fn draw(&self, pencil: &mut Pencil, win_x: usize, win_y: usize);
        fn key_pressed(&mut self, key: Key, app_state: &mut ruscii::app::State);
        fn key_released(&mut self, key: Key, app_state: &mut ruscii::app::State);
        fn key_down(&mut self, key: Key, app_state: &mut ruscii::app::State);
    }

    pub struct Screen<State: GameState> {
        window: Window,
        state: State,
    }

    impl<State: GameState> Screen<State> {
        pub fn new(state: State) -> Self {
            Self {
                window: Window::default(),
                state,
            }
        }

        pub fn run(&mut self) {
            let mut app = App::default();
            let window_size = app.window().size();

            app.run(|app_state: &mut ruscii::app::State, window: &mut Window| {
                for key_event in app_state.keyboard().last_key_events().clone() {
                    match key_event {
                        ruscii::keyboard::KeyEvent::Pressed(x) => {
                            self.state.key_pressed(x, app_state)
                        }
                        ruscii::keyboard::KeyEvent::Released(x) => {
                            self.state.key_released(x, app_state)
                        }
                    }
                }

                for key in app_state.keyboard().get_keys_down() {
                    self.state.key_down(key, app_state);
                }

                self.state.update(app_state);
                let mut pencil = Pencil::new(window.canvas_mut());
                let pencil = &mut pencil;
                self.state
                    .draw(pencil, window_size.x as usize, window_size.y as usize);
            });
        }
    }
}

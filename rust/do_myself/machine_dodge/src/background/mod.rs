use piston_window::*;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use rand::{thread_rng, Rng};
use self::object::*;

pub mod object;

const MAXINUM: u32 = 10;
const TIME_LIMIT: u64 = 2000;
const MAX_EPISODE: u32 = 100;
const BATCH_SIZE: u32 = 200;

pub struct Environment {
    reward: f64,
    episode: u32,
    obstacles: Vec<Object>,
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            reward: 0.0,
            episode: 0,
            obstacles: Vec::new(),
        }
    }
    pub fn start(&mut self, machine: &mut Object) {
        let opengls = OpenGL::V4_5;
        let mut window: PistonWindow = WindowSettings::new("machine_dodge", [400, 400])
            .opengl(opengls)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let (tx, rx) = mpsc::channel();
        let mut start: bool = true;
        let mut game_end: bool = false;
        let mut count = 0;
        let time_limit = Duration::from_millis(TIME_LIMIT);
        while let Some(e) = window.next() {
            if let Some(r) = e.render_args() {
                if start {
                    machine.set_pos(&r, Side::Center);
                    machine.random_arrow_set();
                    start = false;
                }
                if game_end {
                    self.episode += 1;
                    if self.episode > MAX_EPISODE {
                        break;
                    }
                    self.reward = 0.0;
                    start = true;
                    game_end = false;
                    count = 0;
                    self.obstacles.clear();
                    continue;
                }
                if count < MAXINUM {
                    let rng = thread_rng().gen_range(1, 4);
                    count += rng;
                    for _ in 0..rng {
                        let tx = tx.clone();
                        thread::spawn(move || {
                            let mut obstacle = Object::new(10.0, 10.0);
                            let position = match thread_rng().gen_range(0, 4) {
                                0 => Side::Up,
                                1 => Side::Right,
                                2 => Side::Down,
                                3 => Side::Left,
                                _ => {
                                    panic!("system error");
                                }
                            };
                            obstacle.set_pos(&r, position);
                            obstacle.set_speed(30.0);
                            obstacle.set_color([0.0, 0.0, 1.0, 1.0]);
                            obstacle.random_arrow_set();
                            tx.send(obstacle).unwrap();
                        });
                        let temp = match rx.recv_timeout(time_limit) {
                            Ok(result) => result,
                            Err(_) => {
                                panic!("send fail");
                            }
                        };
                        self.obstacles.push(temp);
                    }
                }
                window.draw_2d(&e, |c, g| {
                    let transform = c.transform
                        .trans(machine.current_state.0, machine.current_state.1);
                    machine.set_place(&r);
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    Rectangle::new(machine.color).draw([0.0, 0.0, machine.size.0, machine.size.1],
                                                       &c.draw_state,
                                                       transform,
                                                       g);
                    for obstacle in self.obstacles.iter_mut() {
                        let transform = c.transform
                            .trans(obstacle.current_state.0, obstacle.current_state.1);
                        obstacle.set_place(&r);
                        Rectangle::new(obstacle.color)
                            .draw([0.0, 0.0, obstacle.size.0, obstacle.size.1],
                                  &c.draw_state,
                                  transform,
                                  g);
                    }
                });
            }
            if let Some(u) = e.update_args() {
                machine.update(&u);
                for obstacle in self.obstacles.iter_mut() {
                    {
                        game_end = machine.is_hit(&obstacle);
                        if game_end {
                            break;
                        }
                    }
                    obstacle.update(&u);
                }
                self.reward += 0.01;
            }
        }
    }
}

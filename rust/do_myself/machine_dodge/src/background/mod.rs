use piston_window::*;
use std::f64::consts::PI;
use std::f64;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use rand::{thread_rng, Rng};
enum Speed {
    Go { scala: f64 },
    None,
}
enum SpawnSide {
    None,
    Center,
    Up,
    Down,
    Right,
    Left,
}
struct Arrow {
    sin: f64,
    cos: f64,
}
impl Arrow {
    fn new(theta: f64) -> Self {
        Arrow {
            sin: (PI * theta).sin(),
            cos: (PI * theta).cos(),
        }
    }
}
pub struct Object {
    background: (u32, u32),
    current_speed: Speed,
    current_state: (f64, f64),
    size: (f64, f64),
    spawn: SpawnSide,
    arrow: Arrow,
}
impl Object {
    fn new(width: f64, height: f64) -> Self {
        Object {
            background: (0, 0),
            current_state: (0.0, 0.0),
            current_speed: Speed::None,
            size: (width, height),
            spawn: SpawnSide::None,
            arrow: Arrow::new(0.0),
        }
    }
    fn set_place(&mut self, r: &RenderArgs) {
        self.background = (r.width, r.height);
    }
    fn set_speed(&mut self, scala: f64) {
        self.current_speed = Speed::Go { scala: scala };
    }
    fn set_pos(&mut self, r: &RenderArgs, pos: SpawnSide) {
        let position_x = thread_rng().gen_range(0.0, r.width as f64 - self.size.0);
        let position_y = thread_rng().gen_range(0.0, r.height as f64 - self.size.1);
        self.current_state = match pos {
            SpawnSide::Up => (position_x, 0.0),
            SpawnSide::Right => (r.width as f64 - self.size.0, position_y),
            SpawnSide::Down => (position_x, r.height as f64 - self.size.1),
            SpawnSide::Left => (0.0, position_y),
            SpawnSide::Center => (r.width as f64 / 2.0, r.height as f64 / 2.0),
            _ => panic!("system error"),
        };
        self.spawn = pos;
    }
    fn inner_set_pos(&mut self, pos: (f64, f64)) {
        self.current_state = pos;
    }
    fn random_arrow_set(&mut self) {
        let theta = match self.spawn {
            SpawnSide::Up => thread_rng().gen_range(0.0, 1.0),
            SpawnSide::Right => thread_rng().gen_range(0.5, 1.5),
            SpawnSide::Down => thread_rng().gen_range(1.0, 2.0),
            SpawnSide::Left => thread_rng().gen_range(-0.5, 0.5),
            SpawnSide::Center => thread_rng().gen_range(0.0, 2.0),
            _ => panic!("system error"),
        };
        self.arrow = Arrow::new(theta);
    }
    fn update(&mut self, args: &UpdateArgs) {
        let before_pos = self.current_state;
        match self.current_speed {
            Speed::Go { scala } => {
                self.current_state.0 += scala * self.arrow.cos * args.dt;
                self.current_state.1 += scala * self.arrow.sin * args.dt;
            }
            Speed::None => {}
        };
        if !self.is_wall() {
            self.inner_set_pos(before_pos);
        }
    }
    fn is_wall(&mut self) -> bool {
        let available_x = self.background.0 as f64 - self.size.0;
        let available_y = self.background.1 as f64 - self.size.1;
        0.0 <= self.current_state.0 && self.current_state.0 <= available_x &&
        0.0 <= self.current_state.1 && self.current_state.1 <= available_y
    }
}
const MAXINUM: u32 = 100;
const TIME_LIMIT: u64 = 2000;
pub fn ground() {
    let opengls = OpenGL::V4_5;
    let mut window: PistonWindow = WindowSettings::new("machine_dodge", [400, 400])
        .opengl(opengls)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let (tx, rx) = mpsc::channel();
    let mut start: bool = true;
    let mut count = 0;
    let mut machine = Object::new(20.0, 20.0);
    let mut obstacles: Vec<Object> = Vec::new();
    let time_limit = Duration::from_millis(TIME_LIMIT);
    while let Some(e) = window.next() {
        if let Some(r) = e.render_args() {
            let rng = thread_rng().gen_range(1, 4);
            if start {
                machine.set_pos(&r, SpawnSide::Center);
                machine.set_speed(50.0);
                machine.random_arrow_set();
                start = false;
            }
            if count < MAXINUM {
                count += rng;
                for _ in 0..rng {
                    let tx = tx.clone();
                    thread::spawn(move || {
                        let mut obstacle = Object::new(10.0, 10.0);
                        let position = match thread_rng().gen_range(0, 4) {
                            0 => SpawnSide::Up,
                            1 => SpawnSide::Right,
                            2 => SpawnSide::Down,
                            3 => SpawnSide::Left,
                            _ => {
                                panic!("system error");
                            }
                        };
                        obstacle.set_pos(&r, position);
                        obstacle.set_speed(40.0);
                        obstacle.random_arrow_set();
                        tx.send(obstacle).unwrap();
                    });
                    let temp = match rx.recv_timeout(time_limit) {
                        Ok(result) => result,
                        Err(_) => {
                            panic!("send fail");
                        }
                    };
                    obstacles.push(temp);
                }
            }
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(machine.current_state.0, machine.current_state.1);
                machine.set_place(&r);
                clear([0.0, 0.0, 0.0, 1.0], g);
                Rectangle::new([0.0, 1.0, 0.0, 1.0])
                    .draw([0.0, 0.0, machine.size.0, machine.size.1],
                          &c.draw_state,
                          transform,
                          g);
                for obstacle in obstacles.iter_mut() {
                    let transform = c.transform
                        .trans(obstacle.current_state.0, obstacle.current_state.1);
                    obstacle.set_place(&r);
                    Rectangle::new([0.0, 0.0, 1.0, 1.0])
                        .draw([0.0, 0.0, obstacle.size.0, obstacle.size.1],
                              &c.draw_state,
                              transform,
                              g);
                }
            });
        }
        if let Some(u) = e.update_args() {
            machine.update(&u);
            for obstacle in obstacles.iter_mut() {
                obstacle.update(&u);
            }
        }
    }
}

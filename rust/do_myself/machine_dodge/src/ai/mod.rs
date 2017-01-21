use std::collections::VecDeque;
use super::background;
use background::object::Object;

type State = Vec<(f64, f64)>;

pub struct ReplayBuffer {
    buffer_size: u32,
    count: u32,
    buffer: VecDeque<(State, f64, f64, f64, State)>,
}
impl ReplayBuffer {
    pub fn new(buffer_size: u32) -> Self {
        ReplayBuffer {
            buffer_size: buffer_size,
            count: 0,
            buffer: VecDeque::new(),
        }
    }
    pub fn add(&mut self, target: (State, f64, f64, f64, State)) {
        if self.count < self.buffer_size {
            self.buffer.push_back(target);
            self.count += 1;
        } else {
            self.buffer.pop_front().expect("pop front error");
            self.buffer.push_back(target);
        }
    }
    fn sample_batach(&self,
                     batch_size: u32)
                     -> (Vec<State>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<State>) {
        let mut for_count = 0;
        if self.count < batch_size {
            for_count = self.count;
        } else {
            for_count = batch_size;
        }
        let mut vec_state: Vec<State> = Vec::new();
        let mut vec_action: Vec<f64> = Vec::new();
        let mut vec_reward: Vec<f64> = Vec::new();
        let mut vec_t: Vec<f64> = Vec::new();
        let mut vec_state2: Vec<State> = Vec::new();
        for i in 0..5 {
            for k in 0..self.buffer_size {
                match i {
                    0 => vec_state.push(self.buffer.get(k as usize).unwrap().0.clone()),
                    1 => vec_action.push(self.buffer.get(k as usize).unwrap().1),
                    2 => vec_reward.push(self.buffer.get(k as usize).unwrap().2),
                    3 => vec_t.push(self.buffer.get(k as usize).unwrap().3),
                    4 => vec_state2.push(self.buffer.get(k as usize).unwrap().4.clone()),
                    _ => panic!("match error"),
                }
            }
        }
        (vec_state, vec_action, vec_reward, vec_t, vec_state2)
    }
    fn size(&self) -> u32 {
        self.buffer_size
    }
    fn clear(&mut self) {
        self.buffer.clear();
        self.count = 0;
    }
}

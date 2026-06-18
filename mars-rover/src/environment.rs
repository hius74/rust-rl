use crate::action::Action;
use crate::agent::Agent;

pub struct Environment {
    /// Максимальный размер состояния
    pub states_count: usize,
    /// Начальная позиция агента
    pub agent_start_position: usize,
    /// Награда достижения левого конца
    pub left_reward: f32,
    /// Награда достижения правого конца
    pub right_reward: f32,
}

impl Environment {
    pub fn new(states_count: usize, agent_start_position: usize, left_reward: f32, right_reward: f32) -> Self {
        Self {
            states_count,
            agent_start_position,
            left_reward,
            right_reward,
        }
    }

    pub fn run<A: Agent>(&mut self, agent: &mut A, epoch: usize) {
        let mut position = self.agent_start_position;
        if position == 0 {
            agent.done(position, self.left_reward);
            return;
        }
        if position >= self.states_count - 1 {
            agent.done(self.states_count - 1, self.right_reward);
            return;
        }
        
        let mut reward = 0.0;
        let mut action = agent.init(position, epoch);
        loop {
            match action {
                None => break,
                Some(action) => match action {
                    Action::Left => {
                        position -= 1;
                        if position == 0 {
                            reward = self.left_reward;
                            break;
                        }
                    }
                    Action::Right => {
                        position += 1;
                        if position == self.states_count - 1 {
                            reward = self.right_reward;
                            break;
                        }
                    }
                }
            }

            action = agent.step(position, 0.0); // награда только при достижении конца
        }
        agent.done(position, reward);
    }
}

#[cfg(test)]
mod tests {
    use std::slice::Iter;
    use super::*;

    struct FakeAgent<'a> {
        iter: Iter<'a, Action>,
        positions: Vec<usize>,
    }
    impl<'a> FakeAgent<'a> {
        pub fn new(actions: &'a [Action]) -> Self {
            Self {
                iter: actions.iter(),
                positions: vec![]
            }
        }
    }
    impl<'a> Agent for FakeAgent<'a> {
        fn init(&mut self, state: usize, _current_epoch: usize) -> Option<Action> {
            self.positions.push(state);
            self.iter.next().copied()
        }

        fn step(&mut self, state: usize, _reward: f32) -> Option<Action> {
            self.positions.push(state);
            self.iter.next().copied()
        }

        fn done(&mut self, state: usize, _reward: f32) {
            self.positions.push(state);
        }
    }

    #[test]
    fn loop_test() {
        let mut agent = FakeAgent::new(&[Action::Left, Action::Right]);
        let mut env = Environment::new(5, 2, 10.0, 10.0);
        env.run(&mut agent, 0);

        assert_eq!(vec![2, 1, 2, 2], agent.positions);
    }
}
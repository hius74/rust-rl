pub(crate) use crate::mars_rover::mars_action::Direction;
use crate::rl::Env;
use crate::rl::Step;

pub type MarsState = usize;

pub struct MarsEnv {
    // Максимальное число позиций. [o..max_position)
    pub max_position: MarsState,
    // Стартовая позиция ровера
    pub start_position: MarsState,
    // награда за шаг
    pub step_reward: f64,
    // Награда за достижения позиции 0
    pub left_reward: f64,
    // Награда за достижение правой позиции max_position - 1
    pub right_reward: f64,

    /// Текущая позиция rover
    position: MarsState,
}

impl MarsEnv {
    pub fn new(max_position: MarsState, start_position: MarsState, step_reward: f64, left_reward: f64, right_reward: f64) -> Self {
        assert!((1..max_position).contains(&start_position));
        MarsEnv {
            max_position,
            start_position,
            step_reward,
            left_reward,
            right_reward,
            // Вычисляемы поля будут заполнены на шаге reset
            position: 0,
        }
    }
}

impl Env for MarsEnv {
    type State = MarsState;
    type Action = Direction;

    fn reset(&mut self) -> Self::State {
        self.position = self.start_position;
        self.position
    }

    fn step(&mut self, action: &Self::Action) -> Step<Self::State> {
        match action {
            Direction::Left => self.position = self.position.saturating_sub(1),
            Direction::Right => self.position += 1,
        }

        let (state, reward, done) = match self.position {
            0 => (0, self.left_reward, true),
            p if p >= self.max_position - 1 => (self.max_position - 1, self.right_reward, true),
            p => (p, self.step_reward, false),
        };

        Step { state, reward, done }
    }
}
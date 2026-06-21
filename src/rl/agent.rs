use crate::mars_rover::{Direction, MarsState};

pub trait Agent {
    type State;
    type Action;
    fn choose_action(&mut self, state: &MarsState) -> Direction;

    /// Начало новой эпохи обучения.
    /// epoch >=0 обучение, < 0 финальный прогон
    fn init(&mut self, epoch: i64, state: &Self::State) -> Self::Action;

    /// Выбирает действие на основе текущего состояния.
    fn step(&mut self, state: &Self::State, reward: f64) -> Self::Action;

    /// Окончание эпохи
    fn done(&mut self, state: &Self::State, reward: f64);
}
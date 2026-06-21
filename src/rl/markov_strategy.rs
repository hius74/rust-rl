/// Марковский процесс.
pub trait MarkovStrategy {
    type State;
    type Action;
    
    /// Выбор лучшего действия на основе текущего состояния
    fn get_best_action(&mut self, state: &Self::State) -> Self::Action;
    
    /// Обучение: [S, A, R, S']
    fn learn(&mut self, previous_state: &Self::State, action: Self::Action, reward: f64,
             current_state: &Self::State, done: bool);
}
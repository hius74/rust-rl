use crate::action::Action;

pub trait Agent {

    /// Начало эпизода. Агент возвращает первое действие
    fn init(&mut self, state: usize, current_epoch: usize) -> Option<Action>;

    /// Промежуточный шаг агента
    fn step(&mut self, state: usize, reward: f32) -> Option<Action>;

    /// Окончание эпизода. Агент достиг финальной точки
    fn done(&mut self, state: usize, reward: f32);
}

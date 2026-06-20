use super::agent::Agent;
use super::step::Step;

pub trait Env {
    type State;
    type Action;

    /// Сбрасывает среду в начальное состояние и возвращает его.
    fn reset(&mut self) -> Self::State;

    /// Делает шаг в среде на основе переданного действия.
    fn step(&mut self, action: &Self::Action) -> Step<Self::State>;
}

/// Один эпизод обучения агента
/// Return: total reward
pub fn run_episode<E, A>(env: &mut E, agent: &mut A, epoch: i64) -> f64
where
    E: Env,
    A: Agent<State = E::State, Action = E::Action>, // Типы должны совпадать!
{
    let mut total_reward = 0.0;
    let state = env.reset();
    let mut action = agent.init(epoch, &state);
    let mut step;
    loop {
        step = env.step(&action);
        total_reward += step.reward;
        if step.done {
            break;
        }
        action = agent.step(&step.state, step.reward);
    }
    agent.done(&step.state, step.reward);

    total_reward
}
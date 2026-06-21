use rand::{rng, RngExt};
use rand::rngs::ThreadRng;
use crate::rl::{Agent, EpsilonStrategy, MarkovStrategy};

use super::MarsState;
use super::Direction;

pub struct MarsAgent<E, M> {
    epsilon_strategy: E,
    markov_strategy: M,

    last_state: MarsState,
    last_action: Direction,
    epsilon: f64,
    rng: ThreadRng,
}

impl<E, M> MarsAgent<E, M>
where
    E: EpsilonStrategy,
    M: MarkovStrategy<State = MarsState, Action = Direction>, // Фиксируем типы тут!
{
    pub fn new(epsilon_strategy: E, markov_strategy: M) -> Self {
        MarsAgent {
            epsilon_strategy,
            markov_strategy,
            last_state: 0,
            last_action: Direction::Left,
            epsilon: 0.0,
            rng: rng(),
        }
    }
}

impl<E, M> Agent for MarsAgent<E, M>
where
    E: EpsilonStrategy,
    M: MarkovStrategy<State = MarsState, Action = Direction>,
{
    type State = MarsState;
    type Action = Direction;

    fn choose_action(&mut self, state: &Self::State) -> Self::Action {
        if self.rng.random::<f64>() < self.epsilon {
            // Exploration
            Direction::from_index(self.rng.random_range(0..Direction::COUNT))
        } else {
            // Exploitation
            self.markov_strategy.get_best_action(state)
        }
    }

    fn init(&mut self, epoch: i64, state: &Self::State) -> Self::Action {
        self.last_state = *state;
        self.epsilon = self.epsilon_strategy.get_epsilon(epoch);
        self.last_action = self.choose_action(state);
        self.last_action
    }

    fn step(&mut self, state: &Self::State, reward: f64) -> Self::Action {
        // Обучение
        self.markov_strategy.learn(&self.last_state, self.last_action, reward, state, false);

        self.last_state = *state;
        self.last_action = self.choose_action(state);
        self.last_action
    }

    fn done(&mut self, state: &Self::State, reward: f64) {
        // Окончание обучение - финальный шаг обучения
        self.markov_strategy.learn(&self.last_state, self.last_action, reward, state, true);
    }
}
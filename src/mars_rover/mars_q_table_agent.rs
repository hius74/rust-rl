use rand::{rng, RngExt};
use rand::rngs::ThreadRng;

use crate::rl::{Agent, ExplorationStrategy};

use super::MarsState;
use super::Direction;
use super::q_table::QTable;

pub struct MarsQAgent<Strategy: ExplorationStrategy> {

    q_table: QTable,

    // exploration/eplotation
    epsilon_strategy: Strategy,

    // discount faction (gamma) 0..1
    discount: f64,

    last_state: MarsState,

    last_action: Direction,

    // epsilon текущей эпохи для Exploration/Exploitation
    epsilon: f64,

    // Генератор для exploration/eplotation
    rng: ThreadRng,
}

impl<Strategy: ExplorationStrategy> MarsQAgent<Strategy> {
    pub fn new(total_states: usize, epsilon_strategy: Strategy) -> Self {
        MarsQAgent {
            q_table: QTable::new(total_states, Direction::COUNT, 0.0),
            epsilon_strategy,
            discount: 0.9,
            // Поля перезапишутся после init
            last_state: 0,
            last_action: Direction::Left,
            epsilon: 0.0,
            rng: rng(),
        }
    }

    pub fn display(&self) {
        println!("{}", self.q_table);
    }

    fn choose_action(&mut self, state: &MarsState) -> Direction {
        if self.rng.random::<f64>() < self.epsilon {
            // Exploration
            Direction::from_index(self.rng.random_range(0..Direction::COUNT))
        } else {
            // Exploitation
            Direction::from_index(self.q_table.get_best_action(*state).action_idx)
        }
    }
}

impl<Strategy: ExplorationStrategy> Agent for MarsQAgent<Strategy> {
    type State = MarsState;
    type Action = Direction;

    fn init(&mut self, epoch: i64, state: &Self::State) -> Self::Action {
        self.last_state = *state;
        self.epsilon = self.epsilon_strategy.get_epsilon(epoch);
        self.last_action = self.choose_action(state);

        self.last_action
    }

    fn step(&mut self, state: &Self::State, reward: f64) -> Self::Action {
        let best_action = self.q_table.get_best_action(*state);

        // пересчитываем максимальную суммарную доходность которую получили на предыдущем шаге
        let last_reward = reward + self.discount * best_action.value;
        self.q_table[self.last_state][Direction::to_index(self.last_action)] = last_reward;

        // Выбор действия с учетом Exploration/Exploitation
        let action_idx = if self.rng.random::<f64>() < self.epsilon {
            // Exploration
            self.rng.random_range(0..Direction::COUNT)
        } else {
            // Exploitation
            best_action.action_idx
        };

        // Сохраняем данные
        self.last_state = *state;
        self.last_action = Direction::from_index(action_idx);

        self.last_action
    }

    fn done(&mut self, state: &Self::State, reward: f64) {
        // Эпизод закончился, мы достигли финальной точки
        self.q_table[*state][self.last_action.to_index()] = reward;
        self.q_table[self.last_state][self.last_action.to_index()] = self.discount * reward;
    }
}
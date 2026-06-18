use strum::{EnumCount};
use rand::{rng, RngExt};
use rand::rngs::ThreadRng;

use crate::action::Action;
use crate::agent::Agent;
use crate::exploration_exploitation::ExplorationStrategy;
use crate::q_table::{QTable, QValue};

#[derive(Debug)]
pub struct MarsAgent<Strategy: ExplorationStrategy> {
    q_table: QTable,
    // exploration/eplotation
    epsilon_strategy: Strategy,
    // discount faction (gamma) 0..1
    discount: f32,

    last_state: usize,
    last_action: QValue,

    // epsilon текущей эпохи для Exploration/Exploitation
    epsilon: f32,

    // Генератор для exploration/eplotation
    rng: ThreadRng,
}

impl<Strategy: ExplorationStrategy> MarsAgent<Strategy> {
    pub fn new(states_count: usize, epsilon_strategy: Strategy) -> Self {
        MarsAgent {
            q_table: QTable::new(states_count, Action::COUNT, 0.0),
            epsilon_strategy,
            discount: 0.9,
            // Поля перезапишутся после init
            last_state: 0,
            last_action: QValue {
                action_idx: 0,
                value: 0.0,
            },
            epsilon: 0.0,
            rng: rng(),
        }
    }

    fn random_action(&mut self, state: usize) -> QValue {
        let action_idx = self.rng.random_range(0..Action::COUNT);
        QValue{
            action_idx,
            value:  self.q_table[state][action_idx],
        }
    }

    fn choose_action(&mut self, state: usize) -> QValue {
        if self.rng.random::<f32>() < self.epsilon {
            // Exploration
            self.random_action(state)
        } else {
            // Exploitation
            self.q_table.get_best_action_index(state)
        }
    }

    pub fn print_q_table(&self) {
        println!("{}", self.q_table);
    }
}

impl<Strategy: ExplorationStrategy> Agent for MarsAgent<Strategy> {
    fn init(&mut self, state: usize, current_epoch: usize) -> Option<Action> {
        self.last_state = state;
        self.epsilon = self.epsilon_strategy.get_epsilon(current_epoch);

        self.last_action = self.choose_action(state);

        Some(Action::from_index(self.last_action.action_idx).unwrap())
    }

    fn step(&mut self, state: usize, reward: f32) -> Option<Action> {
        // Выбор действия с учетом Exploration/Exploitation
        let action = self.choose_action(state);

        // пересчитываем максимальную суммарную доходность которую получили на предыдущем шаге
        let current_max = self.q_table.get_best_action_index(state);
        let last_reward = reward + self.discount * current_max.value;
        self.q_table[self.last_state][self.last_action.action_idx] = last_reward;

        // Сохраняем данные
        self.last_state = state;
        self.last_action = action;

        Some(Action::from_index(self.last_action.action_idx).unwrap())
    }

    fn done(&mut self, state: usize, reward: f32) {
        // Эпизод закончился, мы достигли финальной точки
        self.q_table[state][self.last_action.action_idx] = reward;
        self.q_table[self.last_state][self.last_action.action_idx] = self.discount * reward;
    }
}
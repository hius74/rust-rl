use crate::mars_rover::{Direction, MarsState};
use crate::mars_rover::q_table::QTable;
use crate::rl::MarkovStrategy;

pub struct MaraQTableStrategy {
    q_table: QTable,
    discount: f64,
}

impl MaraQTableStrategy {
    pub fn new(total_states: usize, discount: f64) -> Self {
        MaraQTableStrategy {
            q_table: QTable::new(total_states, Direction::COUNT, 0.0),
            discount,
        }
    }
}

impl MarkovStrategy for MaraQTableStrategy {
    type State = MarsState;
    type Action = Direction;

    fn get_best_action(&mut self, state: &MarsState) -> Direction {
        Direction::from_index(self.q_table.get_best_action(*state).action_idx)
    }

    fn learn(&mut self, previous_state: &MarsState, action: Direction, reward: f64,
             current_state: &MarsState, done: bool) {
        let expected_cumulative_reward = if done {
            reward
        } else {
            let best_value = self.q_table.get_best_action(*current_state).value;
            reward + self.discount * best_value
        };
        self.q_table[*previous_state][action.to_index()] = expected_cumulative_reward;
    }
}
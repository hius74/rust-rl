use std::ops::{Index, IndexMut};
use std::fmt;

#[derive(Debug)]
pub struct QValue {
    pub action_idx: usize,
    pub value: f64,
}

#[derive(Debug)]
pub struct QTable {
    states_count: usize,
    actions_count: usize,
    table: Vec<f64>, // плоский массив лучше подходит для использования CPU Cache L1, L2, L3
}

impl QTable {
    pub fn new(states_count: usize, actions_count: usize, initial_value: f64) -> Self {
        Self {
            states_count,
            actions_count,
            table: vec![initial_value; states_count * actions_count],
        }
    }

    /// Поиск лучшего действия
    /// Возвращает индекс действия и его expected total cumulative reward
    pub fn get_best_action(&self, state: usize) -> QValue {
        let start = state * self.actions_count;
        let end = start + self.actions_count;
        let state_slice = &self.table[start..end];

        state_slice
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b)) // лучше подходит для сверки чисел с плавающий точкой
            .map(|(idx, val)| QValue {
                action_idx: idx,
                value: *val,
            })
            .unwrap()
    }
}

// 1. Реализуем чтение: Q[state][action]
impl Index<usize> for QTable {
    // Возвращаемым типом будет срез (slice) f32.
    // Его размер в памяти в точности равен количеству действий.
    type Output = [f64];

    fn index(&self, state: usize) -> &Self::Output {
        debug_assert!(state < self.states_count);
        let start = state * self.actions_count;
        let end = start + self.actions_count;
        // Возвращаем ссылку на "кусок" нашего плоского вектора, относящийся к этому состоянию
        &self.table[start..end]
    }
}

// 2. Реализуем запись: Q[state][action] = 0.5;
impl IndexMut<usize> for QTable {
    fn index_mut(&mut self, state: usize) -> &mut Self::Output {
        debug_assert!(state < self.states_count);
        let start = state * self.actions_count;
        let end = start + self.actions_count;
        &mut self.table[start..end]
    }
}

// Визуализация дерева
impl fmt::Display for QTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n=== Q-Table (States: {}, Actions: {}) ===", self.states_count, self.actions_count)?;
        write!(f, "State    │ ")?;
        for action in 0..self.actions_count {
            write!(f, "Act-{:<6} │ ", action)?;
        }
        writeln!(f)?;
        writeln!(f, "─────────┼{}", "────────────┼".repeat(self.actions_count))?;

        for state in 0..self.states_count {
            let row = &self[state];
            let best = self.get_best_action(state);

            write!(f, "S-{:<6} │ ", state)?;
            for (idx, val) in row.iter().enumerate() {
                if idx == best.action_idx {
                    // Подсвечиваем звездочкой лучшее действие в строке
                    write!(f, "{:<9.4}* │ ", val)?;
                } else {
                    write!(f, "{:<10.4} │ ", val)?;
                }
            }
            writeln!(f)?;
        }

        write!(f, "=======================================")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let max_states = 3;
        let max_actions = 7;
        let mut table = QTable::new(max_states, max_actions, 0.0);
        let mut count = 0.0;
        for state in 0..max_states {
            for action in 0..max_actions {
                table[state][action] = count;
                count += 1.0;
            }
        }
        println!("{table}");
        println!("{:?}", table.table);
    }
}
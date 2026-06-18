use crate::mars_agent::MarsAgent;
use crate::environment::Environment;
use crate::exploration_exploitation::LinearEpsilonStrategy;

mod environment;
mod agent;
mod action;
mod q_table;
mod exploration_exploitation;
mod mars_agent;

fn main() {
    let states_count: usize = 5;
    let agent_start_position = 2;
    let total_epochs: usize = 10;

    let epsilon = LinearEpsilonStrategy {
        start: 1.0,
        min: 0.1,    // Оставляем непредсказуемость даже в конце обучения
        total_epochs,
    };
    let mut agent = MarsAgent::new(states_count, epsilon);
    let mut env = Environment::new(states_count, agent_start_position, 10.0, 10.0);

    for epoch in 0..total_epochs {
        env.run(&mut agent, epoch);
    }
    agent.print_q_table();
}

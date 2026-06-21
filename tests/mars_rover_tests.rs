use rust_rl::rl::{run_episode, LinearEpsilonStrategy};

use rust_rl::mars_rover::MarsEnv;
use rust_rl::mars_rover::MarsAgent;
use rust_rl::mars_rover::MaraQTableStrategy;

#[test]
fn test_q_agent_5_states() {
    let total_epochs = 10;
    let total_states = 5;

    let mut env = MarsEnv::new(total_states, 2, -1.0, 10.0, 10.0);
    let epsilon_strategy = LinearEpsilonStrategy {
        start: 1.0,
        min: 0.1,
        total_epochs
    };
    let markov_strategy = MaraQTableStrategy::new(total_states, 0.9);
    let mut agent = MarsAgent::new(epsilon_strategy, markov_strategy);

    for epoch in 1..total_epochs {
        run_episode(&mut env, &mut agent, epoch);
    }

    let total_reward = run_episode(&mut env, &mut agent, -1);
    assert_eq!(total_reward, 9.0);
}
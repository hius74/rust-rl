use rust_rl::mars_rover::MarsEnv;
use rust_rl::mars_rover::MarsQAgent;
use rust_rl::rl::{run_episode, LinearEpsilonStrategy};

#[test]
fn test_q_agent_3_states() {
    let total_epochs = 20;
    let total_states = 5;

    let mut env = MarsEnv::new(total_states, 2, -1.0, 10.0, 10.0);
    let epsilon_strategy = LinearEpsilonStrategy {
        start: 1.0,
        min: 0.1,
        total_epochs
    };
    let mut agent = MarsQAgent::new(total_states, epsilon_strategy);

    for epoch in 1..total_epochs {
        run_episode(&mut env, &mut agent, epoch);
    }

    let total_reward = run_episode(&mut env, &mut agent, -1);
    agent.display();
    assert_eq!(total_reward, 9.0);
}
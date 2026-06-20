//! # Марсоход
//!
//! Есть состояния марсохода как позиция на линейной шкале 0..n
//!
//! В крайних точках есть награда.
//!
//! Марсоход может выбирать действия:
//! - влево
//! - вправо
//!
//! Необходимо определить оптимальную траекторию движения к краю с максимизацией наградой
//!
//! ## Q-Table
//!
//! ## ML - Linear regression
//!

mod mars_env;
mod mars_q_table_agent;
mod mars_action;
mod q_table;

pub use super::mars_rover::mars_action::Direction;
pub use super::mars_rover::mars_env::MarsState;
pub use super::mars_rover::mars_env::MarsEnv;
pub use super::mars_rover::mars_q_table_agent::MarsQAgent;
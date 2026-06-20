#[derive(Debug, Clone)]
pub struct Step<S> {
    pub state: S,
    pub reward: f64,
    pub done: bool,
    // pub info: HashMap<String, String>, // Опционально: для отладочной информации
}
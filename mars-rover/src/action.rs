use strum::{EnumCount, VariantNames, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantNames, EnumIter)]
pub enum Action {
    Left,
    Right,
}

impl Action {
    /// Преобразует Action в численный индекс usize
    pub fn _to_index(self) -> usize {
        self as usize
    }

    /// Безопасно преобразует индекс в Action
    pub fn from_index(index: usize) -> Option<Self> {
        Action::iter().nth(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_index_test() {
        assert_eq!(Action::_to_index(Action::Left), 0);
        assert_eq!(Action::_to_index(Action::Right), 1);
    }
    
    #[test]
    fn from_index_test() {
        assert_eq!(Action::from_index(0), Some(Action::Left));
        assert_eq!(Action::from_index(1), Some(Action::Right));
    }
}
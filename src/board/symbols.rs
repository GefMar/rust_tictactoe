use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub enum GameSymbol {
    Cross,
    Zero,
    Empty,
}


impl GameSymbol {
    pub fn to_char(&self) -> char {
        match self {
            GameSymbol::Cross => 'X',
            GameSymbol::Zero => 'O',
            GameSymbol::Empty => ' ',
        }
    }
}
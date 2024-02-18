use crate::board::symbols::GameSymbol;

pub struct Player{
    pub name: String,
    pub symbol: GameSymbol,
}

impl Player{
    pub fn new(name:String, symbol: GameSymbol) -> Self {
        return Self {
            name,
            symbol
        }
    }
}
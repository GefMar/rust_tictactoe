use std::collections::{HashMap, HashSet};
use crate::board::symbols::GameSymbol;

pub struct Board {
    pub user_steps: HashMap<(i8, i8), GameSymbol>
}

impl Board {
    pub fn new() -> Self {
        let result = Self {
            user_steps: Self::init_steps()
        };
        return result
    }

    fn init_steps() -> HashMap<(i8, i8), GameSymbol>{
        let mut result = HashMap::new();
        for x_coord in 0..3 {
            for y_coord in 0..3 {
                result.insert((x_coord, y_coord), GameSymbol::Empty);
            }
        }
        return result
    }

    pub fn set_step(&mut self, step: (i8, i8), symbol: GameSymbol) -> bool {
        match self.user_steps.get(&step) {
            Some(GameSymbol::Empty) => { self.user_steps.insert(step, symbol); true}
            _ => false
        }
    }

    pub fn check_winner(&self) -> bool {
        let mut lines = Vec::new();
        lines.push(
            (0..3).map(|y_coord| self.user_steps.get(&(y_coord, y_coord)).unwrap()
            ).collect::<HashSet<&GameSymbol>>()
        );
        lines.push(
            (0..3).map(|y_coord| self.user_steps.get(&(y_coord, 2 - y_coord)).unwrap()
            ).collect::<HashSet<&GameSymbol>>()
        );

        for x_coord in 0..3 {
            lines.push(
                (0..3).map(
                    |y_coord| self.user_steps.get(&(x_coord, y_coord)).unwrap()
                ).collect::<HashSet<&GameSymbol>>()
            );
            lines.push(
                (0..3).map(|y_coord| self.user_steps.get(&(y_coord, x_coord)).unwrap()
            ).collect::<HashSet<&GameSymbol>>()
            );
        };

        for line in lines.into_iter() {
            if self.check_winner_line(line) {return true;}
        }
        return false
        }

    fn check_winner_line(&self, line: HashSet<&GameSymbol>) -> bool {
        if line.len() == 1 && !line.contains(&GameSymbol::Empty) {
            return true;
        }
        return false
    }

}

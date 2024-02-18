use std::io;

use crate::board::board::Board;
use crate::board::symbols::GameSymbol;
use crate::player::player::Player;
use super::messages;
use super::traits::UserInterface;
pub struct TerminalInterface;


impl UserInterface for TerminalInterface {

    fn start_game(&self){
        println!("{}", messages::GAME_START)
    }
    fn end_game(&self){
        println!("{}", messages::GAME_END)
    }
    fn get_user_name(&self, game_symbol: &GameSymbol) -> String{
        let formatted_message = messages::GET_NAME_TMPL.replace("{}", &game_symbol.to_char().to_string());
        println!("{}", formatted_message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(messages::INPUT_ERROR);
        return input.trim_end().to_string()
    }
    fn get_player_step(&self, player:&Player) -> Result<(i8, i8), String>{
        let formatted_message = messages::ENTER_STEP_TMPL.replace("{}", &player.name);
        println!("{}", formatted_message);
        loop {
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("{}\nTry again\n", messages::INPUT_ERROR);
                return Err(messages::INPUT_ERROR.to_string())
            }

            let nums: Vec<Option<i8>> = input
                .split_whitespace()
                .map(|s| s.parse::<i8>().ok())
                .collect();

            match nums.as_slice() {
                [Some(x), Some(y)] if nums.len() == 2 => return Ok((*x, *y)),
                _ => {
                    println!("{}\nTry again\n", messages::INPUT_ERROR);
                    return Err(messages::ERROR_STEP.to_string())
                },
            }
        }
    }
    fn print_board(&self, board: &Board){
        let mut result = String::new();
        for x_coord in 0..3 {
            for y_coord in 0..3 {
                let symbol = board.user_steps.get(&(x_coord, y_coord)).unwrap();
                match symbol {
                    GameSymbol::Empty => {result.push_str("| – ");}
                    _ => {result.push_str(&format!("| {} ", symbol.to_char()));}
                }
            }
            result.push_str("|\n");
        }
        println!("{}", result)
    }
    fn cell_is_not_empty(&self){
        println!("{}\nTry again\n", messages::ERROR_SET_STEP)
    }
}

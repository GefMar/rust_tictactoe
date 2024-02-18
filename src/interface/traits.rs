use crate::board::board::Board;
use crate::board::symbols::GameSymbol;
use crate::player::player::Player;

pub trait UserInterface {
    fn start_game(&self);
    fn end_game(&self);
    fn get_user_name(&self, game_symbol: &GameSymbol) -> String;
    fn get_player_step(&self, player: &Player) -> Result<(i8, i8), String>;
    fn print_board(&self, board: &Board);
    fn cell_is_not_empty(&self);

}
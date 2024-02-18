use crate::board::board::Board;
use crate::board::symbols::GameSymbol;
use crate::interface::traits::UserInterface;
use crate::player::player::Player;

pub struct Game {
    board: Board,
    players: Vec<Player>,
    user_interface: Box<dyn UserInterface>,
    step_number: i8,
    player_number: usize
}

impl Game {
    pub fn new(user_interface: Box<dyn UserInterface>) -> Self {
        return  Self {
            board: Board::new(),
            players: Self::init_players(&user_interface),
            user_interface,
            step_number: 0,
            player_number: 0
        };
    }

    fn init_players(user_interface: &Box<dyn UserInterface>) -> Vec<Player> {
        let mut result: Vec<Player> = Vec::new();
        for symbol in [GameSymbol::Cross, GameSymbol::Zero] {
            result.push(Player::new(user_interface.get_user_name(&symbol), symbol));
        }
        return result

    }

    pub fn start(&mut self) {
        self.user_interface.print_board(&self.board);
        while self.step_number < 9 {
            let player = self.players.get(self.player_number).unwrap();
            let user_step = match self.user_interface.get_player_step(&player){
                    Ok(step) => step,
                    Err(_) => {
                        continue
                    }
                };
            match self.board.set_step(user_step, player.symbol){
                false => {
                    self.user_interface.cell_is_not_empty();
                    continue
                },
                _ => {}
            };
            if self.board.check_winner() {
                self.user_interface.end_game();
                return
            }
            self.user_interface.print_board(&self.board);
            self.step_number += 1;
            match self.player_number {
                0 => self.player_number = 1,
                _ => self.player_number = 0
            }
        }
        self.user_interface.end_game();
    }

}
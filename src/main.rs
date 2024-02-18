mod interface;
mod board;
mod game;
mod player;
use crate::interface::terminal::TerminalInterface;



fn main() {
    let mut game: game::Game = game::Game::new(Box::new(TerminalInterface{}));
    game.start();
}

extern crate rand;

mod card;
mod deck;
mod round;
mod game;
mod interface;
use game::*;

fn main() {
    println!("Введите имя игрока");
    let name = interface::input();
    let game = Game {
        player_name: name,
        player_balance: 100,
        dealer_balance: 100
    };

    let mut round = round::new();

    println!("player_name:  {}", game.player_name);

    round.init_turn();
    interface::show_round_info(&round);
    round.player_turn();
    interface::show_round_info(&round);
    interface::show_game_info(&game);
}

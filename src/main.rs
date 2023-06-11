use std::io;

use guess_word::Game;

fn main() {
    let game = Game::default();
    let mut guess = String::new();
    let answer = game.get_answer();

    println!("({}", answer);

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if answer == guess.trim() {
            println!("You Win!");
            break;
        } else {
            println!("Not match word...");
        }

        guess.clear();
    }
}

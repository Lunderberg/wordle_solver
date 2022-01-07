mod wordle;
use wordle::*;

fn main() -> Result<(), Error> {
    let dictionary = GameState::read_dictionary()?;
    let game_state = GameState::new(dictionary.iter(), 5);

    println!("Best guess = {:?}", game_state.best_guess());

    Ok(())
}

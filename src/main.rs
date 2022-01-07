mod wordle;
use wordle::*;

fn main() -> Result<(), Error> {
    let dictionary = wordle::read_dictionary()?;
    let game_state = GameState::<5>::new(dictionary.iter());

    let best_guess = game_state.best_guess()?;
    println!("Best guess = {:?}", best_guess);

    let clue = "_____".parse()?;

    let _next_state = game_state.after_guess(best_guess, clue);

    Ok(())
}

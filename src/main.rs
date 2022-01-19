use wordle::*;

use rand::Rng;
use structopt::StructOpt;

fn run_interactively(mut game_state: GameState<5>) -> Result<(), Error> {
    while game_state.secret.len() > 1 {
        println!("{} possibilities remaining", game_state.secret.len());

        let best_guess = game_state.best_guess()?;
        println!("Best word to guess = {}", best_guess);

        let mut line = "".to_string();
        std::io::stdin().read_line(&mut line).unwrap();
        let clue = line.trim().parse()?;

        println!("Clue received was {}", clue);
        game_state = game_state.after_guess(best_guess, clue)?;
    }

    assert_eq!(game_state.secret.len(), 1);
    println!("Winning word is {}", game_state.secret[0]);

    Ok(())
}

fn simulate_strategy<F, const N: usize>(
    game_state: &GameState<N>,
    strategy: F,
    secret_word: Word<N>,
) where
    F: FnMut(&GameState<N>) -> Word<N>,
{
    game_state
        .simulate_strategy(secret_word, strategy)
        .for_each(|res_state| {
            match &res_state {
                Ok((Some((guess, clue)), _)) => {
                    println!("Guessed: {}", guess);
                    println!("Clue: {}", clue);
                }
                _ => (),
            }
            match res_state {
                Ok((_, state)) if state.secret.len() == 0 => {
                    println!("Strategy failed, erroneously eliminated all possibilities.");
                }
                Ok((_, state)) if state.secret.len() == 1 => {
                    println!("Winner, discovered secret word {}", state.secret[0]);
                }
                Ok((_, state)) if state.secret.len() < 15 => {
                    println!("{} possible secret words remaining", state.secret.len());
                    state
                        .secret
                        .iter()
                        .for_each(|word| println!("\tPossible: {}", word))
                }
                Ok((_, state)) => {
                    println!("{} possible secret words remaining", state.secret.len());
                }
                Err(e) => println!("Error: {:?}", e),
            }
        });
}

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,

    #[structopt(short = "w", long = "secret-word")]
    secret_word: Option<String>,

    #[structopt(short = "s", long = "simulate")]
    simulate: bool,

    #[structopt(long = "allowed_word_list", default_value = "wordle")]
    word_list: String,
}

fn main() -> Result<(), Error> {
    let opt = Options::from_args();

    let game_state = if opt.word_list == "wordle" {
        GameState::<5>::from_wordle()
    } else if opt.word_list == "scrabble" {
        GameState::<5>::from_scrabble()
    } else {
        GameState::<5>::from_files(&opt.word_list, &opt.word_list)?
    };

    if opt.interactive {
        run_interactively(game_state.clone())?;
    }

    if opt.simulate {
        let secret_word: Word<5> = opt
            .secret_word
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or_else(|| {
                game_state.secret
                    [rand::thread_rng().gen_range(0..game_state.secret.len())]
            });
        let strategy = |state: &GameState<5>| state.best_guess().unwrap();
        simulate_strategy(&game_state, strategy, secret_word);
    }

    Ok(())
}

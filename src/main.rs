use wordle::*;
mod plots;

use rand::Rng;
use structopt::StructOpt;

use itertools::Itertools;

fn run_interactively<S: Strategy<N>, const N: usize>(
    strategy: &S,
    mut game_state: GameState<N>,
) -> Result<(), Error> {
    while game_state.possible_secrets.len() > 1 {
        println!(
            "{} possibilities remaining",
            game_state.possible_secrets.len()
        );

        let best_guess = strategy.make_guess(&game_state)?;
        println!("Best word to guess = {}", best_guess);

        let mut line = "".to_string();
        std::io::stdin().read_line(&mut line).unwrap();
        let clue = line.trim().parse()?;

        println!("Clue received was {}", clue);
        game_state = game_state.after_guess(best_guess, clue);
    }

    assert_eq!(game_state.possible_secrets.len(), 1);
    println!("Winning word is {}", game_state.possible_secrets[0]);

    Ok(())
}

fn simulate_strategy<S: Strategy<N>, const N: usize>(
    game_state: &GameState<N>,
    strategy: &S,
    secret_word: Word<N>,
) {
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
                Ok((_, state)) if state.possible_secrets.len() == 0 => {
                    println!("Strategy failed, erroneously eliminated all possibilities.");
                }
                Ok((_, state)) if state.possible_secrets.len() == 1 => {
                    println!("Winner, discovered secret word {}", state.possible_secrets[0]);
                }
                Ok((_, state)) if state.possible_secrets.len() < 15 => {
                    println!("{} possible secret words remaining", state.possible_secrets.len());
                    state
                        .possible_secrets
                        .iter()
                        .for_each(|word| println!("\tPossible: {}", word))
                }
                Ok((_, state)) => {
                    println!("{} possible secret words remaining", state.possible_secrets.len());
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

    #[structopt(long = "allowed-word-list", default_value = "wordle")]
    word_list: String,

    #[structopt(long = "strategy")]
    strategy: Vec<String>,

    #[structopt(long = "analysis")]
    analysis: bool,
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

    let strategy = opt
        .strategy
        .first()
        .map(|name| {
            strategy::all_strategies()
                .remove(name)
                .unwrap_or_else(|| panic!("Unknown strategy: {}", name))
        })
        .unwrap_or_else(|| Box::new(strategy::MiniMax));

    if opt.interactive {
        run_interactively(&strategy, game_state.clone())?;
    }

    if opt.simulate {
        let secret_word: Word<5> = opt
            .secret_word
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or_else(|| {
                game_state.possible_secrets[rand::thread_rng()
                    .gen_range(0..game_state.possible_secrets.len())]
            });
        simulate_strategy(&game_state, &strategy, secret_word);
    }

    if opt.analysis {
        let mut plotter = plots::WordlePlotter::new();

        let strategies: Vec<(String, Box<dyn Strategy<5>>)> =
            if opt.strategy.len() == 0 {
                strategy::all_strategies()
                    .into_iter()
                    .sorted_by_key(|(name, _strategy)| name.clone())
                    .collect()
            } else {
                let mut strategy_map = strategy::all_strategies();
                opt.strategy
                    .iter()
                    .cloned()
                    .map(|name| {
                        let strategy =
                            strategy_map.remove(&name).unwrap_or_else(|| {
                                panic!("Unknown or repeated strategy: {}", name)
                            });
                        (name, strategy)
                    })
                    .collect()
            };

        strategies.into_iter().for_each(|(name, strategy)| {
            println!("Running strategy '{}'", name);
            let paths =
                strategy.deterministic_strategy_results(game_state.clone());
            let mean_guesses = (paths.iter().map(|p| p.len()).sum::<usize>()
                as f32)
                / (paths.len() as f32);
            println!("Mean guesses: {}", mean_guesses);

            let by_num_guesses = paths.iter().into_group_map_by(|p| p.len());
            by_num_guesses
                .iter()
                .sorted_by_key(|(num, _paths)| *num)
                .for_each(|(num, paths)| {
                    println!("{} guesses to solve {} words", num, paths.len())
                });
            plotter.add_results(&name, &paths);
        });

        plotter.plot();
    }

    Ok(())
}

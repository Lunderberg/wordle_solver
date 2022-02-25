use wordle::*;
mod plots;

use itertools::Itertools;
use rand::Rng;
use structopt::StructOpt;

use std::convert::TryInto;

fn read_clue_from_stdin<const N: usize>() -> Result<Clue<N>, Error> {
    let mut line = "".to_string();
    std::io::stdin().read_line(&mut line).unwrap();
    let clue = line.trim().parse()?;
    Ok(clue)
}

fn run_multigame_interactively<
    S: MultiStrategy<N, GAMES>,
    const N: usize,
    const GAMES: usize,
>(
    strategy: &S,
    mut game_state: MultiGameState<N, GAMES>,
) -> Result<(), Error> {
    while !game_state.is_finished() {
        game_state.games.iter().enumerate().for_each(|(i, game)| {
            println!(
                "Game {} has {} possibilities remaining",
                i,
                game.possible_secrets.len()
            )
        });

        let best_guess = strategy.make_guess(&game_state)?;
        println!("Best word to guess = {}", best_guess);

        let clues: [Clue<N>; GAMES] = (0..GAMES)
            .map(|_| read_clue_from_stdin())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap();

        println!(
            "Clue received was {}",
            clues.iter().map(|clue| format!("{}", clue)).join(" ")
        );
        game_state = game_state.after_guess(best_guess, clues);
    }

    assert!(game_state.is_finished());
    println!(
        "Winning words are {}",
        game_state
            .games
            .iter()
            .map(|game| format!("{}", game.possible_secrets[0]))
            .join(" ")
    );

    Ok(())
}

fn run_interactively<S: Strategy<N>, const N: usize>(
    strategy: &S,
    mut game_state: GameState<N>,
) -> Result<(), Error> {
    while !game_state.is_finished() {
        println!(
            "{} possibilities remaining",
            game_state.possible_secrets.len()
        );

        let best_guess = strategy.make_guess(&game_state)?;
        println!("Best word to guess = {}", best_guess);

        let clue = read_clue_from_stdin()?;

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
                Ok((_, state)) if state.possible_secrets.is_empty() => {
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

    #[structopt(long = "quordle")]
    quordle: bool,
}

fn run_single(game_state: GameState<5>, opt: &Options) -> Result<(), Error> {
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
            .as_ref()
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
            if opt.strategy.is_empty() {
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

fn run_quordle(
    game_state: MultiGameState<5, 4>,
    opt: &Options,
) -> Result<(), Error> {
    let strategy = opt
        .strategy
        .first()
        .map(|name| {
            strategy::all_multi_strategies()
                .remove(name)
                .unwrap_or_else(|| panic!("Unknown strategy: {}", name))
        })
        .unwrap_or_else(|| {
            Box::new(strategy::MultiSequential::new(strategy::MiniMax))
        });

    if opt.interactive {
        run_multigame_interactively(&strategy, game_state.clone())?;
    }

    if opt.simulate {
        panic!("Simulate not implemented for quordle");
    }

    if opt.analysis {
        panic!("Analysis not implemented for quordle");
    }

    Ok(())
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

    if opt.quordle {
        run_quordle(MultiGameState::new(game_state), &opt)?;
    } else {
        run_single(game_state, &opt)?;
    }

    Ok(())
}

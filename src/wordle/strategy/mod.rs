use std::collections::HashMap;

mod traits;
pub use traits::*;

mod impls;
pub use impls::*;

mod multi_impls;
pub use multi_impls::*;

pub fn all_strategies<const N: usize>() -> HashMap<String, Box<dyn Strategy<N>>>
{
    let mut strategies: HashMap<String, Box<dyn Strategy<N>>> = HashMap::new();
    strategies.insert("minimax".to_string(), Box::new(MiniMax));
    strategies.insert("minimize-mean".to_string(), Box::new(MinimizeMean));
    strategies.insert(
        "minimize-sum-squares".to_string(),
        Box::new(MinimizeSumSquares),
    );
    strategies.insert("early-guesses".to_string(), Box::new(EarlyGuesses));
    strategies.insert(
        "alphabetical-order".to_string(),
        Box::new(AlphabeticalOrder),
    );
    strategies
}

pub fn all_multi_strategies<const N: usize, const GAMES: usize>(
) -> HashMap<String, Box<dyn MultiStrategy<N, GAMES>>> {
    let mut strategies: HashMap<String, Box<dyn MultiStrategy<N, GAMES>>> =
        HashMap::new();

    strategies.insert(
        "sequential-minimax".to_string(),
        Box::new(MultiSequential::new(MiniMax)),
    );
    strategies.insert(
        "sequential-minimize-mean".to_string(),
        Box::new(MultiSequential::new(MinimizeMean)),
    );
    strategies.insert(
        "sequential-minimize-sum-squares".to_string(),
        Box::new(MultiSequential::new(MinimizeSumSquares)),
    );
    strategies.insert(
        "sequential-early-guesses".to_string(),
        Box::new(MultiSequential::new(EarlyGuesses)),
    );
    strategies.insert(
        "sequential-alphabetical-order".to_string(),
        Box::new(MultiSequential::new(AlphabeticalOrder)),
    );

    strategies
}

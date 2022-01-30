use std::collections::HashMap;

mod traits;
pub use traits::*;

mod impls;
pub use impls::*;

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

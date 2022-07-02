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

    macro_rules! define_strategy {
        ($cls:ident) => {
            strategies.insert(stringify!($cls).to_string(), Box::new($cls));
        };
    }

    define_strategy!(MiniMax);
    define_strategy!(MinimizeMean);
    define_strategy!(MinimizeSumSquares);
    define_strategy!(EarlyGuesses);
    define_strategy!(AlphabeticalOrder);

    strategies
}

pub fn all_multi_strategies<const N: usize, const GAMES: usize>(
) -> HashMap<String, Box<dyn MultiStrategy<N, GAMES>>> {
    let mut strategies: HashMap<String, Box<dyn MultiStrategy<N, GAMES>>> =
        HashMap::new();

    macro_rules! composed_strategy {
        ($meta_cls:ident, $base_cls:ident) => {
            strategies.insert(
                concat!(stringify!($meta_cls), "-", stringify!($base_cls))
                    .to_string(),
                Box::new($meta_cls::new($base_cls)),
            );
        };
    }

    macro_rules! meta_strategy {
        ($meta_cls:ident) => {
            composed_strategy!($meta_cls, MiniMax);
            composed_strategy!($meta_cls, MinimizeMean);
            composed_strategy!($meta_cls, MinimizeSumSquares);
            composed_strategy!($meta_cls, EarlyGuesses);
            composed_strategy!($meta_cls, AlphabeticalOrder);
        };
    }

    meta_strategy!(MultiSequential);
    meta_strategy!(WorkOnWorst);
    meta_strategy!(MinimizeWorstHeuristic);

    strategies
}

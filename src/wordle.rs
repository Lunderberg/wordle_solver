use itertools::Itertools;

#[derive(Debug)]
pub enum Error {
    UnequalWordLength,
    NoDictionaryFile,
    DictionaryReadError(std::io::Error),
    NoWordsRemaining,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Tile {
    Correct,
    WrongPosition,
    NotPresentInWord,
}

pub struct GameState {
    dictionary: Vec<String>,
    secret: Vec<String>,
}

impl GameState {
    pub fn read_dictionary() -> Result<Vec<String>, Error> {
        let dictionary_path = std::env::var("DICTIONARY_PATH")
            .map_err(|_| Error::NoDictionaryFile)?;
        Ok(std::fs::read_to_string(dictionary_path)
            .map_err(|e| Error::DictionaryReadError(e))?
            .split("\n")
            .map(|s| s.to_string())
            .collect())
    }

    pub fn new<'a>(
        word_iter: impl Iterator<Item = &'a String>,
        num_letters: usize,
    ) -> GameState {
        let dictionary: Vec<_> = word_iter
            .filter(|s| s.len() == num_letters)
            .cloned()
            .collect();
        let secret = dictionary.clone();
        Self { dictionary, secret }
    }

    pub fn after_guess(
        &self,
        guess: &str,
        observed_result: &Vec<Tile>,
    ) -> Result<Self, Error> {
        let secret = self
            .secret
            .iter()
            .map(|secret| {
                compare_words(secret, guess)
                    .map(|res| (secret, &res == observed_result))
            })
            .filter_map_ok(|(secret, matches)| matches.then(|| secret.clone()))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            dictionary: self.dictionary.clone(),
            secret,
        })
    }

    pub fn best_guess(&self) -> Result<String, Error> {
        if self.secret.len() == 0 {
            return Err(Error::NoWordsRemaining);
        }
        Ok(self
            .dictionary
            .iter()
            .enumerate()
            .inspect(|(i, word)| {
                if i % 1000 == 0 {
                    println!("Word {}/{}: {}", i, self.dictionary.len(), word);
                }
            })
            .map(|(_i, word)| word)
            .max_by_key(|guess| {
                self.secret
                    .iter()
                    .map(|secret| compare_words(secret, guess).unwrap())
                    .counts()
                    .into_values()
                    .max()
                    .unwrap()
            })
            .unwrap()
            .clone())
    }
}

pub fn compare_words(
    secret_word: &str,
    guess: &str,
) -> Result<Vec<Tile>, Error> {
    if secret_word.len() != guess.len() {
        return Err(Error::UnequalWordLength);
    }

    let mut result = vec![Tile::NotPresentInWord; secret_word.len()];

    secret_word
        .chars()
        .zip(guess.chars())
        .map(|(a, b)| a == b)
        .zip(result.iter_mut())
        .filter_map(|(matching, loc)| matching.then(|| loc))
        .for_each(|loc| *loc = Tile::Correct);

    let mut counts = secret_word
        .chars()
        .zip(result.iter())
        .filter_map(|(c, res)| (*res != Tile::Correct).then(|| c))
        .counts();

    guess
        .chars()
        .zip(result.iter_mut())
        .filter(|(_guess_char, res)| **res != Tile::Correct)
        .for_each(|(guess_char, res)| {
            let remaining = counts.get_mut(&guess_char);
            if let Some(remaining) = remaining {
                if *remaining > 0 {
                    *remaining -= 1;
                    *res = Tile::WrongPosition;
                }
            }
        });

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    fn shorthand(s: &str) -> Vec<Tile> {
        s.chars()
            .map(|c| match c {
                '_' => Tile::NotPresentInWord,
                'Y' => Tile::WrongPosition,
                'G' => Tile::Correct,
                _ => panic!("Incorrect shorthand format"),
            })
            .collect()
    }

    #[test]
    fn test_compare_unequal() {
        let res = compare_words("apple", "banana");
        assert!(matches!(res, Err(Error::UnequalWordLength)));
    }

    #[rstest]
    #[case("apple", "table", shorthand("_Y_GG"))]
    #[case("farts", "ghost", shorthand("___YY"))]
    fn test_compare(
        #[case] secret: &str,
        #[case] guess: &str,
        #[case] expected: Vec<Tile>,
    ) {
        let res = compare_words(secret, guess);
        assert!(match res {
            Ok(res) => res == expected,
            _ => false,
        });
    }

    #[test]
    fn test_after_guess() {
        let secret: Vec<_> = ["apple", "table", "farts", "ghost"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let before = GameState {
            dictionary: secret.clone(),
            secret,
        };
        let after = before.after_guess("chart", &shorthand("_G__G")).unwrap();

        assert_eq!(after.secret, vec!["ghost".to_string()]);
    }
}

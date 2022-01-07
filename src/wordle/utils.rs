use super::Error;

pub fn read_dictionary() -> Result<Vec<String>, Error> {
    let dictionary_path = std::env::var("DICTIONARY_PATH")
        .map_err(|_| Error::NoDictionaryFile)?;
    Ok(std::fs::read_to_string(dictionary_path)
        .map_err(|e| Error::DictionaryReadError(e))?
        .split("\n")
        .map(|s| s.to_string())
        .collect())
}

use serde::{self, Deserialize}; 

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::country::Country;

#[derive(Deserialize, Debug)]
struct Header {}

#[derive(Deserialize, Debug)]
struct Countries {
    header: Header,
    countries: Vec<Country>
}

pub fn read_countries_from_file(path: &str) -> Result<Vec<Country>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Countries`.
    let countries: Countries = serde_json::from_reader(reader)?;

    // Return the `Country` vector, filtering out those with no capital city.
    Ok(countries.countries
            .into_iter()
            .filter( |c| c.capital_city.len() > 0 )
            .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_file() {
        let countries = read_countries_from_file("resources/countries.json").unwrap();

        assert_eq!(countries.len(), 211);
        assert_eq!(countries[0].id, "ABW");
    }
}
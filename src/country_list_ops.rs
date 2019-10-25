use std::collections::HashMap;

use crate::country::Country;
use crate::distance_calculator;

pub type CitiesByDistance = HashMap<String, Vec<(String, f64)>>;

// I want a Map where:
// - the key is the city name
// - the value is a sorted Vec of Tuples
//    - the first entry in each Tuple is another city name
//    - the second entry in each Tuple is the distance from the source city
// - the list is sorted by ascending distance
pub fn generate_lookup_map(country_list: &Vec<Country>) -> CitiesByDistance {
    let mut country_map = HashMap::new();
    for country in country_list.iter() {
        country_map.insert(country.capital_city.clone(), sorted_list_of_other_countries(&country, &country_list));
    }
    return country_map;
}

fn sorted_list_of_other_countries(country: &Country, country_list: &Vec<Country>) -> Vec<(String, f64)> {
    let mut list_of_other_countries: Vec<(String, f64)> = country_list.iter()
        .filter( |c| c.id != country.id )
        .map( |c| (c.capital_city.clone(), distance_calculator::calculate_distance(country, c)) )
        .collect();
    
    list_of_other_countries.sort_by( |a, b| a.1.partial_cmp(&b.1).unwrap() );
    return list_of_other_countries;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn gen_country_list() -> Vec<Country> { vec![
        Country { 
            id: String::from("GBR"),
            name: String::from("United Kingdom"),
            capital_city: String::from("London"),
            longitude: String::from("-0.126236"),
            latitude: String::from("51.5002")
        },
        Country { 
            id: String::from("IRE"),
            name: String::from("Ireland"),
            capital_city: String::from("Dublin"),
            longitude: String::from("-6.26749"),
            latitude: String::from("53.3441"),
        },
        Country { 
            id: String::from("USA"),
            name: String::from("United States"),
            capital_city: String::from("Washington D.C."),
            longitude: String::from("-77.032"),
            latitude: String::from("38.8895"),
        },
        Country { 
            id: String::from("FRA"),
            name: String::from("France"),
            capital_city: String::from("Paris"),
            longitude: String::from("2.35097"),
            latitude: String::from("48.8566"),
        }

    ]}

    #[test]
    fn test_sorted_list() {
        let country_list = gen_country_list();
        let sorted_list = sorted_list_of_other_countries(&country_list[0], &country_list);

        assert_eq!(3, sorted_list.len());
        assert_eq!("Paris", sorted_list[0].0);
    }
}

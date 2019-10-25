
mod country;
mod country_list_ops;
mod countries_reader;
mod distance_calculator;
// mod route_finder;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_thing() {
        let countries = countries_reader::read_countries_from_file("../countries.json").unwrap();
        let distance = distance_calculator::calculate_distance(&countries[0], &countries[1]); // Aruba to Afghanistam
        assert!( (distance - 13240.6).abs() < 1.0);

        let country_map = country_list_ops::generate_lookup_map(&countries);
        assert_eq!(211, country_map.len());
    }
}
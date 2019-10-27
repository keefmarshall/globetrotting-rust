use std::env;

mod country;
mod country_list_ops;
mod countries_reader;
mod distance_calculator;
mod route_finder;

fn main() {
    let mut target_city = "Washington D.C.";
    let mut target_range = 2000.0;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", args.join(" "));
        let sections: Vec<&str> = args[1].split('|').collect();
        if sections.len() == 2 {
            target_city = sections[1];
            target_range = sections[0].parse().unwrap();
        }
    }

    let countries = countries_reader::read_countries_from_file("resources/countries.json").unwrap();
    let country_map = country_list_ops::generate_lookup_map(&countries);
    let rf = route_finder::RouteFinder(&country_map);
    let route = rf.find_route(target_city, target_range, &vec![]);

    println!("{} {}", route.len(), route.join(","));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_thing() {
        let countries = countries_reader::read_countries_from_file("resources/countries.json").unwrap();
        let distance = distance_calculator::calculate_distance(&countries[0], &countries[1]); // Aruba to Afghanistam
        assert!( (distance - 13240.6).abs() < 1.0);

        let country_map = country_list_ops::generate_lookup_map(&countries);
        assert_eq!(211, country_map.len());

        let rf = route_finder::RouteFinder(&country_map);
        let route = rf.find_route("London", 500.0, &vec![]);
        assert_eq!(3, route.len());
        assert_eq!("Brussels", route[1]);
    }
}
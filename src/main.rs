use std::env;

mod country;
mod country_list_ops;
mod countries_reader;
mod distance_calculator;
mod route_finder;

fn main() {
    // TODO: try to use Rayon for parallelisation (will need some changes to code)
    let countries = countries_reader::read_countries_from_file("resources/countries.json").unwrap();
    let city_map = country_list_ops::generate_lookup_map(&countries);

    let mut target_city = "Washington D.C.";
    let mut target_range = 2000.0;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", args.join(" "));
        let sections: Vec<&str> = args[1].split('|').collect();
        if sections.len() == 2 {
            let target_country = sections[1];
            target_city = &countries.iter().find(|c| c.name == target_country).unwrap().capital_city;
            target_range = sections[0].parse().unwrap();
        }
    }

    let rf = route_finder::RouteFinder(&city_map);
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

        let city_map = country_list_ops::generate_lookup_map(&countries);
        assert_eq!(211, city_map.len());

        let rf = route_finder::RouteFinder(&city_map);
        let route = rf.find_route("London", 500.0, &vec![]);
        assert_eq!(3, route.len());
        assert_eq!("Brussels", route[1]);
    }
}
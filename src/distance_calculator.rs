use crate::country::Country;

const R: f64 = 6371.0;

fn string_to_radians(val: &str) -> f64 {
    let f: f64 = val.parse().unwrap();
    f.to_radians()
}

pub fn calculate_distance(country1: &Country, country2: &Country) -> f64 {
    let lat1 = string_to_radians(&country1.latitude);
    let lat2 = string_to_radians(&country2.latitude);
    let long1: f64 = country1.longitude.parse().unwrap();
    let long2: f64 = country2.longitude.parse().unwrap();
    let long_delta = (long2 - long1).to_radians();

    ( (lat1.sin() * lat2.sin()) + (lat1.cos() * lat2.cos() * long_delta.cos()) ).acos() * R
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_distance() {
        let country1 = Country { 
            id: String::from("abc"),
            name: String::from("United Kingdom"),
            capital_city: String::from("London"),
            longitude: String::from("-0.126236"),
            latitude: String::from("51.5002")
        };
        let country2 = Country { 
            id: String::from("abc"),
            name: String::from("Ireland"),
            capital_city: String::from("Dublin"),
            longitude: String::from("-6.26749"),
            latitude: String::from("53.3441"),
        };

        let distance = calculate_distance(&country1, &country2);
        assert!( (distance - 463.0).abs() < 1.0);
    }
}
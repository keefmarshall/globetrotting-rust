use crate::country_list_ops::*;

pub struct RouteFinder<'a>(pub &'a CitiesByDistance);

impl RouteFinder<'_> {
    pub fn find_route(&self, city: &str, range: f64, visited: &Vec<String>) -> Vec<String> {
        let mut route = vec![String::from(city)];
        let mut longest_length = 0;
        let mut longest: Vec<String> = vec![];

        let mut new_visited = visited.clone();
        new_visited.push(String::from(city));
        for (other, distance) in &self.0[city] {
            if distance > &range {
                break;
            }

            if !visited.contains(other) {
                let next_route = self.find_route(&other, range - distance, &new_visited);
                if next_route.len() > longest_length {
                    longest_length = next_route.len();
                    longest = next_route;
                }
            }
        }

        if longest_length > 0 {
            route.append(&mut longest);
        }

        return route;
    }
}
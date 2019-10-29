use rayon::prelude::*;
use crate::country_list_ops::*;

pub struct RouteFinder<'a>(pub &'a CitiesByDistance);

impl RouteFinder<'_> {
    pub fn find_route(&self, city: &str, range: f64, visited: &Vec<String>) -> Vec<String> {
        let mut route = vec![String::from(city)];
        // let mut longest_length = 0;
        // let mut longest: Vec<String> = vec![];

        let mut new_visited = visited.clone();
        new_visited.push(String::from(city));

        let mut others: Vec<(&str, f64)> = vec![];
        for (other, distance) in &self.0[city] {
            if distance > &range {
                break;
            }

            if !visited.contains(other) {
                others.push((other, range - distance));
            }
        }

        if others.len() > 0 {
            let mut longest = others.par_iter()
                .map( |(o,r)| self.find_route(&o, *r, &new_visited) )
                .max_by( |a,b| a.len().cmp(&b.len()) )
                .unwrap();
            route.append(&mut longest);
        }

        // for (other, new_range) in others {
        //     let next_route = self.find_route(&other, new_range, &new_visited);
        //     if next_route.len() > longest_length {
        //         longest_length = next_route.len();
        //         longest = next_route;
        //     }
        // }

        // if longest_length > 0 {
        //     route.append(&mut longest);
        // }

        return route;
    }
}
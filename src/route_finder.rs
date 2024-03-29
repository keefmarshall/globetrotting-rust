use rayon::prelude::*;
use crate::country_list_ops::*;

pub struct RouteFinder<'a>(pub &'a CitiesByDistance);

impl RouteFinder<'_> {
    pub fn find_route(&self, city: &str, range: f64, visited: &Vec<&str>) -> Vec<String> {
        let mut others: Vec<(&str, f64)> = vec![];
        for (other, distance) in &self.0[city] {
            if distance > &range {
                break;
            }

            let other_ref: &str = &other;
            if !visited.contains(&other_ref) {
                others.push((other, range - distance));
            }
        }

        let mut route = vec![String::from(city)];
        if others.len() > 0 {
            let mut new_visited = visited.clone();
            new_visited.push(city);

            let mut longest = others.par_iter()
                .map( |(o,r)| self.find_route(o, *r, &new_visited) )
                .max_by( |a,b| a.len().cmp(&b.len()) )
                .unwrap();
                
            route.append(&mut longest);
        }

        return route;
    }
}
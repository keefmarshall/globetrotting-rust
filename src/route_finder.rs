use crate::country_list_ops::*;

pub struct RouteFinder<'a>(&'a CitiesByDistance);

impl RouteFinder<'_> {
    fn find_route(&self, city: &str, range: f64) -> Vec<String> {
        let mut route = vec![city];
        let mut longest_length = 0;
        let mut longest: Vec<String>;

        for other in self.0[city] {

        }

        return route;
    }
}
use std::{cmp::Ordering, collections::HashSet};


#[derive(Debug)]
struct WorldDistance {
    world: String,
    distance: usize,
}

impl PartialEq for WorldDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for WorldDistance {}

impl PartialOrd for WorldDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WorldDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}




fn closest_point(points: &Vec<String>, point: &str) -> String {
    let mut distances: Vec<WorldDistance> = points.iter()
        .map(|s| WorldDistance {
            world: s.clone(),
            distance: hamming_distance(s, point),
        })
        .collect();

    distances.sort();
    let closest_point = distances.first().unwrap().world.clone();
    return closest_point;
}

pub fn distance_set_point(points: &Vec<String>, point: &str) -> usize {
    let mut distances: Vec<WorldDistance> = points.iter()
        .map(|s| WorldDistance {
            world: s.clone(),
            distance: hamming_distance(s, point),
        })
        .collect();

    distances.sort();
    let closest_point = distances.first().unwrap().world.clone();
    return hamming_distance(&closest_point, point);
}

pub fn distance_set_set(set1: &Vec<String>, set2: &Vec<String>)-> usize{
    let mut d = 0;
    set1.iter().for_each(|ele| d = d+distance_set_point(set2, ele));
    set2.iter().for_each(|ele| d = d+distance_set_point(set1, ele));
    return d;
}

pub fn hamming_distance(s1: &str, s2: &str) -> usize {
    // Create sets of characters for each string
    let set1: HashSet<_> = s1.chars().collect();
    let set2: HashSet<_> = s2.chars().collect();

    // Count the number of characters unique to each set
    let unique_chars_s1 = set1.difference(&set2).count();
    let unique_chars_s2 = set2.difference(&set1).count();

    // Return the total count of unique characters
    let result = unique_chars_s1 + unique_chars_s2;
    return result;
}


pub fn test() {
    let points = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let target = "apricot";

    let closest = distance_set_point(&points, target);
    println!("The closest point to {} is {}", target, closest);
}


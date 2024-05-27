use std::{cmp::Ordering, collections::HashSet};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ModelDistance {
    model: Vec<String>,
    distance: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorldDistance {
    world: String,
    distance: usize,
}

pub fn create_order_worlds(set: &Vec<String>, world:&str) -> Vec<WorldDistance>{
    let mut order: Vec<WorldDistance> = set.iter()
        .map(|s| WorldDistance {
            world: s.clone(),
            distance: hamming_distance(&s, world),
        })
    .collect();
    order.sort();
    return order;
}

// Given a set of sets, and a set, return the order.
pub fn create_order_models(set_of_set_of_worlds: &Vec<Vec<String>>, set_of_worlds: &Vec<String>) -> Vec<ModelDistance>{
    let mut order: Vec<ModelDistance> = set_of_set_of_worlds.iter()
        .map(|s| ModelDistance {
            model: s.clone(),
            distance: distance_set_to_set(&s, set_of_worlds),
        })
    .collect();
    order.sort();
    return order;
}

fn closest_model(set_of_set_of_worlds: &Vec<Vec<String>>, set_of_worlds: &Vec<String>) -> Vec<String>{
    let order = create_order_models(set_of_set_of_worlds, set_of_worlds);
    let closest_model = order.first().unwrap().model.clone();
    return closest_model;
}

fn distance_sets_to_set(set_of_set_of_worlds: &Vec<Vec<String>>, set_of_worlds: &Vec<String>) -> usize{
    let order = create_order_models(set_of_set_of_worlds, set_of_worlds);
    let closest_model = order.first().unwrap().model.clone();
    return distance_set_to_set(&closest_model, set_of_worlds);
}

// Given a set of world and a world, return the closest world
fn closest_world(points: &Vec<String>, world: &str) -> String {
    let order = create_order_worlds(points, world);
    let closest_world = order.first().unwrap().world.clone();
    return closest_world;
}

// Given a set of world and a world, return the distance to the closest point
pub fn distance_set_to_world(set_of_worlds: &Vec<String>, world: &str) -> usize {
    let closest_world = closest_world(set_of_worlds, world);
    return hamming_distance(&closest_world, world);
}

pub fn distance_set_to_set(set1: &Vec<String>, set2: &Vec<String>)-> usize{
    let mut d = 0;
    set1.iter().for_each(|ele| d = d+distance_set_to_world(set2, ele));
    set2.iter().for_each(|ele| d = d+distance_set_to_world(set1, ele));
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

    let closest = distance_set_to_world(&points, target);
    println!("The closest point to {} is {}", target, closest);
}


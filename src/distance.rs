use std::{cmp::Ordering, collections::HashSet, fmt};
use crate::semantic::S5PointedModel;


#[derive(Debug)]
pub struct Lexicographic{
    pub distance_model: usize,
    pub distance_world: usize
}

impl fmt::Display for Lexicographic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{},{}>", self.distance_model, self.distance_world)
    }
}

impl PartialEq for Lexicographic {
    fn eq(&self, other: &Self) -> bool {
        self.distance_model == other.distance_model && self.distance_world == other.distance_world
    }
}

impl Eq for Lexicographic {}

impl PartialOrd for Lexicographic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Lexicographic {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.distance_model.cmp(&other.distance_model) {
            Ordering::Equal => self.distance_world.cmp(&other.distance_world),
            other => other,
        }
    }
}

#[derive(Debug)]
pub struct PointedModelDistance {
    pub distance: Lexicographic,
    pub pointed_model: S5PointedModel,
}

impl PartialEq for PointedModelDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for PointedModelDistance {}

impl PartialOrd for PointedModelDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointedModelDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.distance.cmp(&other.distance);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModelDistance {
    distance: usize,
    model: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorldDistance {
    distance: usize,
    world: String,
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

pub fn create_order_pointed(set: &Vec<S5PointedModel>, reference: &S5PointedModel) -> Vec<PointedModelDistance>{
    let mut order: Vec<PointedModelDistance> = set.iter()
        .map(|s|{
            let r = reference;
            let p = PointedModelDistance {
                pointed_model: s.clone(),
                distance: distance_pointed_to_pointed(s, r)
            };
            return p;
        })
    .collect();
    order.sort();
    return order;
}

pub fn closest_pointed(set: &Vec<S5PointedModel>, reference: &S5PointedModel) -> S5PointedModel {
    let order = create_order_pointed(set, reference);
    let closest_pointed = order.first().unwrap().pointed_model.clone();
    return closest_pointed;
}

pub fn distance_to_closest(set: &Vec<S5PointedModel>, reference: &S5PointedModel) -> Lexicographic{
    let closest_pointed = closest_pointed(set, reference);
    return distance_pointed_to_pointed(&closest_pointed, reference);
}

pub fn get_set_of_closest_pointed_model(set: &Vec<S5PointedModel>, reference: &S5PointedModel) -> Vec<S5PointedModel> {
    let d = distance_to_closest(set, reference);
    let result = set.iter()
        .filter(|x| distance_pointed_to_pointed(x,reference) == d)
        .map(|y| y.clone())
        .collect();
    return result;
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
    let closest_model = closest_model(set_of_set_of_worlds, set_of_worlds);
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

pub fn distance_pointed_to_pointed(pointed1: &S5PointedModel, pointed2: &S5PointedModel) -> Lexicographic{
    let m1 =  pointed1.clone().model;
    let w1 =  pointed1.clone().world;
    let m2 =  pointed2.clone().model;
    let w2 =  pointed2.clone().world;

    let distance_model = distance_set_to_set(&m1, &m2);
    let distance_world = hamming_distance(&w1, &w2);
    let lexi = Lexicographic {distance_model, distance_world};
    return lexi
}


// Given two sets, returns the set of the closest set to s1. 
pub fn get_revision_models(base: &Vec<S5PointedModel>, input: &Vec<S5PointedModel>) -> Vec<S5PointedModel> {
    let mut min_d = Lexicographic {distance_model : 99999999, distance_world : 999999999};
    let r:Vec<S5PointedModel> = input.clone();
    r.clone().iter()
        .for_each(|p1| {
            let d = distance_to_closest(base, p1);
            if d < min_d {
                min_d = d;
            }
        });
    
    let mut result  = r.iter()
        .filter(|x| distance_to_closest(base,x) == min_d)
        .map(|x| x.clone())
        .collect::<Vec<S5PointedModel>>();

    // REMOVE DUPLICATES. vvvvvvvvvvvvvvvvvvvvvvv
    let mut seen = HashSet::new();
    result.retain(|item| seen.insert(item.clone()));
    // REMOVE DUPLCIATES ^^^^^^^^^^^^^^^^^^^^^^^^
    
    return result;
        
}

pub fn get_base_closest_set(base: &Vec<S5PointedModel>, input: &Vec<S5PointedModel>) -> Vec<S5PointedModel>{
    let mut result = Vec::new();
    input.iter().for_each(|x| result.push(closest_pointed(base, x)));
    // REMOVE DUPLICATES. vvvvvvvvvvvvvvvvvvvvvvv
    let mut seen = HashSet::new();
    result.retain(|item| seen.insert(item.clone()));
    // REMOVE DUPLCIATES ^^^^^^^^^^^^^^^^^^^^^^^^
    return result;
    
}


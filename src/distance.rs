use core::f64;
use std::{cmp::{self, Ordering}, collections::HashSet, fmt::{self, write}};
use ordered_float::OrderedFloat;

use crate::{semantic::S5PointedModel, utils::{difference, intersection, symmetric_difference}, Valuation};


#[derive(Debug, Clone, Hash)]
pub struct Lexicographic{
    pub distance_model: OrderedFloat<f64>,
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

#[derive(Debug,Clone)]
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

impl fmt::Display for PointedModelDistance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", &self.distance);
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct ModelDistance {
    distance: OrderedFloat<f64>,
    model: HashSet<Valuation>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorldDistance {
    distance: usize,
    world: String,
}

pub fn create_w_order(world_set: &HashSet<Valuation>, world:&str) -> Vec<WorldDistance>{
    let mut order: Vec<WorldDistance> = world_set.iter()
        .map(|s| WorldDistance {
            world: s.clone(),
            distance: hamming_distance(&s, world),
        })
    .collect();
    return order;
}

// Given a set of sets, and a set, return the order.
pub fn create_m_order(model_set: &Vec<HashSet<Valuation>>, world_set: &HashSet<Valuation>) -> Vec<ModelDistance>{
    let mut order: Vec<ModelDistance> = model_set.iter()
        .map(|s| ModelDistance {
            model: s.clone(),
            distance: distance_model_to_model(&s, world_set),
        })
    .collect();
    return order;
}

// Create an order over a set of s5 pointed model.
pub fn create_pm_order(pm_set: &Vec<S5PointedModel>, pointed_model: &S5PointedModel) -> Vec<PointedModelDistance>{
    let mut order: Vec<PointedModelDistance> = pm_set.iter()
        .map(|s|{
            let r = pointed_model;
            let p = PointedModelDistance {
                pointed_model: s.clone(),
                distance: distance_pointed_to_pointed(s, r)
            };
            return p;
        })
    .collect();
    return order;
}


pub fn get_pm_at_distance(pm_set: &Vec<PointedModelDistance>, distance: Lexicographic) -> Vec<PointedModelDistance>{
    return pm_set.iter()
        .filter(|&x| x.distance == distance)
        .cloned()
        .collect();
}


// Min Distance from a pointed model to a set of pointed model. Which is equal to 
pub fn min_distance(set: &Vec<S5PointedModel>, reference: &S5PointedModel) -> Lexicographic{
    let closest_pointed = closest_pointed_model(set, reference);
    return distance_pointed_to_pointed(&closest_pointed, reference);
}

// Set of minimal pointed models 
pub fn minimal_set_of_pointed_model(set: &Vec<S5PointedModel>, reference: &S5PointedModel) -> Vec<S5PointedModel> {
    let d = min_distance(set, reference);
    let result = set.iter()
        .filter(|x| distance_pointed_to_pointed(x,reference) == d)
        .map(|y| y.clone())
        .collect();
    return result;
}

// Return the closest world
pub fn closest_world(points: &HashSet<Valuation>, world: &str) -> String {
    let order = create_w_order(points, world);
    let closest_world = order.first().unwrap().world.clone();
    return closest_world;
}


// Return the closest pointed model.
pub fn closest_pointed_model(set: &Vec<S5PointedModel>, reference: &S5PointedModel) -> S5PointedModel {
    let order = create_pm_order(set, reference);
    let closest_pointed = order.first().unwrap().pointed_model.clone();
    return closest_pointed;
}


// Return the closest model.
fn closest_model(set_models: &Vec<HashSet<Valuation>>, set_of_worlds: &HashSet<Valuation>) -> HashSet<Valuation>{
    let order = create_m_order(set_models, set_of_worlds);
    let closest_model = order.first().unwrap().model.clone();
    return closest_model;
}


// Return the distance from a model to the closest model from a set of model.
pub fn distance_set_models_to_model(set_models: &Vec<HashSet<Valuation>>, model: &HashSet<Valuation>) -> OrderedFloat<f64>{
    let closest_model = closest_model(set_models, model);
    return distance_model_to_model(&closest_model, model);
}


// Given a set of world and a world, return the distance to the closest point
pub fn distance_model_to_world(set_of_worlds: &HashSet<Valuation>, world: &str) -> usize {
    let closest_world = closest_world(set_of_worlds, world);
    return hamming_distance(&closest_world, world);
}

fn directed_hausdorff_distance(vec1: &HashSet<Valuation>, vec2: &HashSet<Valuation>) -> usize {
    let mut v = Vec::new();
    vec1.iter()
        .for_each(|s1|{
            vec2.iter()
                .for_each(|s2| v.push(hamming_distance(s1, s2)))
        });

    return v.iter()
        .max()
        .unwrap().clone();
}

fn hausdorff_distance(vec1: &HashSet<Valuation>, vec2: &HashSet<Valuation>) -> usize {
    cmp::max(
        directed_hausdorff_distance(vec1, vec2),
        directed_hausdorff_distance(vec2, vec1),
    )
}

// Return distance between models
pub fn distance_model_to_model(model1: &HashSet<Valuation>, model2: &HashSet<Valuation>)-> OrderedFloat<f64>{
    let mut d = OrderedFloat(0.0);
    let drastic = false;
    let hausdorf = false;
    if hausdorf {
        let dis = hausdorff_distance(model1, model2) as f64;
        let d = OrderedFloat(dis);
        return d
    }
    if drastic{
        if model1 != model2{ //A=0
            let penalty = intersection(model1, model2).len() == 0;
            let m1 = difference(model1,model2);
            let m2 = difference(model2,model1);
            let symdiff = symmetric_difference(model1, model2);
            let sym = OrderedFloat(symdiff.len() as f64);
            let mut dis = OrderedFloat(0.0);
            m2.iter().for_each(|ele| {
                dis = dis + (distance_model_to_world(&model1, ele) as f64); 
            });
            m1.iter().for_each(|ele| {
                dis = dis + (distance_model_to_world(&model2, ele) as f64); 
            });
            dis = dis/sym;
            if penalty {
                d = dis+10000.0;
            }else{
                d = dis;
            }
            // println!(">>>>");
            // println!("model1 = {:?}, model2={:?}", model1,model2);
            // println!("dis = {:?}", dis);
            // println!("sym = {:?}", sym);
            // println!("d = {:?}", d);
        }
    }else{
        model1.iter().for_each(|ele| d = d+(distance_model_to_world(model2, ele) as f64));
        model2.iter().for_each(|ele| d = d+(distance_model_to_world(model1, ele) as f64));
    }
    return d;
}

// // Return the hamnming distance between two worlds.
pub fn hamming_distance(world1: &str, world2: &str) -> usize {
    // Create sets of characters for each string
    let set1: HashSet<_> = world1.chars().collect();
    let set2: HashSet<_> = world2.chars().collect();

    // Count the number of characters unique to each set
    let unique_chars_s1 = set1.difference(&set2).count();
    let unique_chars_s2 = set2.difference(&set1).count();

    // Return the total count of unique characters
    let result = unique_chars_s1 + unique_chars_s2;
    return result;
}

// Return the distance between two pointed models
pub fn distance_pointed_to_pointed(pointed1: &S5PointedModel, pointed2: &S5PointedModel) -> Lexicographic{
    let m1 =  pointed1.clone().model;
    let w1 =  pointed1.clone().world;
    let m2 =  pointed2.clone().model;
    let w2 =  pointed2.clone().world;

    let distance_model = distance_model_to_model(&m1, &m2);
    let distance_world = hamming_distance(&w1, &w2);
    let lexi = Lexicographic {distance_model, distance_world};
    return lexi
}


// Given two sets, returns the set of the closest set to s1. 
pub fn closest_set_pointed(base: &Vec<S5PointedModel>, input: &Vec<S5PointedModel>) -> Vec<S5PointedModel> {
    let mut min_d = Lexicographic {distance_model : OrderedFloat(99999999.9), distance_world : 999999999};
    let r:Vec<S5PointedModel> = input.clone();
    r.clone().iter()
        .for_each(|p1| {
            let d = min_distance(base, p1);
            if d < min_d {
                min_d = d;
            }
        });
    
    let mut result  = r.iter()
        .filter(|x| min_distance(base,x) == min_d)
        .map(|x| x.clone())
        .collect::<Vec<S5PointedModel>>();

    return result;
}

pub fn get_base_closest_set(base: &Vec<S5PointedModel>, input: &Vec<S5PointedModel>) -> Vec<S5PointedModel>{
    let mut result = Vec::new();
    input.iter().for_each(|x| result.push(closest_pointed_model(base, x)));
    return result;
}



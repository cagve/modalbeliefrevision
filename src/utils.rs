use std::collections::HashSet;

use crate::semantic::*;
use itertools::Itertools;
use s5rust::modal::ModalFormula;

pub fn generate_propset_from_atoms(atoms:String) -> String {
    let prop_set: Vec<char> = atoms.chars().collect();
    let string: String = prop_set.into_iter().collect();
    return string;
}

pub fn generate_propset(n: usize) -> String {
    let mut prop_set: Vec<char> = Vec::with_capacity(n);

    for i in 0..n { // start at p
        prop_set.push((i as u8 + b'p') as char);
    } 
    
    let string: String = prop_set.into_iter().collect();
    return string;
}

pub fn generate_universe(propset: String) -> Vec<String>{
    let elements: Vec<char> = propset.chars().collect();
    let mut combinations = Vec::new();

    // Generar todas las combinaciones posibles
    for i in 1..=elements.len() {
        for combo in elements.iter().combinations(i) {
            let valuation: String = combo.into_iter().map(|i| i.to_string()).collect::<String>();
            combinations.push(valuation);
        }
    }
    combinations.push("".to_string());
    return combinations;
}

pub fn generate_all_poss_model(universe:&Vec<String>) -> Vec<Vec<String>>{
    let powerset = universe.iter()
        .map(|s| s.to_string())
        .powerset()
        .map(|subset| subset.into_iter().collect())
        .collect();
    return powerset;
}

pub fn generate_all_poss_pointed(universe:&Vec<String>) -> Vec<S5PointedModel>{
    let mut s5_pointed_models:Vec<S5PointedModel> = Vec::new();
    let powerset = generate_all_poss_model(universe);
    powerset.iter()
        .for_each(|set|{
            for world in set.iter() {
                let m = S5PointedModel {model:set.to_vec(), world:world.to_string()};
                s5_pointed_models.push(m);
            }
        });
    return s5_pointed_models
}

pub fn vec_symmdiff(v1: &Vec<String>, v2: &Vec<String>) -> Vec<String> {
    let s1: HashSet<String> = v1.iter().cloned().collect();
    let s2: HashSet<String> = v2.iter().cloned().collect();
    
    let intersection = s1.symmetric_difference(&s2).into_iter().cloned().collect();
    return intersection;
}

pub fn vec_difference(v1: &Vec<S5PointedModel>, v2: &Vec<S5PointedModel>) -> Vec<S5PointedModel> {
    let s1: HashSet<S5PointedModel> = v1.iter().cloned().collect();
    let s2: HashSet<S5PointedModel> = v2.iter().cloned().collect();
    (&s1 - &s2).iter().cloned().collect()
}

pub fn vec_difference_str(v1: &Vec<String>, v2: &Vec<String>) -> Vec<String> {
    let s1: HashSet<String> = v1.iter().cloned().collect();
    let s2: HashSet<String> = v2.iter().cloned().collect();
    (&s1 - &s2).iter().cloned().collect()
}



pub fn vec_intersection(v1: &Vec<String>, v2: &Vec<String>) -> Vec<String> {
    let s1: HashSet<String> = v1.iter().cloned().collect();
    let s2: HashSet<String> = v2.iter().cloned().collect();

    let intersection = s1.intersection(&s2).into_iter().cloned().collect();
    return intersection;
}

pub fn vec_union(v1: &Vec<String>, v2: &Vec<String>) -> Vec<String> {
    let s1: HashSet<String> = v1.iter().cloned().collect();
    let s2: HashSet<String> = v2.iter().cloned().collect();

    let union = s1.union(&s2).into_iter().cloned().collect();
    return union;
}
 
pub fn subset(v1: &Vec<String>, v2: &Vec<String>) -> bool{ //v1 is a subset of v2.
    let result = v1.iter().all(|item| v2.contains(item));
    return result;
}


pub fn remove_duplicates<T>(vec: Vec<T>) -> Vec<T>
where
    T: std::hash::Hash + Eq + Clone,
{
    
    // Convert Vec<T> to HashSet<T> to remove duplicates, then back to Vec<T>
    let unique_elements: HashSet<_> = vec.into_iter().collect();
    unique_elements.into_iter().collect()
}

pub fn get_atoms_from_fromula(f: &ModalFormula) -> Vec<String>  {
    return f.get_atoms().iter().map(|x| x.to_string()).collect();
}

pub fn get_universe_from_formula(f: &ModalFormula) -> Vec<String> {
    let atoms = get_atoms_from_fromula(f);
    let atomsstr = String::from_iter(atoms);
    return generate_universe(atomsstr);
}

pub fn power_set<T: Clone>(a: &Vec<T>) -> Vec<Vec<T>> {
    a.iter().fold(vec![vec![]], |mut p, x| {
        let i = p.clone().into_iter()
            .map(|mut s| {s.push(x.clone()); s});
        p.extend(i); p})
}


pub fn contains_b_in_a(a: Vec<Vec<String>>, b: Vec<String>) -> bool {
    // Convert `b` to a HashSet
    let b_set: HashSet<_> = b.into_iter().collect();

    // Check if `b_set` is equal to any `Vec<String>` in `a`, converted to a HashSet
    a.iter().any(|vec| {
        let a_set: HashSet<_> = vec.iter().cloned().collect();
        a_set == b_set
    })
}


pub fn same_models(mut vec1: Vec<Vec<String>>, mut vec2: Vec<Vec<String>>) -> bool {
    for m in &mut vec1 {
        m.into_iter().for_each(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();  // Sort the characters in the string
            *s = chars.into_iter().collect();  // Rebuild the string from the sorted characters
        });
        m.sort();
    }


    for n in &mut vec2 {
        n.into_iter().for_each(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();  // Sort the characters in the string
            *s = chars.into_iter().collect();  // Rebuild the string from the sorted characters
        });
        n.sort();
    }

    let mut vec1_sorted = vec1;
    let mut vec2_sorted = vec2;
    
    vec1_sorted.sort();
    vec2_sorted.sort();
    
    vec1_sorted == vec2_sorted
}

pub fn projection(vec1: &Vec<String>, P:&String ) -> Vec<String>{
    let mut hs = HashSet::new();

    // Iteramos sobre cada palabra en W
    for w in vec1 {
        // Verificamos si la palabra tiene algún carácter en común con P
        if w.chars().any(|c| P.contains(c)) {
            hs.insert(w.clone());
        }
    }

    let result = hs.into_iter().collect();
    return result;
}


pub fn same_vecs5model(vec1: Vec<S5PointedModel>, vec2: Vec<S5PointedModel>) -> bool{
    if vec1.len() != vec2.len(){
        return false
    }

    for m in vec1.clone() {
        if !s5_contains(m, vec2.clone()){
            return false;
        }
    }

    for m in vec2 {
        if !s5_contains(m, vec1.clone()){
            return false;
        }
    }
    return true
}

pub fn same_s5model(s51: &S5PointedModel, s52: &S5PointedModel) -> bool{
    let model_1 = s51.clone().model;
    let model_2 = s52.clone().model;
    let world_1 = s51.clone().world;
    let world_2 = s52.clone().world;

    let f1 = same_model(model_1, model_2);
    let f2 = same_world(world_1, world_2);
    return f1 && f2;
}

pub fn same_model(W: Vec<String>, U: Vec<String>) -> bool {
    // Convertimos cada String a Vec<char>, ordenamos y comparamos.
    let mut v1_chars: Vec<Vec<char>> = W.iter().map(|s| {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        chars
    }).collect();

    let mut v2_chars: Vec<Vec<char>> = U.iter().map(|s| {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        chars
    }).collect();

    // Ordenamos los vectores de caracteres.
    v1_chars.sort();
    v2_chars.sort();

    // Comparamos los vectores resultantes.
    v1_chars == v2_chars
}


pub fn same_world(w1: String, w2: String) -> bool {
    let mut chars1: Vec<char> = w1.chars().collect();
    let mut chars2: Vec<char> = w2.chars().collect();
    chars1.sort();
    chars2.sort();
    chars1 == chars2
}

pub fn s5_contains(s5:S5PointedModel, vecs5:Vec<S5PointedModel>) -> bool{
    for m in vecs5 {
        if same_s5model(&s5, &m){
            return true
        }
    }
    return false;
}


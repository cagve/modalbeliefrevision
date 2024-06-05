use crate::semantic::*;
use itertools::Itertools;

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

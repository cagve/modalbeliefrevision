use ::distance::hamming;
use itertools::Itertools;
use s5rust::formula::Tree;
use s5rust::parser::build_formula;
use s5rust::modal::*;
use s5rust::prop::PropBinary::*;

use crate::distance::*;
mod distance;

#[derive(Debug, Clone)]
struct S5PointedModel {
    set: Vec<String>,
    world: String
}

fn generate_propset(n: usize) -> String {
    let mut prop_set: Vec<char> = Vec::with_capacity(n);

    for i in 0..n {
        prop_set.push((i as u8 + b'a') as char);
    } 
    
    let string: String = prop_set.into_iter().collect();
    return string;
}

fn generate_universe(propset: String) -> Vec<String>{
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

fn generate_all_poss_set(universe:&Vec<String>) -> Vec<Vec<String>>{
    let powerset = universe.iter()
        .map(|s| s.to_string())
        .powerset()
        .map(|subset| subset.into_iter().collect())
        .collect();
    return powerset;
}

fn generate_all_poss_pointed_set(universe:&Vec<String>) -> Vec<S5PointedModel>{
    let mut s5_pointed_models:Vec<S5PointedModel> = Vec::new();
    let powerset = generate_all_poss_set(universe);
    powerset.iter()
        .for_each(|set|{
            for world in set.iter() {
                let m = S5PointedModel {set:set.to_vec(), world:world.to_string()};
                s5_pointed_models.push(m);
            }
        });
    return s5_pointed_models
}


fn check_pointed_model(formula: ModalFormula, m: &S5PointedModel) -> bool{
    match formula.tree {
        Tree::Binary { conn, left, right } => {
            let term_left = build_formula(&left.to_string()).unwrap();
            let term_right = build_formula(&right.to_string()).unwrap();
            match conn {
                Iff => {
                    // println!("Iff");
                    return check_pointed_model(term_left, m) == check_pointed_model(term_right, m);
                },
                Implies => {
                    // println!("If");
                    return !(check_pointed_model(term_left, m) && !check_pointed_model(term_right, m));
                },
                And => {
                    // println!("And");
                    return check_pointed_model(term_left, m) && check_pointed_model(term_right, m);
                },
                Or => {
                    // println!("Or");
                    return check_pointed_model(term_left, m) || check_pointed_model(term_right, m);
                }
            }
        }
        Tree::Unary { conn, next } => {
            let term = build_formula(&next.to_string()).unwrap();
            match conn {
                s5rust::modal::ModalUnary::Not => {
                    // println!("Not");
                    return !check_pointed_model(term, m);
                },
                s5rust::modal::ModalUnary::Box => {
                    // println!("Box");
                    let mut flag = true;
                    let mut i = 0;
                    let set = m.set.clone();
                    while flag && i<set.len(){
                        let val = m.set.get(i).unwrap().clone();
                        let u = S5PointedModel {set: set.clone(), world: val.to_owned()};
                        flag = check_pointed_model(term.clone(), &u);
                        i = i+1;
                    };
                    return flag;
                },
                s5rust::modal::ModalUnary::Diamond => {
                    // println!("Diamond");
                    let mut flag = false;
                    let mut i = 0;
                    let set = m.set.clone();
                    while !flag {
                        let val = m.set.get(i).unwrap().clone();
                        let u = S5PointedModel {set: set.clone(), world: val.to_owned()};
                        flag = check_pointed_model(term.clone(), &u);
                        i = i+1;
                    }
                    return flag
                }
            }
        }
        Tree::Atom(_) => {
            return m.world.contains(&formula.to_string())
        }
    }
}

fn get_models(formula: ModalFormula, universe:Vec<String>) -> Vec<S5PointedModel>{
    let s5_pointed_models = generate_all_poss_pointed_set(&universe);
    let mut s5_filtered:Vec<S5PointedModel> = Vec::new();
    s5_pointed_models.iter()
        .for_each(|pointed| {
            let f = formula.clone();
            if check_pointed_model(f, pointed){
                s5_filtered.push(pointed.clone());
            }
        });
    return s5_filtered
}

fn main() {
    let prop_set = generate_propset(2);
    let universe = generate_universe(prop_set.clone());
    let mut formula: ModalFormula = build_formula("box a").unwrap();
    let pointed_set = generate_all_poss_pointed_set(&universe);
    let world = "a".to_string();
    let order = create_order_worlds(&universe, &world);
    println!("universe = {:?}", universe);
    println!("world = {:?}", world);
    println!("order = {:?}", order);

}

use std::fmt;

use std::fs::File;
use std::io::Write;

use s5rust::formula::Tree;
use s5rust::parser::build_formula;
use s5rust::modal::*;
use s5rust::prop::PropBinary::*;
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct S5PointedModel {
    pub model: Vec<String>,
    pub world: String
}

impl S5PointedModel{
    pub fn render(&self){
        let mut dot_content = String::new();
        dot_content.push_str("digraph G {\n");
        for node in &self.model {
            if node.clone() == self.world {
                dot_content.push_str(&format!("    \"{}\" [shape=doublecircle];\n", self.world));
            }else{
                dot_content.push_str(&format!("    \"{}\" [shape=circle];\n", node));
            }
        }
        dot_content.push_str("}\n");
        println!("dot_content = {:?}", dot_content);

        let mut file = File::create("hola.dot").expect("Unable to create file");
        file.write_all(dot_content.as_bytes());
    } 
}


impl fmt::Display for S5PointedModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut model_clone:Vec<String> = Vec::new();
        let mut world_clone = String::new();
        self.model.iter()
            .for_each(|world| {
                    if world == ""  {
                        model_clone.push("∅".to_string());
                    }else{
                        model_clone.push(world.to_string());
                    }
                });

        if self.world == ""  {
            world_clone = "∅".to_string();
        }else{
            world_clone = self.world.clone();
        }
        write!(f, "W: {:?}, w: {}", model_clone, world_clone)
    }
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

pub fn generate_all_poss_set(universe:&Vec<String>) -> Vec<Vec<String>>{
    let powerset = universe.iter()
        .map(|s| s.to_string())
        .powerset()
        .map(|subset| subset.into_iter().collect())
        .collect();
    return powerset;
}

pub fn generate_all_poss_pointed_set(universe:&Vec<String>) -> Vec<S5PointedModel>{
    let mut s5_pointed_models:Vec<S5PointedModel> = Vec::new();
    let powerset = generate_all_poss_set(universe);
    powerset.iter()
        .for_each(|set|{
            for world in set.iter() {
                let m = S5PointedModel {model:set.to_vec(), world:world.to_string()};
                s5_pointed_models.push(m);
            }
        });
    return s5_pointed_models
}


pub fn check_pointed_model(formula: &ModalFormula, m: &S5PointedModel) -> bool{
    match &formula.tree {
        Tree::Binary { conn, left, right } => {
            let term_left = build_formula(&left.to_string()).unwrap();
            let term_right = build_formula(&right.to_string()).unwrap();
            match conn {
                Iff => {
                    // println!("Iff");
                    return check_pointed_model(&term_left, m) == check_pointed_model(&term_right, m);
                },
                Implies => {
                    // println!("If");
                    return !(check_pointed_model(&term_left, m) && !check_pointed_model(&term_right, m));
                },
                And => {
                    // println!("And");
                    return check_pointed_model(&term_left, m) && check_pointed_model(&term_right, m);
                },
                Or => {
                    // println!("Or");
                    return check_pointed_model(&term_left, m) || check_pointed_model(&term_right, m);
                }
            }
        }
        Tree::Unary { conn, next } => {
            let term = build_formula(&next.to_string()).unwrap();
            match conn {
                s5rust::modal::ModalUnary::Not => {
                    // println!("Not");
                    return !check_pointed_model(&term, m);
                },
                s5rust::modal::ModalUnary::Box => {
                    // println!("Box");
                    let mut flag = true;
                    let mut i = 0;
                    let set = m.model.clone();
                    while flag && i<set.len(){
                        let val = m.model.get(i).unwrap().clone();
                        let u = S5PointedModel {model: set.clone(), world: val.to_owned()};
                        flag = check_pointed_model(&term.clone(), &u);
                        i = i+1;
                    };
                    return flag;
                },
                s5rust::modal::ModalUnary::Diamond => {
                    // println!("Diamond");
                    let mut flag = false;
                    let mut i = 0;
                    let set = m.model.clone();
                    while !flag && i<set.len() {
                        let val = m.model.get(i).unwrap().clone();
                        let u = S5PointedModel {model: set.clone(), world: val.to_owned()};
                        flag = check_pointed_model(&term.clone(), &u);
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

pub fn get_models(formula: ModalFormula, universe:Vec<String>) -> Vec<S5PointedModel>{
    let s5_pointed_models = generate_all_poss_pointed_set(&universe);
    let mut s5_filtered:Vec<S5PointedModel> = Vec::new();
    s5_pointed_models.iter()
        .for_each(|pointed| {
            let f = formula.clone();
            if check_pointed_model(&f, pointed){
                s5_filtered.push(pointed.clone());
            }
        });
    return s5_filtered
}

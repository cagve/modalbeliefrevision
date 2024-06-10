use std::collections::HashSet;
use std::fmt;

use std::fs::File;
use std::io::Write;

use s5rust::formula::Tree;
use s5rust::parser::build_formula;
use s5rust::modal::*;
use s5rust::prop::PropBinary::*;
use crate::utils::*;
use crate::distance::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct S5PointedModel {
    pub model: Vec<String>,
    pub world: String
}

impl S5PointedModel{
    pub fn render(&self, file:&str) -> std::io::Result<()> {
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

        let mut file = File::create(file).expect("Error on file.");
        let _ = file.write_all(dot_content.as_bytes())?;
        Ok(())
    } 

    pub fn print_order(&self, set:Vec<S5PointedModel>){
        let order = pointed_model_order(&set, &self);
        let mut order_distances:Vec<Lexicographic> = order.clone().iter().map(|x| x.distance.clone()).collect();
        // REMOVE DUPLICATES. vvvvvvvvvvvvvvvvvvvvvvv
        let mut seen = HashSet::new();
        order_distances.retain(|item| seen.insert(item.clone()));
        // REMOVE DUPLCIATES ^^^^^^^^^^^^^^^^^^^^^^^^
        
        println!("Order over {} ", self);
        for distance in order_distances{
            let current_d = distance.clone(); 
            let v = get_pointed_model_at_distance(&order, current_d);
            println!("> D={}", distance);
            for model in v{
                println!("  | Model = {}", model.pointed_model);

            }
        };
    }
}


impl fmt::Display for S5PointedModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut model_clone:Vec<String> = Vec::new();
        let mut world_clone: String = String::new();
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
    let s5_pointed_models = generate_all_poss_pointed(&universe);
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

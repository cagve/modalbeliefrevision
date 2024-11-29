use std::collections::HashSet;
use std::fmt;

use std::fmt::write;
use std::fs::File;
use std::io::Write;

use s5rust::formula::Tree;
use s5rust::parser::build_formula;
use s5rust::modal::*;
use s5rust::prop::PropBinary::*;
use s5rust::prop::PropFormula;
use crate::utils::*;
use crate::distance::*;

#[derive(Debug, Clone, Eq, Ord, PartialOrd, Hash)]
pub struct S5PointedModel {
    pub model: Vec<String>,
    pub world: String
}

impl S5PointedModel{
    pub fn get_atoms(&self) -> String {
        let mut atoms = Vec::new();
        self.model.iter()
            .for_each(|x| {
                if x != ""{
                    atoms.push(x.clone())
                }
            });
        return remove_duplicates(atoms).concat().to_string();
    }

    pub fn new(model: Vec<String>, world: String) -> Self {
        let mut s5 = S5PointedModel { model, world };
        s5.sort();
        return s5;
    }

    pub fn sort(&mut self) {
        // Sorting string in models.
        self.model = self.model.iter()
            .map(|valuation| {
                let new_valuation = sort_string(valuation.to_string());
                return new_valuation;
            })
        .collect();
        self.model.sort();
        self.world = sort_string(self.world.clone());
    }


    pub fn to_latex(&self) -> String{
        let mut n = 0;
        let mut tab = String::new();
        tab.push_str("$\\{");
        self.model.iter().for_each(|x|{
            if n != 0{
                tab.push_str(",");
            }
            n = n+1;
            let mut world_1 = x.clone();
            let mut world_2 = self.world.clone();
            if x.clone() == "" {
                world_1 = "\\emptyset".to_string();
            }
            if self.world.clone() == "" {
                world_2 = "\\emptyset".to_string();
            }
            if world_1 == world_2.clone() {
                tab.push_str(&format!("\\underline{{{}}}", world_1).to_string());
            }else{
                tab.push_str(&format!("{}", world_1).to_string());
            }
        });
        tab.push_str("$\\}");
        return tab
    }
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
        let order = create_pm_order(&set, &self);
        let mut order_distances:Vec<Lexicographic> = order.clone().iter().map(|x| x.distance.clone()).collect();
        // REMOVE DUPLICATES. vvvvvvvvvvvvvvvvvvvvvvv
        let mut seen = HashSet::new();
        order_distances.retain(|item| seen.insert(item.clone()));
        // REMOVE DUPLCIATES ^^^^^^^^^^^^^^^^^^^^^^^^
        
        println!("Order over {} ", self);
        for distance in order_distances{
            let current_d = distance.clone(); 
            let v = get_pm_at_distance(&order, current_d);
            println!("> D={}", distance);
            for model in v{
                println!("  | Model = {}", model.pointed_model);

            }
        };
    }
}


impl PartialEq for S5PointedModel {
    fn eq(&self, other: &Self) -> bool {
        let mut s1 = self.clone();
        let mut s2 = other.clone();
        s1.sort();
        s2.sort();

        let f1 = s1.model == s2.model;
        let f2 = s1.world == s2.world;
        return f1 && f2;
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


pub fn check_prop_model(formula: &PropFormula, m: &String) -> bool{
    match &formula.tree {
        Tree::Binary { conn, left, right } => {
            let term_left = build_formula(&left.to_string()).unwrap();
            let term_right = build_formula(&right.to_string()).unwrap();
            match conn {
                Iff => {
                    // println!("Iff");
                    return check_prop_model(&term_left, m) == check_prop_model(&term_right, m);
                },
                Implies => {
                    // println!("If");
                    return !(check_prop_model(&term_left, m) && !check_prop_model(&term_right, m));
                },
                And => {
                    // println!("And");
                    return check_prop_model(&term_left, m) && check_prop_model(&term_right, m);
                },
                Or => {
                    // println!("Or");
                    return check_prop_model(&term_left, m) || check_prop_model(&term_right, m);
                }
            }
        }
        Tree::Unary { conn, next } => {
            let term = build_formula(&next.to_string()).unwrap();
            match conn {
                s5rust::prop::PropUnary::Not => {
                    // println!("Not");
                    return !check_prop_model(&term, m);
                }

            }
        }
        Tree::Atom(_) => {
            return m.contains(&formula.to_string())
        }
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

pub fn get_prop_models(formula:PropFormula, universe: &Vec<String>) -> Vec<String> {
    let mut models:Vec<String> = Vec::new();
    universe.iter()
        .for_each(|val| {
            if check_prop_model(&formula, val){
                models.push(val.clone());
            }
        });
    return models; }


pub fn get_mmodels(formula: ModalFormula, universe:Vec<String>) -> Vec<Vec<String>>{
    let s5_pointed_models = get_models(formula, universe);
    let mut models = Vec::new();
    s5_pointed_models.iter().for_each(|pm|{
        models.push(pm.clone().model);
    });
    // REMOVE DUPLICATES. vvvvvvvvvvvvvvvvvvvvvvv
    let mut seen = HashSet::new();
    models.retain(|item| seen.insert(item.clone()));
    // REMOVE DUPLCIATES ^^^^^^^^^^^^^^^^^^^^^^^^
    return models;
}


pub fn get_models(formula: ModalFormula, universe:Vec<String>) -> Vec<S5PointedModel>{
    let mut s5_pointed_models = generate_all_poss_pointed(&universe);
    s5_pointed_models.sort();
    let mut s5_filtered:Vec<S5PointedModel> = Vec::new();
    s5_pointed_models.iter()
        .for_each(|pointed| {
            let f = formula.clone();
            if check_pointed_model(&f, pointed){
                s5_filtered.push(pointed.clone());
            }
        });
    s5_filtered.sort();
    return s5_filtered
}




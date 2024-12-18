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

pub type Valuation = String;


#[derive(Eq, PartialEq, Clone, Debug,)]
pub struct S5PointedModel {
    pub model: HashSet<Valuation>,
    pub world: Valuation
}

impl S5PointedModel{
    //Función que devuelve todos los átomos (caracteres) que aparecen en el modelo
    pub fn atoms(&self) ->String {
        let mut atoms = String::new();
        for valuation in &self.model {
            atoms = atoms + valuation;
        }
        return atoms;
    }

    pub fn eq(&self, other: &Self) -> bool {
        //order model 
        let sorted_self_model:HashSet<Valuation> = self.model.iter()
            .map(|x|{
                let mut chars: Vec<char> = x.chars().collect();
                chars.sort();
                let sorted_string: String = chars.into_iter().collect(); 
                return sorted_string;
        })
        .collect();
        let sorted_other_model:HashSet<Valuation> = other.model.iter()
            .map(|x|{
                let mut chars: Vec<char> = x.chars().collect();
                chars.sort();
                let sorted_string: String = chars.into_iter().collect(); 
                return sorted_string;
        })
        .collect();

        let mut self_world_char:Vec<char>= self.world.chars().collect();
        self_world_char.sort();
        let sorted_self_world:Valuation = self_world_char.into_iter().collect();

        let mut other_world_char:Vec<char>= other.world.chars().collect();
        other_world_char.sort();
        let sorted_other_world:Valuation = other_world_char.into_iter().collect();


        return sorted_self_world == sorted_other_world && sorted_other_model == sorted_self_model;
    }

    pub fn new(model_values: HashSet<Valuation>, world: Valuation) -> Self {
        let _ = model_values.iter()
            .map(|x| {
                let mut chars: Vec<char> = x.chars().collect();
                chars.sort_by(|a, b| a.to_lowercase().cmp(b.to_lowercase()));
                let str:Valuation = chars.into_iter().collect();
                return str;
            })
        .collect::<HashSet<Valuation>>();

        let mut chars: Vec<char> = world.chars().collect();
        chars.sort_by(|a, b| a.to_lowercase().cmp(b.to_lowercase()));
        let world_value:Valuation = chars.into_iter().collect();

        S5PointedModel {
            model:model_values,
            world:world_value,
        }
    }

    pub fn to_latex(&self) -> String{
        todo!();
        // let mut n = 0;
        // let mut tab = String::new();
        // tab.push_str("$\\{");
        // self.model.iter().for_each(|x|{
        //     if n != 0{
        //         tab.push_str(",");
        //     }
        //     n = n+1;
        //     let mut world_1 = x.clone();
        //     let mut world_2 = self.world.clone();
        //     if x.clone() == "" {
        //         world_1 = "\\emptyset".to_string();
        //     }
        //     if self.world.clone() == "" {
        //         world_2 = "\\emptyset".to_string();
        //     }
        //     if world_1 == world_2.clone() {
        //         tab.push_str(&format!("\\underline{{{}}}", world_1).to_string());
        //     }else{
        //         tab.push_str(&format!("{}", world_1).to_string());
        //     }
        // });
        // tab.push_str("$\\}");
        // return tab
    }
    pub fn render(&self, file:&str) -> std::io::Result<()> {
        todo!();
        // let mut dot_content = String::new();;
        // dot_content.push_str("digraph G {\n");
        // for node in &self.model {
        //     if node.clone() == self.world {
        //         dot_content.push_str(&format!("    \"{}\" [shape=doublecircle];\n", self.world));
        //     }else{
        //         dot_content.push_str(&format!("    \"{}\" [shape=circle];\n", node));
        //     }
        // }
        // dot_content.push_str("}\n");
        // println!("dot_content = {:?}", dot_content);
        //
        // let mut file = File::create(file).expect("Error on file.");
        // let _ = file.write_all(dot_content.as_bytes())?;
        // Ok(())
    } 

    pub fn print_order(&self, set:Vec<S5PointedModel>){
        todo!()
        // let order = create_pm_order(&set, &self);
        // let mut order_distances:Vec<Lexicographic> = order.clone().iter().map(|x| x.distance.clone()).collect();
        // // REMOVE DUPLICATES. vvvvvvvvvvvvvvvvvvvvvvv
        // let mut seen = HashSet::new();
        // order_distances.retain(|item| seen.insert(item.clone()));
        // // REMOVE DUPLCIATES ^^^^^^^^^^^^^^^^^^^^^^^^
        // 
        // println!("Order over {} ", self);
        // for distance in order_distances{
        //     let current_d = distance.clone(); 
        //     let v = get_pm_at_distance(&order, current_d);
        //     println!("> D={}", distance);
        //     for model in v{
        //         println!("  | Model = {}", model.pointed_model);
        //
        //     }
        // };
    }
    pub fn contained(&self, set:&Vec<S5PointedModel>) -> bool{
        let mut flag = false;
        for s5model in set{
            if s5model.clone().eq(&self.clone()){
                flag = true;
                break;
            }
        }
        return flag;
    }
}



impl fmt::Display for S5PointedModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Clonamos el conjunto de mundos (`model`) y el mundo actual (`world`)
        let model_clone = self.model.clone();
        let world_clone = self.world.clone();
        write!(f, "W: {:?}, w: {}", model_clone, world_clone)
    }
}



pub fn check_pointed_model(formula: &ModalFormula, s5: &S5PointedModel) -> bool{
    match &formula.tree {
        Tree::Binary { conn, left, right } => {
            let term_left = build_formula(&left.to_string()).unwrap();
            let term_right = build_formula(&right.to_string()).unwrap();
            match conn {
                Iff => {
                    // println!("Iff");
                    return check_pointed_model(&term_left, s5) == check_pointed_model(&term_right, s5);
                },
                Implies => {
                    // println!("If");
                    return !(check_pointed_model(&term_left, s5) && !check_pointed_model(&term_right, s5));
                },
                And => {
                    // println!("And");
                    return check_pointed_model(&term_left, s5) && check_pointed_model(&term_right, s5);
                },
                Or => {
                    // println!("Or");
                    return check_pointed_model(&term_left, s5) || check_pointed_model(&term_right, s5);
                }
            }
        }
        Tree::Unary { conn, next } => {
            let term = build_formula(&next.to_string()).unwrap();
            match conn {
                s5rust::modal::ModalUnary::Not => {
                    // println!("Not");
                    return !check_pointed_model(&term, s5);
                },
                s5rust::modal::ModalUnary::Box => {
                    // println!("Box");
                    let mut flag = true;
                    let mut i = 0;
                    let set = s5.model.clone();
                    let mut iter = set.iter();
                    while flag {
                        if let Some(item) = iter.next() {
                            let val = item;
                            let u = S5PointedModel {model: set.clone(), world: val.to_owned()};
                            flag = check_pointed_model(&term.clone(), &u);
                            i = i+1;
                        }else {
                            break;
                        }
                    };
                    return flag;
                },
                s5rust::modal::ModalUnary::Diamond => {
                    // println!("Diamond");
                    let mut flag = false;
                    let mut i = 0;
                    let set = s5.model.clone();
                    let mut iter = set.iter();
                    while !flag {
                        if let Some(item) = iter.next(){
                            let val = item;
                            let u = S5PointedModel {model: set.clone(), world: val.to_owned()};
                            flag = check_pointed_model(&term.clone(), &u);
                            i = i+1;
                        }else{
                            break;
                        }
                    }
                    return flag
                }
            }
        }
        Tree::Atom(_) => {
            return s5.world.contains(&formula.to_string());
        }
    }
}

// pub fn get_prop_models(formula:PropFormula, universe: &Vec<Valuation>) -> Vec<Valuation> {
//     let mut models:Vec<Valuation> = Vec::new();
//     universe.iter()
//         .for_each(|val| {
//             if check_prop_model(&formula, val){
//                 models.push(val.clone());
//             }
//         });
//     return models; }


// pub fn get_mmodels(formula: ModalFormula, universe:Vec<Valuation>) -> Vec<HashSet<String>>{
//     let s5_pointed_models = get_models(formula, universe);
//     let mut models = HashSet::new();
//     s5_pointed_models.iter().for_each(|pm|{
//         models.insert(pm.clone().model);
//     });
//     return models;
// }


pub fn get_phi_models(formula: &ModalFormula, universe:&Vec<Valuation>) -> Vec<S5PointedModel>{
    let s5_pointed_models = generate_s5models(&universe); // esto puede irse a la puta mierda si tarda mucho.
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




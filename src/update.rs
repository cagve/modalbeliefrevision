use std::collections::HashSet;

use s5rust::modal::ModalFormula;

use crate::{distance::{closest_pointed_model, distance_pointed_to_pointed}, get_models, S5PointedModel};

#[derive(Clone)]
pub struct Update {
    pub phi: ModalFormula,
    pub mu: ModalFormula,
    pub universe:Vec<String>,
    pub base_set: Vec<S5PointedModel>,
    pub input_set: Vec<S5PointedModel>,
}


impl Update {
    pub fn new( phi: ModalFormula, mu: ModalFormula, universe:Vec<String>) -> Self {
        let base_set = get_models(phi.clone(), universe.clone());
        let input_set = get_models(mu.clone(), universe.clone());

        return Self{
            phi,
            mu,
            universe,
            base_set,
            input_set,
        };
    }

    pub fn output(&self) -> Vec<S5PointedModel>{
        let mut result = Vec::new();
        let input_set =  &self.input_set;
        self.base_set.iter()
            .for_each(|pmodel| {
                println!("pmodel = {:?}", pmodel);
                self.input_set.iter().for_each(|x| {
                    let d = distance_pointed_to_pointed(x,pmodel);
                    println!(">>> {} at distance {}", x, d);
                });
                let minimal_set = closest_pointed_model(input_set,pmodel);
                result.push(minimal_set);
            });
        // REMOVE DUPLICATES. vvvvvvvvvvvvvvvvvvvvvvv
        let mut seen = HashSet::new();
        result.retain(|item| seen.insert(item.clone()));
        // REMOVE DUPLCIATES ^^^^^^^^^^^^^^^^^^^^^^^^
        return result;
    }

    pub fn print_output(&self) {
        let output = &self.output();
        println!("Output models: {}", output.len());
        output.iter().for_each(|x| {
            println!("{}", x);
       });
    }
    pub fn verbose_output(&self) {
       let output = &self.output();
       println!("Output models: {}", output.len());
       output.iter().for_each(|x| {
           let mut id = 0;
           let m1 = closest_pointed_model(&self.base_set, x);
           let d = distance_pointed_to_pointed(&m1, x);
           println!("> Model {}:  {}",id, x);
           println!("  | closest: {}", m1);
           println!("  | D={}", d);
           id = id+1;
       })
    }
}

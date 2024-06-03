use crate::semantic::*; //TODO refactor name
use crate::distance::*; //TODO refactor name
use s5rust::modal::*;
use crate::semantic::S5PointedModel;

use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct Revision {
    pub phi: ModalFormula,
    pub mu: ModalFormula,
    pub universe:Vec<String>,
    pub base_set: Vec<S5PointedModel>,
    pub input_set: Vec<S5PointedModel>,
    pub output:Vec<S5PointedModel>,
    pub distance:Lexicographic
}



impl Revision {
    pub fn new( phi: ModalFormula, mu: ModalFormula, universe:Vec<String>) -> Self {
        let base_set = get_models(phi.clone(), universe.clone());
        let input_set = get_models(mu.clone(), universe.clone());
        let output = get_revision_models(&base_set, &input_set);
        let distance = distance_to_closest(&base_set, output.get(0).unwrap());

        Self {
            phi,
            mu,
            universe,
            base_set,
            input_set,
            output,
            distance,
        }
    }
    
    pub fn _to_string(&self) -> String{
        let s = format!("{} * {}", self.phi, self.mu); 
        return s;
    }

    pub fn beauty_distance(&self) -> String{
        return format!("{} * {} = {} ", self.phi, self.mu, self.distance.to_string());
    }

    pub fn beauty(&self, comp: &str) {
        match comp {
            "formula"    => println!("{} * {}", self.phi, self.mu),
            "output"    => {
                println!("Output models: {}", self.output.len());
                self.output.iter().for_each(|x| println!("{}", x))
            },
            "base_set"  => {
                println!("Base models: {}", self.base_set.len());
                self.base_set.iter().for_each(|x| println!("{}", x))
            },
            "input_set" => {
                println!("Input models: {}", self.input_set.len());
                self.input_set.iter().for_each(|x| println!("{}", x))
            },
            "debug"     => {
                println!("{} * {} = ", self.phi, self.mu);
                self.output.iter().for_each(|x| {
                    println!("| {}", x);
                })
            },
            _           => println!("Error") // TODO error managment
        };

    }

    pub fn verbose(&self){
        println!("{} * {} = ", self.phi, self.mu);
        self.output.iter().for_each(|x| {
            let d = distance_to_closest(&self.base_set, x);
            println!("| {}. D={}", x, d);
            let closest = get_set_of_closest_pointed_model(&self.base_set, x);
            closest.iter().for_each(|y|{
                println!("| | {}", y);
            })
        })
    }


}

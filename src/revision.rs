use crate::semantic::*; //TODO refactor name
use crate::distance::*; //TODO refactor name
use s5rust::modal::*;
use crate::semantic::S5PointedModel;

use std::fs::File;
use std::io::Write;

pub struct Revision {
    pub phi: ModalFormula,
    pub mu: ModalFormula,
    pub universe:Vec<String>,
    pub base_set: Vec<S5PointedModel>,
    pub input_set: Vec<S5PointedModel>,
    pub output:Vec<S5PointedModel>
}



impl Revision {
    pub fn new( phi: ModalFormula, mu: ModalFormula, universe:Vec<String>) -> Self {
        let base_set = get_models(phi.clone(), universe.clone());
        let input_set = get_models(mu.clone(), universe.clone());
        let output = get_revision_models(&base_set, &input_set);

        Self {
            phi,
            mu,
            universe,
            base_set,
            input_set,
            output,
        }
    }

    pub fn beauty(&self ) {
        println!("{} * {} = ", self.phi, self.mu);
        self.output.iter().for_each(|x| {
            println!("| {}", x);
        })
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

    pub fn render(&self, id: &str){
        let mut set = Vec::new();
        let mut title = "";
        match id {
            "phi" => {
                set = self.base_set.clone();
                let title = self.phi.to_string();
            },
            "mu" => {
                set = self.input_set.clone();
                let title = &self.mu.to_string();
            },
            "revision" => {
                set = self.output.clone();
                let title = "Result";
            },
            _ => {}
        };
        let mut dot_content = String::new();
        dot_content.push_str("digraph G {\n");
        dot_content.push_str(&format!("label=\"{}\"",title));
        dot_content.push_str("node [width=0.5, height=0.5, fixedsize=true]\n ");

        let mut cluster_id = 0;
        let mut node_id = 0;
        for pointed in set {
            dot_content.push_str(&format!(" subgraph cluster_{} {{ \n",cluster_id));
            let model = pointed.model.clone();
            for node in model {
                if node.clone() == pointed.world {
                    dot_content.push_str(&format!("     node_{} [label=\"{}\", shape=doublecircle]\n", node_id, node));
                }else{
                    dot_content.push_str(&format!("     node_{} [label=\"{}\", shape=circle]\n", node_id, node));
                }
                node_id = node_id + 1;
            }
            dot_content.push_str("  }\n");
            cluster_id = cluster_id + 1;
        }
        dot_content.push_str("}\n");

        let mut file = File::create(id.to_string()+".dot").expect("Unable to create file");
        file.write_all(dot_content.as_bytes());
    }
}

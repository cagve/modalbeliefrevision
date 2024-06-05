use std::collections::HashSet;
use std::fmt::format;
use std::fs::File;
use std::io::Write;

use crate::distance::{get_base_closest_set, distance_pointed_to_pointed, min_distance};
use crate::revision::Revision;
use crate::semantic::S5PointedModel;



pub fn write_dot(file:&str, dot_content:&str){
    let mut file = File::create(file).expect("Unable to create file");
    file.write_all(dot_content.as_bytes()).expect("Unable to write file");
}

pub fn dot_model(pointed: &S5PointedModel, world_id: usize, cluster_id:usize) -> String{
    let mut world_content = String::new();
    let m = &pointed.model;
    let mut id = world_id;
    m.iter()
        .for_each(|w| {
            if w.clone() == pointed.world {
                world_content.push_str(&format!("node_{} [label=\"{}\", shape=doublecircle]\n", id, w));
            }else{
                world_content.push_str(&format!("node_{} [label=\"{}\", shape=circle]\n", id, w));
            }
            id = id + 1;
        });

    let dot=&format!("subgraph cluster_{} {{
        label=\"Model {}\"
        {} }}
        ", cluster_id, cluster_id, world_content);
    return dot.to_string();
}

pub fn dot_models(title:&str, models:&Vec<S5PointedModel>, world_id: usize, cluster_id: usize, ) -> (usize, String){
    println!("printing models");
    let mut dot_content = String::new();
    let world_start = world_id.clone();
    let mut world_id = world_id;
    let mut cluster_id = cluster_id;
    models.iter()
        .for_each(|model|{
            let dot_model = dot_model(model, world_id, cluster_id);
            dot_content.push_str(&dot_model);
            world_id = world_id + model.model.len();
            cluster_id = cluster_id +1;
        });

    for n in world_start..world_id-1{
        dot_content.push_str(&format!("node_{}->node_{}\n",n,n+1));
    }

    let dot=&format!("subgraph cluster_{} {{
        edge [style=invis]
        label=\"{}\"
        {} }}
        ", cluster_id, title, dot_content);
    return (world_id, dot.to_string());
}

pub fn dot_debug_output(revision:&Revision) -> String{
    let mut dot_content = String::new();
    let mut cluster_id = 0;
    let world_id = 0;
    let dot_input_models = dot_models(&revision.mu.to_string(), &revision.output, world_id, cluster_id);
    let dot_input_str = dot_input_models.1;
    let world_id = dot_input_models.0;
    
    //input/output
    dot_content.push_str(&dot_input_str);

    //base
    cluster_id = cluster_id+revision.output.len();
    let base_closest = get_base_closest_set(&revision.base_set, &revision.input_set);
    println!("base = {:?}", base_closest);
    let dot_base_str = dot_models(&revision.phi.to_string(), &base_closest, world_id, cluster_id).1;
    dot_content.push_str(&dot_base_str);

    // Arrow

    return dot_content;
}

pub fn debug_output(revision:&Revision) -> String{
    let input = &revision.input_set;
    let revision = &revision.output;
    let set1: HashSet<_> = input.iter().cloned().collect();
    let set2: HashSet<_> = revision.iter().cloned().collect();
    let intersection: Vec<_> = set1.intersection(&set2).cloned().collect();
    let disjoint_set: Vec<_> = set1.symmetric_difference(&set2).cloned().collect();
    let mut dot_content = String::new();
    let mut cluster_id = 0;
    let world_id = 0;
    let dot_input_models = dot_models("Close", &intersection, world_id, cluster_id);
    let dot_input_str = dot_input_models.1;
    let world_id = dot_input_models.0;
    
    //input/output
    dot_content.push_str(&dot_input_str);

    //base
    cluster_id = cluster_id+intersection.len();
    let dot_base_str = dot_models("Far", &disjoint_set, world_id, cluster_id).1;
    dot_content.push_str(&dot_base_str);

    // Arrow

    return dot_content;

    
}

pub fn render_dot(title: &str, dot_content:&str) -> String{
    let dot=&format!("digraph G {{
    label = \"{}\"
    node [width=0.5, height=0.5, fixedsize=true]
    {}
}}",title,dot_content);

    return dot.to_string()
}

pub fn render(revision:&Revision, type_id:usize, file:&str) -> String{
    let mut result = String::new();
    let id = 0;
    match type_id {
        0 => { // Base 
            let title = revision.phi.to_string();
            let dot_content = dot_models(&title, &revision.base_set,0,0).1;
            result = render_dot(&title, &dot_content);
        }
        1 => { // Input
            let title = revision.mu.to_string();
            let dot_content = dot_models(&title, &revision.input_set, 0,0).1;
            result = render_dot(&title, &dot_content);
        },
        2 => { //Revision
            let title = revision._to_string();
            let dot_content = dot_models(&title, &revision.output,0,0).1;
            result = render_dot(&title, &dot_content);
        }
        3=> { //Debug Revision
            let d = revision.beauty_distance();
            let dot_content = dot_debug_output(revision);
            result = render_dot(&d, &dot_content);
        }
        4=> { //Debug own revision
            let d = revision.beauty_distance();
            let dot_content = debug_output(revision);
            result = render_dot(&d, &dot_content);
        }
        _ => println!("Not expected")
    }
    write_dot(file, &result);
    return result;
}





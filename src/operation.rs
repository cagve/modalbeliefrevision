use std::collections::HashSet;
use s5rust::{modal::ModalFormula, parser::build_formula};
use crate::{get_atoms_from_fromula, power_set, remove_duplicates, s5_contains, same_model, same_models, same_s5model, same_vecs5model, vec_union, S5PointedModel};
use crate::get_universe_from_formula;
use crate::get_models;
use crate::generate_universe;
use crate::generate_propset_from_atoms;
use crate::contains_b_in_a;

fn projection_w(world: &String, voc: &String) -> String{
    let set_voc: HashSet<char> = voc.chars().collect();
    let val_voc: HashSet<char> = world.chars().collect();
    let common_chars: HashSet<char> = set_voc.intersection(&val_voc).cloned().collect();
    let str = common_chars.into_iter().collect();
    return str;
}


fn projection_model(model: &Vec<String>, voc: &String) -> Vec<String>{
    let collect = model.iter()
        .map(|val| {
            let str = projection_w(&val.to_string(), voc);
            return str;
        })
        .collect::<Vec<String>>();
    return remove_duplicates(collect);
}

fn projection_pointed(pointed:&S5PointedModel, voc:&String) -> S5PointedModel{
    let s5pm = S5PointedModel {
        model: projection_model(&pointed.model, &voc),
        world: projection_w(&pointed.world, &voc)
    };
    return s5pm
}

fn circ(valset1: &Vec<String>, valset2: &Vec<String>) -> Vec<String>  {
    let mut result = Vec::new();
    for w in valset1 {
        for u in valset2 {
            if w.contains(u){
                result.push(w.to_string()); // Concatenate w and u
            } else if u.contains(w) {
                result.push(u.to_string()); // Concatenate w and u
            }else{
                let temp = format!("{}{}", w, u);
                result.push(temp); // Concatenate w and u
            }
        }
    }
    
    return remove_duplicates(result);
}


pub fn generate_Z(pm1:Vec<S5PointedModel>, pm2: Vec<S5PointedModel>, voc1:Vec<String>, voc2:Vec<String>) -> Vec<S5PointedModel>{
    // Genero el universo a partir de los modelos del primer conjunto de modelos puntuados. Es decir genero 2^P
    // let mut voc1 = Vec::new();
    // pm1.iter() 
    //     .for_each(|x| voc1.push(x.get_atoms()));
    // voc1 = remove_duplicates(voc1);
    let uni1 = generate_universe(voc1.concat());

    // Genero el universo a partir de los modelos del segundo conjunto de modelos puntuados. Es decir genero 2^Q
    // let mut voc2 = Vec::new();
    // pm2.iter() 
    //     .for_each(|x| voc2.push(x.get_atoms()));
    // voc2 = remove_duplicates(voc2);
    let uni2 = generate_universe(voc2.concat());


    // Genero el conjunto de modelos de Z 
    let op_models = circ(&uni1, &uni2);
    let models_z = power_set(&op_models);
    // Genero el conjunto de mundos de Z 
    let union_worlds = vec_union(&voc1, &voc2);
    let worlds_z  = power_set(&union_worlds);
    
    let mut pmodels = Vec::new();
    for modelo in models_z {
        for world_vec in worlds_z.clone() {
            let world = world_vec.concat();
            let model = modelo.clone();
            if model.contains(&world){
                pmodels.push(S5PointedModel{model,world});
            }
        }
    }


    // Debug
    return pmodels;
}

pub fn oplus(pm1:Vec<S5PointedModel>, pm2: Vec<S5PointedModel>, voc1:Vec<String>, voc2:Vec<String>) -> Vec<S5PointedModel>{
    let z = generate_Z(pm1.clone(), pm2.clone(), voc1.clone(), voc2.clone());

    // let mut voc1 = Vec::new();
    // pm1.iter() 
    //     .for_each(|x| voc1.push(x.get_atoms()));
    // voc1 = remove_duplicates(voc1);

    // let mut voc2 = Vec::new();
    // pm2.iter() 
    //     .for_each(|x| voc2.push(x.get_atoms()));
    // voc2 = remove_duplicates(voc2);

    let mut filtered = Vec::new();
    for w in z {
            let projection_1 = projection_pointed(&w, &voc1.concat());
            let projection_2 = projection_pointed(&w, &voc2.concat());
            // println!("");
            // println!("Model = {}", w);
            // println!("> projection_1 = {}", projection_1);
            // println!("> projection_2 = {}", projection_2);
            if s5_contains(projection_1, pm1.clone()) && s5_contains(projection_2, pm2.clone()){
                    filtered.push(w.clone());
            }
    };

    return filtered;
}

 
// SE PUEDE MEJORAR TODO MUCHO
pub fn op(phi:ModalFormula, psi:ModalFormula) -> Vec<S5PointedModel> {
    let universe_phi = get_universe_from_formula(&phi);
    let universe_psi = get_universe_from_formula(&psi);

    let pmmodels_phi = get_models(phi.clone(), universe_phi);
    let pmmodels_psi = get_models(psi.clone(), universe_psi);
    let voc1 = get_atoms_from_fromula(&phi);
    let voc2 = get_atoms_from_fromula(&psi);

    let result = oplus(pmmodels_phi, pmmodels_psi, voc1, voc2);
    return result;
}


pub fn test_fn(){
    let W = vec!["pq".to_string(), "q".to_string()];
    let U = vec!["qp".to_string(), "q".to_string()];

    let s51 = S5PointedModel{
        model: W.clone(),
        world: "q".to_string()
    };

    let s52 = S5PointedModel{
        model: U.clone(),
        world: "q".to_string()
    };

    // let Z = generate_Z(vec![s51], vec![s52]);
    // println!("Z = {:?}", Z);
    // let filtered = oplus(vec![s51], vec![s52]);
    // for (idx, pm)  in filtered.iter().enumerate() {
    //     println!("PModel {}", idx);
    //     println!(" > {} ", pm);
    // }
    let s1 = vec![s51.clone()];
    let s2 = vec![s52.clone()];
    let r = same_vecs5model(s2.clone(), s1.clone());
    let f = s5_contains(s51.clone(), s2);
    let v = same_s5model(&s51, &s52);
    let z = same_model(W, U);
    println!("r = {:?}", r);
    println!("f = {:?}", f);
    println!("v = {:?}", v);
    println!("z = {:?}", z);
}

pub fn theorem(fstr1:String, fstr2:String, debug:bool) -> bool{
    let f1: ModalFormula = build_formula(&fstr1).unwrap();
    let f2: ModalFormula = build_formula(&fstr2).unwrap();
    let con:ModalFormula = build_formula(&format!("({}) and ({})", fstr1, fstr2)).unwrap();


    let con_uni = get_universe_from_formula(&con);
    let con_models = get_models(con.clone(), con_uni.clone());

    let op = op(f1.clone(),f2.clone());
    
    
    let bool = same_vecs5model(op.clone(), con_models.clone());

    if debug{
        println!("");
        println!("ANALIZANDO {} y {}", f1, f2);
        println!("Oplus result >");
        for m in &op {
            println!("m = {}", m);
        }
        println!("Models >");
        for m in &con_models {
            println!("m = {}", m);
        }
        println!("====> RESULT: {}", bool);

    }
    return bool;

}


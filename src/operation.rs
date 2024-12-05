use std::{collections::HashSet, hash::Hash};
use s5rust::{modal::ModalFormula, parser::build_formula};

use crate::{get_phi_models, revision::{self, Revision}, utils::{generate_valuations, get_atoms_from_fromula, is_subset, powerset, powerset_hash, s5veceq, union}, S5PointedModel, Valuation};

fn projection_w(world: &String, voc: &String) -> String{
    let set_voc: HashSet<char> = voc.chars().collect();
    let val_voc: HashSet<char> = world.chars().collect();
    let common_chars: HashSet<char> = set_voc.intersection(&val_voc).cloned().collect();
    let str:String = common_chars.into_iter().collect();
    let mut sorted_valuation: Vec<char> = str.chars().collect();
    sorted_valuation.sort();
    let result:String = sorted_valuation.into_iter().collect();
    return result;
}


fn projection_model(model: &HashSet<Valuation>, voc: &String) -> HashSet<Valuation>{
    let collect = model.iter()
        .map(|val| {
            let str = projection_w(&val.to_string(), voc);
            return str;
        })
    .collect::<HashSet<Valuation>>();
    return collect;
}

pub fn projection_pointed(s5model:&S5PointedModel, voc:&String) -> S5PointedModel{
    let model= projection_model(&s5model.model, &voc);
    let world= projection_w(&s5model.world, &voc);

    let s5pm = S5PointedModel::new(model, world);
    return s5pm
}

pub fn circ(valset1: &HashSet<Valuation>, valset2: &HashSet<Valuation>) -> HashSet<Valuation>  {
    let mut result = HashSet::new();
    for w in valset1 {
        for u in valset2 {
            if w.contains(u){
                result.insert(w.to_string()); // Concatenate w and u
            } else if u.contains(w) {
                result.insert(u.to_string()); // Concatenate w and u
            }else{
                let temp = format!("{}{}", w, u);
                result.insert(temp); // Concatenate w and u
            }
        }

    }
    return result;
}


pub fn generate_z(voc1:&String, voc2:&String) -> Vec<S5PointedModel>{
    let uni1:HashSet<Valuation> = generate_valuations(&voc1).into_iter().collect();
    let uni2:HashSet<Valuation> = generate_valuations(&voc2).into_iter().collect();

    let mut owned_string: String = voc1.to_owned();
    let borrowed_string: &str = voc2;
    owned_string.push_str(borrowed_string);

    // Genero el conjunto de modelos de Z 
    let op_models2 = circ(&uni1, &uni2);
    let op_models = generate_valuations(&owned_string);
    let op_vec:Vec<_> = op_models.clone().into_iter().collect();        // VECTOR
    let models_vec = powerset(&op_vec);                         // VECTOR
    let models_z = models_vec.iter()                            // Vec of Hash [{"p", ""}, {"q"}]
        .map(|x| {
            let hash:HashSet<_> = x.into_iter().collect();  
            return hash;
        })
        .collect::<Vec<_>>();


    let mut pmodels = Vec::new();
    for model in models_z {
        for world in op_models.clone() {
            if model.contains(&world){
                let m = model.iter().map(|x| x.to_string()).collect();
                let s5 = S5PointedModel::new(m, world);
                pmodels.push(s5);
            }
        }
    }

    // Debug
    return pmodels;
}

pub fn oplus(set1:Vec<S5PointedModel>, set2: Vec<S5PointedModel>, voc1:String, voc2:String) -> Vec<S5PointedModel>{
    let z = generate_z(&voc1, &voc2);

    let mut filtered = Vec::new();
    for w in z {
            let projection_1 = projection_pointed(&w, &voc1);
            let projection_2 = projection_pointed(&w, &voc2);
            // println!("");
            // println!("Model = {}", w);
            // println!("> projection_1 = {}", projection_1);
            // println!("> projection_2 = {}", projection_2);
            let flag1 = projection_1.contained(&set1);
            let flag2 = projection_2.contained(&set2);
            if flag1&&flag2 {
                filtered.push(w.clone());
            }
    };

    return filtered;
}

// // SE PUEDE MEJORAR TODO MUCHO
pub fn op(phi:ModalFormula, psi:ModalFormula) -> Vec<S5PointedModel> {
    let atoms_phi = get_atoms_from_fromula(&phi);
    let atoms_psi = get_atoms_from_fromula(&psi);
    let universe_phi = generate_valuations(&atoms_phi);
    let universe_psi = generate_valuations(&atoms_psi);

    let pmmodels_phi = get_phi_models(&phi, &universe_phi);
    let pmmodels_psi = get_phi_models(&psi, &universe_psi);
    let voc1 = get_atoms_from_fromula(&phi);
    let voc2 = get_atoms_from_fromula(&psi);

    let result = oplus(pmmodels_phi, pmmodels_psi, voc1, voc2);
    return result;
}

// pub fn test_fn(){
//     let W = vec!["qp".to_string(), "q".to_string()];
//     let U = vec!["qp".to_string(), "q".to_string(), "rqp".to_string()];
//     let V = vec!["pq".to_string(), "q".to_string()];
//
//     let mut s51 = S5PointedModel{
//         model: W.clone(),
//         world: "qp".to_string()
//     };
//
//     let s52 = S5PointedModel{
//         model: U.clone(),
//         world: "qp".to_string()
//     };
//
//     let s53 = S5PointedModel{
//         model: V.clone(),
//         world: "p".to_string()
//     };
//
//     // let Z = generate_Z(vec![s51], vec![s52]);
//     // println!("Z = {:?}", Z);
//     // let filtered = oplus(vec![s51], vec![s52]);
//     // for (idx, pm)  in filtered.iter().enumerate() {
//     //     println!("PModel {}", idx);
//     //     println!(" > {} ", pm);
//     // }
//     let s1 = vec![s51.clone(), s52.clone(), s53.clone() ];
//     let s2 = vec![s52.clone(), s51.clone(), s52.clone()];
//     let s3 = vec![s53.clone()];
//     let r = s5veceq(&s1, &s2);
//     println!("r = {:?}", r);
//     // let f = s5_contains(s51.clone(), s2);
//     // let v = same_s5model(&s51, &s52);
//     // let z = same_model(W, U);
//     // println!("r = {:?}", r);
//     // println!("f = {:?}", f);
//     // println!("v = {:?}", v);
//     // println!("z = {:?}", z);
// }
//
pub fn theorem(fstr1:String, fstr2:String, fstr3:String, debug:bool) -> bool{
    let f1: ModalFormula = build_formula(&fstr1).unwrap();
    let f2: ModalFormula = build_formula(&fstr2).unwrap();
    let f3: ModalFormula = build_formula(&fstr3).unwrap();
    let con:ModalFormula = build_formula(&format!("({}) and ({})", fstr1, fstr2)).unwrap();


    let atoms_con = get_atoms_from_fromula(&con);
    let universe_con = generate_valuations(&atoms_con);
    let con_models = get_phi_models(&con, &universe_con);

    let op = op(f1.clone(),f2.clone());
    println!("OPERATION");
    
    
    let bool = s5veceq(&op, &con_models);

    if debug{
        println!("");
        println!("ANALIZANDO {} y {}", f1, f2);
        println!("Oplus result > {}", op.len());
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

pub fn theorem_revision(fstr1:String, fstr2:String, fstr3:String, debug:bool) -> bool{
    let phi: ModalFormula = build_formula(&fstr1).unwrap();
    let mu: ModalFormula = build_formula(&fstr2).unwrap();
    let psi: ModalFormula = build_formula(&fstr3).unwrap();
    let con:ModalFormula = build_formula(&format!("({}) and ({})", fstr1, fstr3)).unwrap();
    println!("phi = {}", phi);
    println!("psi = {}", psi);
    println!("con = {}", con);
    println!("mu = {}", mu);

    let atoms_phi = get_atoms_from_fromula(&phi);
    let universe_phi = generate_valuations(&atoms_phi);
    // let models_phi = get_phi_models(&phi, &universe_phi);
    println!("PHI");

    let atoms_psi = get_atoms_from_fromula(&psi);
    let universe_psi = generate_valuations(&atoms_psi);
    let models_psi = get_phi_models(&psi, &universe_psi);
    println!("PSI");


    
    let atoms_con = format!("{}{}", atoms_psi, atoms_phi);
    let universe_con = generate_valuations(&atoms_con);
    let m = get_phi_models(&con, &universe_con);
    println!("CON");

    

    let revision = Revision::new(con.clone(),mu.clone(),universe_con.clone());
    let ou1 = revision.clone().output;

    let revision2 = Revision::new(phi.clone(),mu.clone(),universe_phi.clone());
    let ou2 = revision2.output;

    println!("ou2 = {:?}", ou2);
    println!("models_psi = {:?}", models_psi);

    
    println!("atoms_psi = {:?}", atoms_psi);
    println!("atoms_phi = {:?}", atoms_phi);
    println!("atoms_con = {:?}", atoms_phi);
    let oplus = oplus(ou2.clone(), models_psi, atoms_phi, atoms_psi);

    let result = ou1 == oplus;

    if debug{
        println!("");
        println!("Oplus result > {}", oplus.len());
        for m in &oplus {
            println!("m = {}", m);
        }
        println!("Models >");
        for m in &ou1 {
            println!("m = {}", m);
        }
        println!("====> RESULT: {}", result);
    }

    return result;
}

pub fn debug_example(phi:ModalFormula, psi:ModalFormula) {
    let atoms_phi = get_atoms_from_fromula(&phi);
    let atoms_psi = get_atoms_from_fromula(&psi);
    let universe_phi = generate_valuations(&atoms_phi);
    let universe_psi = generate_valuations(&atoms_psi);

    let universe_both = generate_valuations(&"pq".to_string());

    let pmmodels_phi = get_phi_models(&phi, &universe_phi);
    let pmmodels_phi_both = get_phi_models(&phi, &universe_both);
    let f = is_subset(pmmodels_phi, pmmodels_phi_both);
    println!("Subset = {:?}", f);
    // let pmmodels_psi = get_phi_models(&psi, &universe_psi);
    // let voc1 = get_atoms_from_fromula(&phi);
    // let voc2 = get_atoms_from_fromula(&psi);
    //
    //
    // println!("Restricted models");
    // for m in &pmmodels_phi {
    //     println!("  {}", m);
    // }
    //
    // println!("Non restricted models");
    // for m in &pmmodels_phi_both {
    //     println!("  {}", m);
    // }
    //
    //
}

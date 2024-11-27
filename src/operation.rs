use std::collections::HashSet;
use s5rust::{modal::ModalFormula, parser::build_formula};
use crate::{generate_propset_from_atoms, generate_universe, get_atoms_from_fromula, get_mmodels, get_models, get_universe_from_formula, power_set, projection, remove_duplicates, same_models, S5PointedModel};
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


pub fn generate_Z(pm1:Vec<S5PointedModel>, pm2: Vec<S5PointedModel>, voc1:String, voc2:String){
    // let z = Vec::new();
    //
    // let circ = circ(voc1, voc2); Z
    // TODO
    // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
}

fn oplus(set1: &Vec<Vec<String>>, voc1:String, set2: &Vec<Vec<String>>, voc2:String, debug:bool) -> Vec<Vec<String>>  {
    // let pow_voc1 = power_set(voc1);
    // let pow_voc2 = (power_set(voc2));
    let pow_voc1 = generate_universe(voc1.clone());
    let pow_voc2 = generate_universe(voc2.clone());

    println!("pow_voc1 = {:?}", pow_voc1);
    println!("pow_voc2 = {:?}", pow_voc2);

    let mut result = Vec::new();
    let temp = circ(&pow_voc1, &pow_voc2);
    let universe = power_set(&temp);

    for model in &universe {
        let projection1 = projection(&model, &voc1);
        let projection2 = projection(&model, &voc2);
        if debug {
            println!(" ");
            println!("====== Projection 1");
            println!(">> model.clone() = {:?}", model.clone());
            println!(">> projection = {:?}", projection1);
            println!("====== Projection 2");
            println!(">> model.clone() = {:?}", model.clone());
            println!(">> projection = {:?}", projection2);
        }
        if contains_b_in_a(set1.clone(), projection1.clone()) && contains_b_in_a(set2.clone(), projection2.clone()){
            result.push(model.clone());
            if debug {
                println!("******************************************* vvvvvv");
                println!("DEBUG={:?} in {:?}", projection1, set1);
                println!("DEBUG={:?} in {:?}", projection2, set2);
                println!("******************************************* ^^^^^^");
            }
        }else{
            if debug {
                println!("DEBUG={:?} not in {:?}", projection1, set1);
                println!("DEBUG={:?} not in {:?}", projection2, set2);
            }
        };
    }

    let deduplicated  = remove_duplicates(result);
    return deduplicated;
}

fn oplus_remove(set1: &Vec<Vec<String>>, voc1:String, set2: &Vec<Vec<String>>, voc2:String, debug:bool) -> Vec<Vec<String>>  {
    // let pow_voc1 = power_set(voc1);
    // let pow_voc2 = (power_set(voc2));
    let pow_voc1 = generate_universe(voc1.clone());
    let pow_voc2 = generate_universe(voc2.clone());

    println!("pow_voc1 = {:?}", pow_voc1);
    println!("pow_voc2 = {:?}", pow_voc2);

    let mut result = Vec::new();
    let temp = circ(&pow_voc1, &pow_voc2);
    let universe = power_set(&temp);
    for model in &universe {
        let projection1 = remove_proposition(model.clone(), voc2.clone());
        let projection2 = remove_proposition(model.clone(), voc1.clone());
        if debug {
            println!(" ");
            println!("====== Projection 1");
            println!(">> model.clone() = {:?}", model.clone());
            println!(">> projection = {:?}", projection1);
            println!("====== Projection 2");
            println!(">> model.clone() = {:?}", model.clone());
            println!(">> projection = {:?}", projection2);
        }
        if contains_b_in_a(set1.clone(), projection1.clone()) && contains_b_in_a(set2.clone(), projection2.clone()){
            result.push(model.clone());
            if debug {
                println!("******************************************* vvvvvv");
                println!("DEBUG={:?} in {:?}", projection1, set1);
                println!("DEBUG={:?} in {:?}", projection2, set2);
                println!("******************************************* ^^^^^^");
            }
        }else{
            if debug {
                println!("DEBUG={:?} not in {:?}", projection1, set1);
                println!("DEBUG={:?} not in {:?}", projection2, set2);
            }
        };
    }

    let deduplicated  = remove_duplicates(result);
    return deduplicated;
}


pub fn op(f1: ModalFormula, f2:ModalFormula, debug:bool) ->Vec<Vec<String>> {

    let f1_atoms = get_atoms_from_fromula(&f1);
    let f1_pvar = generate_propset_from_atoms(f1_atoms.concat());
    let f1_uni = generate_universe(f1_pvar.clone());
    let f1_models = get_models(f1, f1_uni.clone()).iter().map(|x| x.clone().model).collect();

    let f2_atoms = get_atoms_from_fromula(&f2);
    let f2_pvar = generate_propset_from_atoms(f2_atoms.concat());
    let f2_uni = generate_universe(f2_pvar.clone());
    let f2_models = get_models(f2, f2_uni.clone()).iter().map(|x| x.clone().model).collect();

    let result = oplus_remove(&f1_models, f1_pvar.clone(), &f2_models, f2_pvar.clone(), debug);

    if debug{
        println!("---------------------");
        println!("At={:?}", f1_atoms );
        println!("Pvar={:?}", f1_pvar);
        println!("Uni={:?}", f1_uni);
        println!("Models={:?}", f1_models);

        println!("---------------------");
        println!("At={:?}", f2_atoms );
        println!("Pvar={:?}", f2_pvar);
        println!("Uni={:?}", f2_uni);
        println!("Models={:?}", f2_models);

    }
    return result;
}
 
pub fn test_fn(){
    let W = vec!["pq".to_string(), "q".to_string(), "p".to_string(),"".to_string()];
    let s5 = S5PointedModel{
        model: W.clone(),
        world: "pq".to_string()
    };
    let projectionW = projection_model(&W, &"q".to_string());
    let projectionPM = projection_pointed(&s5, &"q".to_string());
    println!("projectionW = {:?}", projectionW);
    println!("projectionPM = {:?}", projectionPM);
}

pub fn theorem(fstr1:String, fstr2:String, debug:bool) -> bool{
    let f1: ModalFormula = build_formula(&fstr1).unwrap();
    let f2: ModalFormula = build_formula(&fstr2).unwrap();
    let con:ModalFormula = build_formula(&format!("({}) and ({})", fstr1, fstr2)).unwrap();


    let con_uni = get_universe_from_formula(&con);
    let con_models = get_mmodels(con.clone(), con_uni.clone());

    let oplus = op(f1,f2, true);
    if debug{
        println!("Oplus result >");
        for m in &oplus {
            println!("m = {:?}", m);
        }
        println!("Models >");
        for m in &con_models {
            println!("m = {:?}", m);
        }
    }
    let bool = same_models(oplus.clone(), con_models.clone());
    return bool;

}


use ::distance::hamming;
use s5rust::parser::build_formula;
use s5rust::modal::*;

use crate::semantic::*; //TODO refactor name
use crate::revision::*;
use crate::utils::*;
mod distance;
mod semantic;
mod revision;
mod graphics;
mod utils;

fn m2(f1: ModalFormula, f2: ModalFormula, f3:ModalFormula) -> bool{
    let prop_set = generate_propset(2); 
    let universe = generate_universe(prop_set.clone());
    let mut flag = true ;
    let revision = Revision::new(f1.clone(),f2.clone(),universe);
    for model in revision.base_set.clone()  {
        flag = check_pointed_model(&f3, &model);
        if !flag{
            println!("{} ⊬ {}", f1, f3);
            break;
        }
    }

    flag = true;
    for model in revision.output.clone()  {
        flag = check_pointed_model(&f3, &model);
        if !flag{
            println!("{} * {} ⊬ {}", revision.phi, revision.mu, f3);
            break;
        }
    }

    if flag {
        println!("{} * {} Ⱶ {}", revision.phi, revision.mu, f3);
    }
    return flag;
}



fn main() {
    let prop_set = generate_propset(2); 
    let universe = generate_universe(prop_set.clone());
    let mut f1: ModalFormula = build_formula("diamond(p and q)").unwrap();
    let mut f2: ModalFormula = build_formula("box (not p)").unwrap();
    let mut f3: ModalFormula = build_formula("q").unwrap();
    m2(f1,f2,f3);
}

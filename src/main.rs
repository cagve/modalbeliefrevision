use ::distance::hamming;
use graphics::dot_distance;
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

fn simple_output(revision: &Revision){
    revision.beauty("formula");
    revision.beauty("distance");
    revision.beauty("base_set");
    revision.beauty("input_set");
    revision.beauty("output");
}

fn verbose_output(revision: &Revision){
    revision.beauty("formula");
    revision.beauty("distance");
    revision.beauty("base_set");
    revision.verbose_beauty("input_set");
    revision.beauty("output");
}

fn order_output(revision: &Revision){
    let base = revision.base_set.clone();
    for model in revision.input_set.clone() {
        model.print_order(base.clone());
    }
}

// f1 (box p) and not q
// f2 (diamond(not p)) or q")
fn counterexample(){
// f1 (box p) and not q * (diamond(not p)) or q")
    let phi_model1 = vec!["p".to_string(),"pq".to_string()];
    let mu_model1 = vec!["p".to_string(),"pq".to_string(), "".to_string()];
    let phi_model2 = vec!["p".to_string(),"pq".to_string()];
    let mu_model2 = vec!["p".to_string(),"pq".to_string()];
    let phi_pointed1 = S5PointedModel{
        model: phi_model1,
        world: "p".to_string()
    };
    let mu_pointed1 = S5PointedModel{
        model: mu_model1,
        world: "p".to_string()
    };
    let phi_pointed2 = S5PointedModel{
        model: phi_model2,
        world: "p".to_string()
    };
    let mu_pointed2 = S5PointedModel{
        model: mu_model2,
        world: "pq".to_string()
    };
    dot_distance("dot/dot1.dot", &phi_pointed1, &mu_pointed1);
    dot_distance("dot/dot2.dot", &phi_pointed2, &mu_pointed2);
}

fn main() {
    let prop_set = generate_propset(2); 
    let universe = generate_universe(prop_set.clone());
    let mut f1: ModalFormula = build_formula("(box p) and not q").unwrap();
    let mut f2: ModalFormula = build_formula("(diamond(not p)) or q").unwrap();
    let mut f3: ModalFormula = build_formula("q").unwrap();
    // let mut f1: ModalFormula = build_formula("p and (box q)").unwrap();
    // let mut f2: ModalFormula = build_formula("box (not q) ").unwrap();

    let revision = Revision::new(f1.clone(),f2.clone(),universe.clone());
    // verbose_output(&revision);
    // order_output(&revision);
    counterexample();


    

}

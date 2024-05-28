use ::distance::hamming;
use s5rust::parser::build_formula;
use s5rust::modal::*;

use crate::semantic::*; //TODO refactor name
use crate::distance::*;
use crate::revision::*;
mod distance;
mod semantic;
mod revision;




fn main() {
    let prop_set = generate_propset(2);
    let universe = generate_universe(prop_set.clone());
    let mut f1: ModalFormula = build_formula("diamond b").unwrap();
    let mut f2: ModalFormula = build_formula("box b").unwrap();


    // let m1 = vec!["a".to_string(), "b".to_string()];
    // let m2 = vec!["a".to_string(), "b".to_string()];
    // let d = distance_set_to_set(&m1, &m2);
    // let w1 = m1.get(1).unwrap();
    // let w2 = m2.get(1).unwrap();
    // let h = hamming_distance(w1,w2);
    // println!("d = {:?}", d);
    // println!("h = {:?}", h);
    // let world = "".to_string();
    //

    let revision = Revision::new(f1,f2,universe);
    print!("{:?}", revision.output);
}

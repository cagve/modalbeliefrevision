use ::distance::hamming;
use s5rust::parser::build_formula;
use s5rust::modal::*;

use crate::semantic::*; //TODO refactor name
use crate::graphics::*;
use crate::revision::*;
mod distance;
mod semantic;
mod revision;
mod graphics;




fn main() {
    let prop_set = generate_propset(2); let universe = generate_universe(prop_set.clone());
    let mut f1: ModalFormula = build_formula(" a").unwrap();
    let mut f2: ModalFormula = build_formula(" b").unwrap();


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
    render(&revision, 0, "dot/base.dot");
    render(&revision, 1, "dot/input.dot");
    render(&revision, 2, "dot/revision.dot");
    render(&revision, 3, "dot/debug.dot");
    render(&revision, 4, "dot/revision_output.dot");
    // let dot_file = "dot/dot.dot";
    // render(dot_file, "Output", &revision.output);
}

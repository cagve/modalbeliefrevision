use graphics::dot_distance;
use s5rust::parser::build_formula;
use s5rust::modal::*;

use crate::revision::*;
use crate::utils::*;
mod distance;
mod semantic;
mod revision;
mod graphics;
mod utils;




fn main() {
    let prop_set = generate_propset(2); 
    let universe = generate_universe(prop_set.clone());
    let mut f1: ModalFormula = build_formula("diamond (p and q)").unwrap();
    let mut f2: ModalFormula = build_formula("not p").unwrap();



    let revision = Revision::new(f1,f2,universe);

    revision.beauty("formula");
    // revision.debug("base_set");
    // revision.debug("input_set");
    // revision.debug("output");

    let mut f3: ModalFormula = build_formula("q").unwrap();
    let r = revision.clone();
    let p1 = r.output.get(3).unwrap();
    let p2 = r.output.get(0).unwrap();
    dot_distance("test.dot", &p1, &p2);
}

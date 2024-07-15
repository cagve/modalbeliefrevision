use std::collections::HashSet;

use distance::closest_set_pointed;
use ::distance::hamming;
use graphics::dot_distance;
use s5rust::parser::build_formula;
use s5rust::modal::*;

use crate::distance::closest_pointed_model;
use crate::distance::min_distance;
use crate::semantic::*; //TODO refactor name
use crate::revision::*;
use crate::utils::*;
mod distance;
mod semantic;
mod revision;
mod graphics;
mod utils;

fn test_formula(rev: Revision, f3:ModalFormula) -> bool{
    let mut flag = true;
    let remain = vect_difference(&rev.input_set, &rev.output);
    for model in rev.output.clone()  {
        flag = check_pointed_model(&f3, &model);
        println!("flag = {:?}", flag);
        if !flag{
            println!("{} * {} ⊬ {}", rev.phi, rev.mu, f3);
            println!("model = {:?}", model);
            break;
        }
    }
    println!("====");
    let mut flag2 = false;
    for model in remain.clone()  {
        flag2 = check_pointed_model(&f3, &model);
        println!("flag2 = {:?}", flag2);
        if flag2 { //real
        // if !flag2 { //prov para pobar que formula descarta que.
            println!("model = {:?}", model);
        }
    }

    if flag {
        println!("{} * {} Ⱶ {}", rev.phi, rev.mu, f3);
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
    print!("D= ");
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
    let mut f1: ModalFormula = build_formula("box (p or q)").unwrap();
    let mut fb: ModalFormula = build_formula("(not q) or (box (not p))").unwrap();
    let mut f2: ModalFormula = build_formula("diamond ((not p) and (not q))").unwrap();
    let mut f3: ModalFormula = build_formula("box (not p)").unwrap();
    // let mut fa: ModalFormula = build_formula("(not (box(p implies q))) and (p implies q) and (").unwrap(); 
    // let mut fb: ModalFormula = build_formula("(not (box(p implies q))) and (p implies q) and (q implies ((not (box((not q) implies p))) or (not (box (q implies (not p))))))").unwrap(); 
    // let mut f3: ModalFormula = build_formula("(not q) and (diamond q) and ((diamond (p and not q)) implies (box(q or p))) and ((not (p or q)) implies (diamond (q and not p))) and ((p and (not q)) implies (diamond (p and q)))").unwrap(); THIS IS RESULT


    let r1 = Revision::new(f1.clone(),fb.clone(),universe.clone());
    let r2 = Revision::new(f1.clone(),f2.clone(),universe.clone());
    verbose_output(&r2);
    // let o1 = r1.output;
    // let o2 = iterated_rev(&r2, f3, universe);
    // println!("o1 = {:?}", o1);
    // println!("o2 = {:?}", o2);
    // let f = equiv_output(o1, o2);
    // println!("f = {:?}", f);


    // // EXAMPLE 8
    // let a1 = "(not q)";
    // let a2 = "(p and q) ";
    // let b1 ="(not p)";
    // let b2 = "(p and box(p implies q)) ";
    // let c1 = "(box p)";
    // let c2 ="(diamond (not p))";
    // let d1 = "(box p )";
    // let d2 = "(box (not p))";
    //
    // let example8 = vec![(a1,a2),(b1,b2),(c1,c2),(d1,d2)];
    // 
    // // Example 9
    // let a1 = "(p and q)";
    // let a2 = "(not q)";
    // let b1 ="(p and (box(p implies q)))";
    // let b2 = "(not q)";
    // let c1 = "(box q)";
    // let c2 ="(not q)";
    // let d1 = "((box q) and p)";
    // let d2 = "(not q)";
    // let example9 =  vec![(a1,a2),(b1,b2),(c1,c2),(d1,d2)];
    //
    //
    // EXAMPLE PROBLEMATIC STRICT
    // let a1="(box(p implies q))";
    // let a2="(not(box(p implies q)))";
    // let example9 =  vec![(a1,a2)];
    // example_paper(example9, universe);

}

// Example 8
fn example_paper (example:Vec<(&str,&str)>, universe: Vec<String>){
    example.iter().for_each(|x|{
        let mut f1: ModalFormula = build_formula(x.0).unwrap();
        let mut f2: ModalFormula = build_formula(x.1).unwrap();
        let revision = Revision::new(f1.clone(),f2.clone(),universe.clone());
        let dmin = revision.clone().distance;
        let phi_latex = to_latex(revision.phi);
        let mu_latex = to_latex(revision.mu);
        let mut output = format!("
\\begin{{table}}[ht!]
\\centering
Example: $({phi_latex})*({mu_latex})$ \\\\
Min $\\Delta$: ${dmin}$ \\\\
\\begin{{tabular}}{{ | c | c | c |}}
\\hline
Models of ${phi_latex}$ & Models of ${mu_latex}$ & Distance \\\\
\\hline
        ");
        revision.input_set.iter().for_each(|x| {
            let closest_pointed = closest_pointed_model(&revision.base_set, x);
            let ph_model = closest_pointed.to_latex().to_string();
            let mu_model = x.to_latex().to_string();
            let d = min_distance(&revision.base_set, x);
            if d == dmin{
                output.push_str("\\rowcolor{{green}}");
            }
            output.push_str(&format!("{ph_model} & {mu_model} & {d} \\\\ \n").to_string());
        });
        output.push_str(&format!("\\hline"));
        output.push_str(&format!("\\end{{tabular}}"));
        output.push_str(&format!("\\end{{table}}"));
        output.push_str(&format!("%==================="));
        println!("{}", output);
        // verbose_output(&revision);
    });


}


fn to_latex(f:ModalFormula) -> String{
    let mut f = f.to_string();
    f = f.replace("◻", "\\Box ");
    f = f.replace("◊", "\\Diamond ");
    f = f.replace("¬", "\\lnot ");
    f = f.replace(" ↔ ", "\\leqv ");
    f = f.replace(" → ", "\\limp ");
    f = f.replace(" ∧ ", "\\land ");
    f = f.replace(" ∨ ", "\\lor ");
    return f

}


fn equiv_output(r1: Vec<S5PointedModel>, r2:Vec<S5PointedModel>) -> bool{
    if r1.len() != r2.len(){
        println!("Card diff");
        return false
    }
    let a_set: HashSet<_> = r1.iter().collect();
    let b_set: HashSet<_> = r2.iter().collect();
    let mut flag = r2.iter().all(|item| a_set.contains(item));
    flag = r1.iter().all(|item| b_set.contains(item));

    return flag;
}

fn iterated_rev(r1: &Revision, f3:ModalFormula, universe: Vec<String>) -> Vec<S5PointedModel> {
    let outputr1 = r1.clone().output; // Models of revision f1 * f2
    println!("outputr1 = {:?}", outputr1);
    let models = get_models(f3, universe); // Modles of f3.
    for m in models.clone(){
        println!("{}", m);
    }
    let r2 = closest_set_pointed(&outputr1, &models); // Revision (f1*f2) * f3
    return r2;
}

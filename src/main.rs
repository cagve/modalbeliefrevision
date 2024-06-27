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
    let mut f1: ModalFormula = build_formula("(box p) and (not q)").unwrap();
    let mut f2: ModalFormula = build_formula("(not p) or (not q)").unwrap();
    let mut f3: ModalFormula = build_formula("q").unwrap();

    // let revision = Revision::new(f1.clone(),f2.clone(),universe.clone());
    // verbose_output(&revision);
    // println!("{}", get_models(f2, ));
    // counterexample();


    // EXAMPLE 8
    let a1 = "(not q)";
    let a2 = "(p and q) ";
    let b1 ="(not p)";
    let b2 = "(p and box(p implies q)) ";
    let c1 = "(box p)";
    let c2 ="(diamond (not p))";
    let d1 = "(box p )";
    let d2 = "(box (not p))";

    let example8 = vec![(a1,a2),(b1,b2),(c1,c2),(d1,d2)];
    
    // Example 9
    let a1 = "(p and q)";
    let a2 = "(not q)";
    let b1 ="(p and (box(p implies q)))";
    let b2 = "(not q)";
    let c1 = "(box q)";
    let c2 ="(not q)";
    let d1 = "((box q) and p)";
    let d2 = "(not q)";
    let example9 =  vec![(a1,a2),(b1,b2),(c1,c2),(d1,d2)];

    example_paper(example9, universe);

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
            // output.push_str(format!("{ph_model} & {mu_model} & {d} \\\\").to_string());
            // output.push_str(format!("{} & {} & {} \\\\", ph_model, mu_model, d.to_string()));
            // println!("{} & {} & {} \\\\",  closest_pointed.to_latex(), x.to_latex(), d);
        });
        println!("\\hline");
        println!("\\end{{tabular}}");
        println!("\\end{{table}}");
        println!("%===================");
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

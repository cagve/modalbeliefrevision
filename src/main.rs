use std::collections::HashSet;
use std::env;

use distance::closest_set_pointed;
use graphics::dot_distance;
use operation::test_fn;
use operation::theorem;
use s5rust::parser::build_formula;
use s5rust::modal::*;

use crate::distance::closest_pointed_model;
use crate::distance::min_distance;
use crate::semantic::*; 
use crate::revision::*;
use crate::utils::*;
mod distance;
mod operation;
mod update;
mod semantic;
mod revision;
mod graphics;
mod utils;

fn test_formula(output: Vec<S5PointedModel>, input: ModalFormula, f3:ModalFormula, universe: Vec<String>, debug:bool) -> bool{
    if debug{println!("Debugging formula {}", f3);}
    let mut flag = true;
    let mut r1 = true;
    let mut r2 = false;
    let input_set = get_models(input, universe);
    let remain = vec_difference(&input_set, &output);
    if debug{println!("#> Cheking output models. {} should be true", f3);}
    for model in output  {
        flag = check_pointed_model(&f3, &model);
        if debug{ 
            if !flag {
                r1 = flag;
                println!("#>> {} => {:?}", model, flag); 
            } else {
                println!(" >> {} => {:?}", model, flag);
            }
        }
    }
    if debug{println!("#> Cheking input models but not in the result. {} should be false", f3);}
    let mut flag2 = false;
    for model in remain.clone()  {
        flag2 = check_pointed_model(&f3, &model);
        if debug{ 
            if flag2 {
                r2 = flag2;
                println!("#>> {} => {:?}", model, flag2); 
            } else {
                println!(" >> {} => {:?}", model, flag2);
            }
        }
    }

    // if r1 && !r2 {
    //      println!("{} * {} ≡ {}", rev.phi, rev.mu, f3);
    // }else{
    //      println!("{} * {} ≢ {}", rev.phi, rev.mu, f3);
    // }
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
    let formulas1 = vec!["box(p and diamond(q)) implies diamond(p and box(q))".to_string(),
    "diamond(p and box(q)) implies box(p implies diamond(q))".to_string(),
    "box(p implies diamond(q)) and diamond(p or q)".to_string(),
    "diamond(p and box(q)) implies box(diamond(p) implies q)".to_string(),
    "box(diamond(p) implies diamond(q)) implies diamond(p implies box(q))".to_string(),
    "diamond(box(p) and diamond(q)) implies box(diamond(p) implies q)".to_string(),
    "box(p implies diamond(box(q))) and diamond(p and q)".to_string(),
    "diamond(box(p implies q)) implies box(diamond(p implies q))".to_string(),
    "box(diamond(p) and diamond(q)) implies diamond(p and q)".to_string(),
    "diamond(p implies box(q)) implies box(p implies diamond(q))".to_string(),
    ];

    let formulas2 = vec![
        "box(r) and diamond(r)".to_string(),
        "diamond(r) implies box(r)".to_string(),
        "box(r) implies diamond(r)".to_string(),
        "diamond(r) and box(r)".to_string(),
        "diamond(r or r)".to_string(),
        "box(r and diamond(r))".to_string(),
        "diamond(box(r))".to_string(),
        "box(diamond(r))".to_string(),
        "diamond(r) or box(r)".to_string(),
        "box(r) implies diamond(box(r))".to_string(),
    ];

    let f1 = "diamond p".to_string();
    let f2 = "diamond q".to_string();
    // theorem(f1, f2, true);
    theorem(formulas1[0].to_string(), formulas2[1].to_string(), true);
    //  for f1 in &formulas1 {
    //     for f2 in &formulas2 {
    //         theorem(f1.to_string(), f2.to_string(), true);
    //     }
    // }
    // test_fn();

    // let mut f1: ModalFormula = build_formula("(not q) and (diamond q) and (not p) and (diamond p)").unwrap();
    // let mut f2: ModalFormula = build_formula("not p").unwrap();
    // let mut f3: ModalFormula = build_formula("((p) and (diamond (not p))) and ((q) and (diamond (not q))) ").unwrap();
    // let mut f3: ModalFormula = build_formula("(box p) and (diamond r) and (box q)").unwrap();
    //
    // let args: Vec<String> = env::args().collect();
    // let atoms = vec![f1.get_atoms().len(), f2.get_atoms().len()];
    // let mut num = *atoms.iter().max().unwrap();
    // if args.len() > 1 {
    //     let num_str = &args[1]; 
    //     num = num_str.parse::<i32>().unwrap() as usize; 
    // }
    // let prop_set = generate_propset(num); 
    // let universe = generate_universe(prop_set.clone());

    // REVISION /////////////////////////////////////////////////////////////////////////
    // let r1 = Revision::new(f1.clone(),f2.clone(),universe.clone());
    // verbose_output(&r1);
    // let output = r1.output;
    
    // UPDATE ///////////////////////////////////////////////////////////////////////////
    // let update = Update::new(f1.clone(), f2.clone(), universe.clone());
    // update.print_output();
    // let output = update.output();

    // TEST ///////////////////////////////////////////////////////////////////////////
    // test_formula(output, f2.clone(), f3.clone(),universe, true);
    // //
    // let m = get_models(f1, universe);
    // for i in &m{
    //     println!("{}", i);
    // }

    // let m = get_models(f2, universe);
    // println!("WANTED P_MODELS");
    // for i in &m{
    //     println!("{}", i);
    // }
    //
    // println!("NO WANTED P_MODES");
    // let d = vec_difference(&r1.input_set, &m);
    // for i in d {
    //     println!("{}", i);
    // }
    // verbose_output(&r1);
    // test_formula(r1, f3);
    // let r2 = Revision::new(f1.clone(),f2.clone(),universe.clone());
    // verbose_output(&r1);
    // let o2 = r1.output;
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

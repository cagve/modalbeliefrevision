use std::collections::HashSet;

use operation::{generate_z, projection_pointed, theorem};
use s5rust::{modal::*, parser::build_formula};
use utils::{generate_models, generate_propset_from_atoms, generate_propset_from_n, generate_s5models, generate_valuations};

use crate::semantic::*; 
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
    // let input_set = get_models(input, universe);
    // let remain = vec_difference(&input_set, &output);
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
    //  for model in remain.clone()  {
    //     flag2 = check_pointed_model(&f3, &model);
    //     if debug{ 
    //         if flag2 {
    //             r2 = flag2;
    //             println!("#>> {} => {:?}", model, flag2); 
    //         } else {
    //             println!(" >> {} => {:?}", model, flag2);
    //         }
    //     }
    // }

    // if r1 && !r2 {
    //      println!("{} * {} ≡ {}", rev.phi, rev.mu, f3);
    // }else{
    //      println!("{} * {} ≢ {}", rev.phi, rev.mu, f3);
    // }
    return flag;
}

// fn simple_output(revision: &Revision){
//     revision.beauty("formula");
//     revision.beauty("distance");
//     revision.beauty("base_set");
//     revision.beauty("input_set");
//     revision.beauty("output");
// }
//
// fn verbose_output(revision: &Revision){
//     revision.beauty("formula");
//     print!("D= ");
//     revision.beauty("distance");
//     revision.beauty("base_set");
//     revision.verbose_beauty("input_set");
//     revision.beauty("output");
// }

fn main() {
    // let f1 = "(box p) implies (box q)".to_string();
    // let phi: ModalFormula = build_formula(&f1).unwrap();
    //
    // let atoms = "pq".to_string();
    // let universe = generate_valuations(&atoms);
    // let models = generate_models(&universe);
    // let s5models = generate_s5models(&universe);
    // let phimodels = get_phi_models(&phi, &universe);
    // println!("propset = {:?}", atoms);
    // println!("universe = {:?}", universe);
    // println!("models = {:?}", models);
    // println!("S5 models ");
    // for s5model in phimodels{
    //     println!(" M={}, A={} ", s5model, s5model.atoms());
    // }
    //
    // let m = vec!["p".to_string(), "".to_string()];
    // let n = vec!["q".to_string(), "".to_string()];
    // let s1_model:HashSet<Valuation> = m.into_iter().collect();
    // let s2_model:HashSet<Valuation> = n.into_iter().collect();
    // let s1_world = "pq".to_string();
    // let s2_world = "qp".to_string();
    // let s1 = S5PointedModel::new(s1_model, s1_world);
    // let s2 = S5PointedModel::new(s2_model, s2_world);
    // let v1 = vec![s1];
    // let v2 = vec![s2];
    // let u = generate_z(v1,v2, "p".to_string(), "q".to_string());
    // 
    // for i in u {
    //     println!("{}", i);
    // }

    // let formulas1 = vec!["box(p and diamond(q)) implies diamond(p and box(q))".to_string(),
    // "diamond(p and box(q)) implies box(p implies diamond(q))".to_string(),
    // "box(p implies diamond(q)) and diamond(p or q)".to_string(),
    // "diamond(p and box(q)) implies box(diamond(p) implies q)".to_string(),
    // "box(diamond(p) implies diamond(q)) implies diamond(p implies box(q))".to_string(),
    // "diamond(box(p) and diamond(q)) implies box(diamond(p) implies q)".to_string(),
    // "box(p implies diamond(box(q))) and diamond(p and q)".to_string(),
    // "diamond(box(p implies q)) implies box(diamond(p implies q))".to_string(),
    // "box(diamond(p) and diamond(q)) implies diamond(p and q)".to_string(),
    // "diamond(p implies box(q)) implies box(p implies diamond(q))".to_string(),
    // ];
    //
    // let formulas2 = vec![
    //     "box(r) and diamond(r)".to_string(),
    //     "diamond(r) implies box(r)".to_string(),
    //     "box(r) implies diamond(r)".to_string(),
    //     "diamond(r) and box(r)".to_string(),
    //     "diamond(r or r)".to_string(),
    //     "box(r and diamond(r))".to_string(),
    //     "diamond(box(r))".to_string(),
    //     "box(diamond(r))".to_string(),
    //     "diamond(r) or box(r)".to_string(),
    //     "box(r) implies diamond(box(r))".to_string(),
    // ];



    // let f1 = "box(p and diamond(q)) implies diamond(p and box(q))".to_string();
    // let f2 = "(box r) implies (diamond r) ".to_string();
    let f1 = "(box p) implies (diamond q)".to_string();
    let f2 = "(box r)".to_string();
    let mut phi: ModalFormula = build_formula(&f1).unwrap();
    let mut psi: ModalFormula = build_formula(&f2).unwrap();

    // let t = theorem(f1.to_string(), f2.to_string(), f2.to_string(), true);
    // println!("RESULT = {}", t);
    let universe1 = generate_valuations(&"pq".to_string());
    let universe2 = generate_valuations(&"r".to_string());
    let t = get_phi_models(&phi, &universe1);
    let u = get_phi_models(&psi, &universe2);

    for m in &t{
        println!("{}", m);
    }

    for m in &u{
        println!("{}", m);
    }
    let m = vec!["pqr".to_string()];
    let s1_model:HashSet<Valuation> = m.into_iter().collect();
    // let s2_model:HashSet<Valuation> = n.into_iter().collect();
    let s1_world = "pqr".to_string();
    // let s2_world = "qp".to_string();
    let s1 = S5PointedModel::new(s1_model, s1_world);
    // let s2 = S5PointedModel::new(s2_model, s2_world);
    let v1 = vec![s1.clone()];
    let m = projection_pointed(&s1, &"r".to_string());
    println!("{}", m);
    let flag = m.contained(&u);
    println!("flag = {:?}", flag);
    // let v2 = vec![s2];

    // let mut flag = true;
    // for f1 in &formulas1 {
    //     if flag{
    //         for f2 in &formulas2 {
    //             flag = theorem(f1.to_string(), f2.to_string(), false);
    //             if !flag{
    //                 println!(">>>>>> Debuggin result <<<<<<<", );
    //                 println!("phi = {}", f1);
    //                 println!("psi = {}", f2);
    //                 break;
    //             }
    //         }
    //     }
    // }
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
// fn example_paper (example:Vec<(&str,&str)>, universe: Vec<String>){
//     example.iter().for_each(|x|{
//         let mut f1: ModalFormula = build_formula(x.0).unwrap();
//         let mut f2: ModalFormula = build_formula(x.1).unwrap();
//         let revision = Revision::new(f1.clone(),f2.clone(),universe.clone());
//         let dmin = revision.clone().distance;
//         let phi_latex = to_latex(revision.phi);
//         let mu_latex = to_latex(revision.mu);
//         let mut output = format!("
// \\begin{{table}}[ht!]
// \\centering
// Example: $({phi_latex})*({mu_latex})$ \\\\
// Min $\\Delta$: ${dmin}$ \\\\
// \\begin{{tabular}}{{ | c | c | c |}}
// \\hline
// Models of ${phi_latex}$ & Models of ${mu_latex}$ & Distance \\\\
// \\hline
//         ");
//         revision.input_set.iter().for_each(|x| {
//             // let closest_pointed = closest_pointed_model(&revision.base_set, x);
//             // let ph_model = closest_pointed.to_latex().to_string();
//             let mu_model = x.to_latex().to_string();
//             // let d = min_distance(&revision.base_set, x);
//             if d == dmin{
//                 output.push_str("\\rowcolor{{green}}");
//             }
//             output.push_str(&format!("{ph_model} & {mu_model} & {d} \\\\ \n").to_string());
//         });
//         output.push_str(&format!("\\hline"));
//         output.push_str(&format!("\\end{{tabular}}"));
//         output.push_str(&format!("\\end{{table}}"));
//         output.push_str(&format!("%==================="));
//         println!("{}", output);
//         // verbose_output(&revision);
//     });
//
//
// }


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



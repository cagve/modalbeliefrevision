use std::collections::HashSet;

use crate::semantic::*;
use itertools::Itertools;
use s5rust::modal::ModalFormula;

pub fn generate_propset_from_atoms(atoms:&String) -> String {
    let prop_set: HashSet<char> = atoms.chars().collect();
    let string: String = prop_set.into_iter().collect();
    return string;
}

pub fn generate_propset_from_n(n: usize) -> String {
    let mut prop_set: Vec<char> = Vec::with_capacity(n);
    for i in 0..n { // start at p
        prop_set.push((i as u8 + b'p') as char);
    } 
    let string: String = prop_set.into_iter().collect();
    return string;
}

pub fn generate_valuations(atoms: &String) -> Vec<Valuation> {

    // Convertimos el conjunto de átomos a un vector para poder indexar sus elementos
    let atoms_vec: Vec<char> = atoms.chars().collect();
    let pow = powerset(&atoms_vec);
    let valuations = pow.iter()
        .map(|x| String::from_iter(x))
        .collect();
    return valuations;
}

pub fn generate_models(universe:&Vec<Valuation>) -> Vec<HashSet<Valuation>>{
    let powerset = powerset(universe);
    let vec_of_hashsets: Vec<HashSet<String>> = powerset
        .into_iter()
        .map(|inner_vec| inner_vec.into_iter().collect()) 
        .collect(); 
    return vec_of_hashsets;
}

pub fn generate_s5models(universe:&Vec<Valuation>) -> Vec<S5PointedModel>{
    let mut s5models = Vec::new();
    let models = generate_models(universe);
    models.iter()
        .for_each(|model|{
            for valuation in model.iter() {
                let s5model =  S5PointedModel::new(model.clone(), valuation.to_string());
                s5models.push(s5model);
            }
        });
    return s5models;
}

pub fn powerset<T: Clone>(a: &Vec<T>) -> Vec<Vec<T>> {
    a.iter().fold(vec![vec![]], |mut p, x| {
        let i = p.clone().into_iter()
            .map(|mut s| {s.push(x.clone()); s});
        p.extend(i); p})
}

pub fn union(set1: &HashSet<Valuation>, set2: &HashSet<Valuation>) -> HashSet<Valuation> {
    set1.union(set2).cloned().collect()
}

pub fn intersection(set1: &HashSet<Valuation>, set2: &HashSet<Valuation>) -> HashSet<Valuation> {
    set1.intersection(set2).cloned().collect()
}

pub fn difference(set1: &HashSet<Valuation>, set2: &HashSet<Valuation>) -> HashSet<Valuation> {
    set1.difference(set2).cloned().collect()
}

pub fn symmetric_difference(set1: &HashSet<Valuation>, set2: &HashSet<Valuation>) -> HashSet<Valuation> {
    set1.symmetric_difference(set2).cloned().collect()
}

pub fn powerset_hash(set: &HashSet<Valuation>) -> Vec<HashSet<Valuation>> {
    let mut result = Vec::new();
    let elements: Vec<Valuation> = set.iter().cloned().collect();
    let n = elements.len();

    // Generate all possible subsets (2^n subsets)
    for i in 0..(1 << n) {
        let mut subset = HashSet::new();
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                subset.insert(elements[j].clone());
            }
        }
        result.push(subset);
    }
    result
}




pub fn get_atoms_from_fromula(f: &ModalFormula) -> Valuation  {
    let str_f:Vec<String> = f.get_atoms().iter().map(|x| x.to_string()).collect();
    return str_f.concat();
}

pub fn s5veceq(vec1:&Vec<S5PointedModel>, vec2:&Vec<S5PointedModel>) -> bool{
    if vec1.len() != vec2.len(){
        return false
    }
    let mut flag1=false;
    let mut flag2=false;

    for s5 in vec1 {
        flag1 = s5.contained(vec2);
        if flag1 == true{
            break;
        }
    }
    for s5 in vec1 {
        flag2 = s5.contained(vec1);
        if flag2 == true{
            break;
        }
    }
    return flag1 && flag2;
}

// pub fn get_universe_from_formula(f: &ModalFormula) -> Vec<String> {
//     let atoms = get_atoms_from_fromula(f);
//     let atomsstr = String::from_iter(atoms);
//     return generate_universe(atomsstr);
// }
//
// pub fn contains_b_in_a(a: Vec<Vec<String>>, b: Vec<String>) -> bool {
//     // Convert `b` to a HashSet
//     let b_set: HashSet<_> = b.into_iter().collect();
//
//     // Check if `b_set` is equal to any `Vec<String>` in `a`, converted to a HashSet
//     a.iter().any(|vec| {
//         let a_set: HashSet<_> = vec.iter().cloned().collect();
//         a_set == b_set
//     })
// }
//
//
// pub fn same_models(mut vec1: Vec<Vec<String>>, mut vec2: Vec<Vec<String>>) -> bool {
//     for m in &mut vec1 {
//         m.into_iter().for_each(|s| {
//             let mut chars: Vec<char> = s.chars().collect();
//             chars.sort();  // Sort the characters in the string
//             *s = chars.into_iter().collect();  // Rebuild the string from the sorted characters
//         });
//         m.sort();
//     }
//
//
//     for n in &mut vec2 {
//         n.into_iter().for_each(|s| {
//             let mut chars: Vec<char> = s.chars().collect();
//             chars.sort();  // Sort the characters in the string
//             *s = chars.into_iter().collect();  // Rebuild the string from the sorted characters
//         });
//         n.sort();
//     }
//
//     let mut vec1_sorted = vec1;
//     let mut vec2_sorted = vec2;
//     
//     vec1_sorted.sort();
//     vec2_sorted.sort();
//     
//     vec1_sorted == vec2_sorted
// }
//
// pub fn projection(vec1: &Vec<String>, P:&String ) -> Vec<String>{
//     let mut hs = HashSet::new();
//
//     // Iteramos sobre cada palabra en W
//     for w in vec1 {
//         // Verificamos si la palabra tiene algún carácter en común con P
//         if w.chars().any(|c| P.contains(c)) {
//             hs.insert(w.clone());
//         }
//     }
//
//     let result = hs.into_iter().collect();
//     return result;
// }
//
//
//
// pub fn same_s5model(s51: &S5PointedModel, s52: &S5PointedModel) -> bool{
//     let mut s1 = s51.clone();
//     let mut s2 = s52.clone();
//
//     // let f1 = s1.model == s2.model;
//     // let f2 = s1.world == s2.world;
//     // return f1 && f2;
//     return true
// }
//
// // pub fn s5_contains(mut s5:S5PointedModel, vecs5:&HashSet<S5PointedModel>) -> bool{
// //     let mut copy = vecs5.clone();
// //     copy.iter_mut()
// //         .for_each(|x| 
// //                  x.sort());
// //     s5.sort();
// //     for m in copy {
// //         if same_s5model(&s5, &m){
// //             return true
// //         }
// //     }
// //     return false;
// // }
// //
// // pub fn s5veceq(vec1:&HashSet<S5PointedModel>, vec2:&HashSet<S5PointedModel>) -> bool{
// //     if vec1.len() != vec2.len(){
// //         return false
// //     }
// //     let mut flag1=false;
// //     let mut flag2=false;
// //
// //     for s5 in vec1 {
// //         flag1 = s5_contains(s5.clone(), &vec2);
// //         if flag1 == true{
// //             break;
// //         }
// //     }
// //     for s5 in vec2 {
// //         flag2 = s5_contains(s5.clone(), &vec1);
// //         if flag2 == true{
// //             break;
// //         }
// //     }
// //     return flag1 && flag2;
// // }
// //
// // pub fn sort_string(str: String) -> String{
// //     let mut l: Vec<char> = str.chars().collect();
// //     l.sort();
// //     let j: String = l.into_iter().collect();
// //     return j;
// // }

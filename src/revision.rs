// use crate::semantic::*; //TODO refactor name
// use crate::distance::*; //TODO refactor name
// use s5rust::modal::*;
// use s5rust::prop::*;
// use crate::semantic::S5PointedModel;
//
// #[derive(Clone)]
// pub struct DalalRevision {
//     pub phi: PropFormula,
//     pub mu: PropFormula,
//     pub universe:Vec<String>,
//     pub base_set: Vec<String>,
//     pub input_set: Vec<String>,
//     pub output:Vec<String>,
//     pub distance:i32
// }
//
//
//
// #[derive(Clone)]
// pub struct Revision {
//     pub phi: ModalFormula,
//     pub mu: ModalFormula,
//     pub universe:Vec<String>,
//     pub base_set: Vec<S5PointedModel>,
//     pub input_set: Vec<S5PointedModel>,
//     pub output:Vec<S5PointedModel>,
//     pub distance:Lexicographic
// }
//
//
// impl Revision {
//     pub fn new( phi: ModalFormula, mu: ModalFormula, universe:Vec<String>) -> Self {
//         let base_set = get_models(phi.clone(), universe.clone());
//         let input_set = get_models(mu.clone(), universe.clone());
//         let output = closest_set_pointed(&base_set, &input_set);
//         let distance = min_distance(&base_set, output.get(0).unwrap());
//
//         Self {
//             phi,
//             mu,
//             universe,
//             base_set,
//             input_set,
//             output,
//             distance,
//         }
//     }
//     
//     pub fn _to_string(&self) -> String{
//         let s = format!("{} * {}", self.phi, self.mu); 
//         return s;
//     }
//
//     pub fn beauty_distance(&self) -> String{
//         return format!("{} * {} = {} ", self.phi, self.mu, self.distance.to_string());
//     }
//
//     pub fn debug(&self, comp: &str) {
//         match comp {
//             "formula"    => println!("{} * {}", self.phi, self.mu),
//             "output"    => {
//                 let mut id = 0;
//                 println!("Output models: {}", self.output.len());
//                 self.output.iter().for_each(|x| {
//                     let m1 = closest_pointed_model(&self.base_set, x);
//                     let d = distance_pointed_to_pointed(&m1, x);
//                     println!("> Model {}:  {}",id, x);
//                     println!("  | closest: {}", m1);
//                     println!("  | D={}", d);
//                     id = id+1;
//                 })
//             },
//             "base_set"  => {
//                 println!("Base models: {}", self.base_set.len());
//                 let mut id = 0;
//                 self.base_set.iter().for_each(|x| {
//                     let m1 = closest_pointed_model(&self.input_set, x);
//                     let d = distance_pointed_to_pointed(&m1, x);
//                     println!("> Model {}:  {}", id, x);
//                     println!("  | closest: {}", m1);
//                     println!("  | D={}", d);
//                     id = id+1;
//                 });
//             },
//             "input_set" => {
//                 println!("Input models: {}", self.input_set.len());
//                 let mut id = 0;
//                 self.input_set.iter().for_each(|x|{
//                     let m1 = closest_pointed_model(&self.base_set, x);
//                     let d = distance_pointed_to_pointed(&m1, x);
//                     println!("> Model {}:  {}", id, x);
//                     println!("  | closest: {}", m1);
//                     println!("  | D={}", d);
//                     id = id+1;
//                 })
//             },
//             "debug"     => {
//                 println!("{} * {} = ", self.phi, self.mu);
//                 self.output.iter().for_each(|x| {
//                     println!("| {}", x);
//                 })
//             },
//             _           => println!("Error") // TODO error managment
//         };
//
//     }
//     pub fn beauty(&self, comp: &str) {
//         match comp {
//             "distance"    => println!("{}", self.distance),
//             "formula"    => println!("{} * {}", self.phi, self.mu),
//             "output"    => {
//                 println!("Output models: {}", self.output.len());
//                 self.output.iter().for_each(|x| println!("{}", x))
//             },
//             "base_set"  => {
//                 println!("Base models: {}", self.base_set.len());
//                 self.base_set.iter().for_each(|x| println!("{}", x))
//             },
//             "input_set" => {
//                 println!("Input models: {}", self.input_set.len());
//                 self.input_set.iter().for_each(|x| println!("{}", x))
//             },
//             "debug"     => {
//                 println!("{} * {} = ", self.phi, self.mu);
//                 self.output.iter().for_each(|x| {
//                     println!("| {}", x);
//                 })
//             },
//             _           => println!("Error") // TODO error managment
//         };
//
//     }
//     pub fn verbose_beauty(&self, comp: &str) {
//         match comp {
//             "distance"    => println!("{}", self.distance),
//             "formula"    => println!("{} * {}", self.phi, self.mu),
//             "output"    => {
//                 println!("Output models: {}", self.output.len());
//                 self.output.iter().for_each(|x| println!("{}", x))
//             },
//             "base_set"  => {
//                 println!("Base models: {}", self.base_set.len());
//                 self.base_set.iter().for_each(|x| println!("{}", x))
//             },
//             "input_set" => {
//                 println!("Input models: {}", self.input_set.len());
//                 self.input_set.iter().for_each(|x| {
//                     let d = min_distance(&self.base_set, x);
//                     let m = closest_pointed_model(&self.base_set, x);
//                     println!("{} at distance {} from model: {}", x, d, m)
//                 })
//             },
//             "debug"     => {
//                 println!("{} * {} = ", self.phi, self.mu);
//                 self.output.iter().for_each(|x| {
//                     println!("| {}", x);
//                 })
//             },
//             _           => println!("Error") // TODO error managment
//         };
//
//     }
//
//     pub fn verbose(&self){
//         println!("{} * {} = ", self.phi, self.mu);
//         self.output.iter().for_each(|x| {
//             let d = min_distance(&self.base_set, x);
//             println!("| {}. D={}", x, d);
//             let closest = minimal_set_of_pointed_model(&self.base_set, x);
//             closest.iter().for_each(|y|{
//                 println!("| | {}", y);
//             })
//         })
//     }


// }

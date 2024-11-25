use crate::geometry::{AllPrimitives, Primitives};
use crate::geometry::base::*;
use crate::geometry::*;
use serde::Serialize;
use serde_json::{self, Result, Value};
use std::fs;

// top level load metho that pass a file path in str
pub fn deserialize_geometry(geometry_file: &str) -> () {
  let s = fs::read_to_string(geometry_file).unwrap();
  let v: Value = serde_json::from_str(&s).expect("parse json");
  println!("s_read = {:?}", v);
  println!("S_read_val = {:?}", v["type"]);

  // deserialize_geometric_primitive(&v);
}

// pub fn deserialize_geometric_primitive(v: &Value) -> impl GeometricPrimitive {
//   let type_str = v["type"].as_str().unwrap();
//   deserialize_match(&v, &type_str)
// }

// pub fn deserialize_match<A, B>(v: &Value, type_str: &str) -> AllPrimitives<A, B> 
// where A: GeometricPrimitive,
//       B: GeometricPrimitive {
//   println!("{:?}", type_str);
//   match type_str {
//     // Booleans
//     // "Difference"   => deserialize_boolean(&v["left"], &v["right"], type_str),
//     // "Intersection" => deserialize_boolean(&v["left"], &v["right"], type_str),
//     "Union"        => deserialize_boolean(&v["left"], &v["right"], type_str),
//     // Primitives 
//     // "Torus"        => serde_json::
//     // "Torus"        => Torus::new(v["a"].as_f64().unwrap(), v["c"].as_f64().unwrap()),
//     "Torus"        => deserialize_primitive(v, type_str),
//     _              => panic!("wtf")
//   }
// }

// pub fn deserialize_boolean<A, B>(left: &Value, right: &Value, type_str: &str) -> AllPrimitives<A, B> 
// where A: GeometricPrimitive, 
//       B: GeometricPrimitive {
//   let left_type_str = left["type"].as_str().unwrap();
//   let right_type_str = right["type"].as_str().unwrap();

//   let left = deserialize_match(left, &left_type_str);
//   let right = deserialize_match(right, &right_type_str);

//   match type_str {
//     // "Difference"   => Difference::new(left, right),
//     // "Intersection" => Intersection::new(left, right),
//     "Union"        => Union::new(left, right),
//     _              => panic!("wtf") 
//   }
// }

// pub fn deserialize_primitive<A, B>(v: &Value, type_str: &str) -> AllPrimitives<A, B> 
// where A: GeometricPrimitive,
//       B: GeometricPrimitive {
//   match type_str {
//     "Torus" => {
//       Primitives::Torus(
//         Torus::new(v["a"].as_f64().unwrap(), v["c"].as_f64().unwrap())
//       )
//     },
//     _       => panic!("wtf")
//   }
// }

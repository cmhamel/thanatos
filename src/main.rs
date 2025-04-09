use clap::{Parser, Subcommand};
use pyo3::prelude::*;
use serde_json::{self, Value};
use std::fs;
use std::time::Instant;
use thanatos::geometry::*;
use thanatos::mesh::*;


// fn main() -> Result<()> {
//   // setup a geometry
//   let e1: Primitive = Primitive::Torus(Torus::new(0.5, 2.5));
//   let r1: Primitive = rotate(e1.clone(), 'x', 45.);
//   let r2: Primitive = rotate(e1.clone(), 'y', 45.);
//   let g2: Primitive = Primitive::Union(Union::new(e1, r1));
//   let g3: Primitive = Primitive::Union(Union::new(g2, r2));

//   // // spherical annulus
//   // // let s1 = Sphere::new(2.);
//   // // let s2 = Sphere::new(1.);
//   // // let g3 = Difference::new(s2, s1);

//   // serialization
//   println!("Geometry = {:?}", g3);

//   let s = serde_json::to_string(&g3)?;
//   let _ = fs::write("example.json", &s);

//   let g3: Primitive = serde_json::from_str(&s)?;
//   println!("S = {:?}", s);

//   // // let e1 = &s["left"]["left"];
//   // let temp = serde_json::to_string(&s["left"]["left"])?;
//   // let e1: Primitive = serde_json::from_str(&temp)?;

//   // // println!("S = {:?}", s);
//   // // println!("S = {:?}", s["left"]["left"]);

//   // // stl meshing
//   // let n_samples = 64;
//   // let m = mesh(g3, n_samples, n_samples, n_samples, 1., 0.25);
//   // m.export_stl("example.stl");

//   // voxel meshing
//   let n_samples = 128;
//   // let voxels = VoxelMesh::new(g3, n_samples, n_samples, n_samples);
//   let _ = voxel_mesh(g3, n_samples, n_samples, n_samples, "mesh.inp");
//   // println!("Voxels = {:?}", voxels);
//   // println!("Voxels = {:?}", voxels.voxels.sum());
//   Ok(())
// }

#[derive(Clone, Debug, Parser)]
#[command(arg_required_else_help = true, version)]
struct CLIArgs {
  #[command(subcommand)]
  command: Option<Commands>
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
  #[command(about = "Dump the json file with saved csg information")]
  Dump {
    #[arg(long, short, value_name = "FILE")]
    input: String
  },
  #[command(about = "Generate a hex mesh")]
  HexMesh {
    #[arg(long, short, value_name = "FILE")]
    input: String,
    #[arg(long, short, value_name = "FILE")]
    output: String,
    /// Number of voxels in the x-direction
    #[arg(long, short = 'x', value_name = "NEL")]
    nelx: usize,
    /// Number of voxels in the y-direction
    #[arg(long, short = 'y', value_name = "NEL")]
    nely: usize,
    /// Number of voxels in the z-direction
    #[arg(long, short = 'z', value_name = "NEL")]
    nelz: usize
  },
  #[command(about = "Generate an SPN file")]
  Spn {
    #[arg(long, short, value_name = "FILE")]
    input: String,
    #[arg(long, short, value_name = "FILE")]
    output: String,
    /// Number of voxels in the x-direction
    #[arg(long, short = 'x', value_name = "NEL")]
    nelx: usize,
    /// Number of voxels in the y-direction
    #[arg(long, short = 'y', value_name = "NEL")]
    nely: usize,
    /// Number of voxels in the z-direction
    #[arg(long, short = 'z', value_name = "NEL")]
    nelz: usize
  },
  #[command(about = "Generate an STL file")]
  StlMesh {
    #[arg(long, short, value_name = "FILE")]
    input: String,
    #[arg(long, short, value_name = "FILE")]
    output: String,
    /// Number of voxels in the x-direction
    #[arg(long, short = 'x', value_name = "NEL")]
    nelx: usize,
    /// Number of voxels in the y-direction
    #[arg(long, short = 'y', value_name = "NEL")]
    nely: usize,
    /// Number of voxels in the z-direction
    #[arg(long, short = 'z', value_name = "NEL")]
    nelz: usize
  }
}

fn dump(input_file: &str) -> () {
  let time = Instant::now();
  let json_str: String = fs::read_to_string(input_file).expect("Can't find file");
  let v: Value = serde_json::from_str(&json_str).expect("Problem reading json");
  println!("Dumping file contents from {:?}", input_file);
  println!("{:?}", v);
  println!(
      "           \x1b[1;93m⤷ Dumped file in \x1b[0m {:?}",
      time.elapsed()
  );
}

// fn hex_mesh(input: &str, output: &str, nelx: usize, nely: usize, nelz: usize) -> () {
//   let time = Instant::now();
//   let primitive = Primitive::from_json_file(input);
//   println!(
//     "           \x1b[1;93m⤷ Read primitive from file in \x1b[0m {:?}",
//     time.elapsed()
//   );  
//   let time = Instant::now();
//   let _ = voxel_mesh(primitive, nelx, nely, nelz, output);
//   println!(
//     "           \x1b[1;93m⤷ Hex mesh generated in \x1b[0m {:?}",
//     time.elapsed()
//   );  
// }

fn stl_mesh(input: &str, output: &str, nelx: usize, nely: usize, nelz: usize) -> () {
  let primitive = Primitive::from_json_file(input);
  let m = mesh(primitive, nelx, nely, nelz, 1., 0.25);
  m.export_stl(output);
}

#[pyfunction]
fn write_spn(input_file: &str, output: &str, nelx: usize, nely: usize, nelz: usize) -> () {
  let time = Instant::now();
  let primitive = Primitive::from_json_file(input_file);
  println!(
    "           \x1b[1;93m⤷ Read primitive from file in \x1b[0m {:?}",
    time.elapsed()
  );
  let voxels = MyVoxels::new(primitive, nelx, nely, nelz);
  let time = Instant::now();
  let _ = write_voxels_to_spn(&voxels.data, output);
  println!(
    "           \x1b[1;93m⤷ Wrote SPN file in \x1b[0m {:?}",
    time.elapsed()
  );
}

fn main() -> ()   {
  let args = CLIArgs::parse();

  match args.command {
    Some(Commands::Dump {
      input
    }) => dump(&input),
    // Some(Commands::HexMesh {
    //   input,
    //   output,
    //   nelx,
    //   nely,
    //   nelz
    // }) => hex_mesh(&input, &output, nelx, nely, nelz),
    Some(Commands::Spn {
      input,
      output,
      nelx,
      nely,
      nelz
    }) => write_spn(&input, &output, nelx, nely, nelz),
    Some(Commands::StlMesh {
      input,
      output,
      nelx,
      nely,
      nelz
    }) => stl_mesh(&input, &output, nelx, nely, nelz),
    _  => println!("wtf")
  }
}

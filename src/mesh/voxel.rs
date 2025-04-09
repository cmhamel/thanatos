use crate::geometry::Primitive;
// use automesh::Voxels;
use ndarray::Array3;
use ndarray::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Error, Write};

type VoxelData = Array3<u8>;

pub struct MyVoxels {
  pub data: VoxelData
}

impl MyVoxels {
  pub fn new(primitive: Primitive, x_count: usize, y_count: usize, z_count: usize) -> Self {
    let bb = primitive.get_bbox();
    let xo = bb.0[0];
    let xf = bb.1[0];
    let yo = bb.0[1];
    let yf = bb.1[1];
    let zo = bb.0[2];
    let zf = bb.1[2];

    let dx = (xf - xo) / x_count as f64;
    let dy = (yf - yo) / y_count as f64;
    let dz = (zf - zo) / z_count as f64;

    let voxels = (0..x_count)
      .into_par_iter()
      .map(|x| {
        (0..y_count)
          .map(|y| {
            (0..z_count)
              .map(|z| {
                let x_c = xo + dx * x as f64;
                let y_c = yo + dy * y as f64;
                let z_c = zo + dz * z as f64;
                let sdf = primitive.get_sdf(x_c, y_c, z_c);
                match sdf {
                  x if x < 0.  => 1 as u8,
                  x if x >= 0. => 0 as u8,
                  _            => todo!("NAN or INF encountered wtf?")
                }
              })
              .collect::<Vec<u8>>()
          })
          .flatten()
          .collect::<Vec<u8>>()
      })
      .flatten()
      .collect::<Vec<u8>>();

    // now convert to an ndarray
    let mut data = VoxelData::zeros((x_count, y_count, z_count));
    data.axis_iter_mut(Axis(2))
      .enumerate()
      .for_each(|(k, mut data_k)| {
        data_k
          .axis_iter_mut(Axis(1))
          .enumerate()
          .for_each(|(j, mut data_jk)| {
            data_jk.iter_mut().enumerate().for_each(|(i, data_ijk)| {
              *data_ijk = voxels[i + x_count * j + x_count * y_count * k]
            })
          })
      });
    
    MyVoxels {
      data: data
    }
  }
}

pub fn write_voxels_to_spn(data: &VoxelData, file_path: &str) -> Result<(), Error> {
  let mut file = BufWriter::new(File::create(file_path)?);
  data.axis_iter(Axis(2)).try_for_each(|entry_2d| {
    entry_2d
      .axis_iter(Axis(1))
      .flatten()
      .try_for_each(|entry| writeln!(file, "{}", entry))
  })
}

// pub fn voxel_mesh(
//   primitive: Primitive, 
//   x_count: usize, y_count: usize, z_count: usize,
//   mesh_file: &str
// ) -> () {
//   // clean this up
//   let bb = primitive.get_bbox();
//   let xo = bb.0[0];
//   let xf = bb.1[0];
//   let yo = bb.0[1];
//   let yf = bb.1[1];
//   let zo = bb.0[2];
//   let zf = bb.1[2];

//   let dx = (xf - xo) / x_count as f64;
//   let dy = (yf - yo) / y_count as f64;
//   let dz = (zf - zo) / z_count as f64;

//   let my_voxels = MyVoxels::new(primitive.clone(), x_count, y_count, z_count);
//   let _ = write_voxels_to_spn(&my_voxels.data, "temp.spn");
//   let n_voxels: [usize; 3] = [x_count, y_count, z_count];
//   let voxels = Voxels::from_spn("temp.spn", n_voxels).expect("voxels from spn failed");
//   let bb = primitive.get_bbox();
//   let x_scale = bb.1[0] - bb.0[0];
//   let y_scale = bb.1[1] - bb.0[1];
//   let z_scale = bb.1[2] - bb.0[2];
//   let scale: [f64; 3] = [x_scale, y_scale, z_scale];
//   let translate: [f64; 3] = [-bb.0[0], -bb.0[1], -bb.0[2]];

//   // deleting void space "0" id voxels
//   let voxels_temp = (0..x_count)
//       .into_par_iter()
//       .map(|x| {
//         (0..y_count)
//           .map(|y| {
//             (0..z_count)
//               .map(|z| {
//                 let x_c = xo + dx * x as f64;
//                 let y_c = yo + dy * y as f64;
//                 let z_c = zo + dz * z as f64;
//                 let sdf = primitive.get_sdf(x_c, y_c, z_c);
//                 match sdf {
//                   x if x < 0.  => 1 as u8,
//                   x if x >= 0. => 0 as u8,
//                   _            => todo!("NAN or INF encountered wtf?")
//                 }
//               })
//               .collect::<Vec<u8>>()
//           })
//           .flatten()
//           .collect::<Vec<u8>>()
//       })
//       .flatten()
//       .collect::<Vec<u8>>();
//   let remove = voxels_temp
//     .into_iter()
//     .enumerate()
//     .filter(|&(i, _)| i == 0 as usize)
//     .map(|(_, e)| e)
//     .collect::<Vec<u8>>();

//   let fem = voxels.into_finite_elements(Some(remove), &scale, &translate).expect("into fem failed");

//   let _ = fem.write_inp(mesh_file);
// }

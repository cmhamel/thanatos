
use crate::geometry::GeometricPrimitive;
use crate::Point;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct VoxelMesh<T: GeometricPrimitive> {
    primitive: T,
    x_count: usize,
    y_count: usize,
    z_count: usize
}

impl<T> VoxelMesh<T> 
where T: GeometricPrimitive + Sync {
    pub fn new(primitive: T, x_count: usize, y_count: usize, z_count: usize) -> Self {
        Self {
            primitive: primitive,
            x_count: x_count,
            y_count: y_count,
            z_count: z_count
        }
    }

    pub fn mesh(&self) {
        let bb = self.primitive.bounding_box();
        let min_point = bb.min;
        let max_point = bb.max;
        let dx = (max_point[0] - min_point[0]) / (self.x_count + 1) as f64;
        let dy = (max_point[1] - min_point[1]) / (self.y_count + 1) as f64;
        let dz = (max_point[2] - min_point[2]) / (self.z_count + 1) as f64;

        let voxels: Vec<usize> = (0..self.x_count)
            .into_par_iter()
            .map(|i| {
                (0..self.y_count)
                    .map(move |j| {
                        (0..self.z_count) 
                            .map(move |k| {
                                // let x = min_point[0] + (i as f64 + 0.5) * (max_point[0] - min_point[0]) / self.x_count as f64;
                                // let y = min_point[1] + (j as f64 + 0.5) * (max_point[1] - min_point[1]) / self.y_count as f64;
                                // let z = min_point[2] + (k as f64 + 0.5) * (max_point[2] - min_point[2]) / self.z_count as f64;
                                // let x = (i as f64 + 0.5) * dx;
                                // let y = (j as f64 + 0.5) * dy;
                                // let z = (k as f64 + 0.5) * dz;

                                // this isn't quite right. We want the center of the voxel
                                let x = min_point[0] + i as f64 * dx;
                                let y = min_point[1] + j as f64 * dy;
                                let z = min_point[2] + k as f64 * dz;
                                let p = Point::new(x, y, z);
                                if self.primitive.sdf(&p) <= 0. {
                                    1 as usize
                                } else {
                                    0 as usize
                                }
                            })
                    })
                    .flatten()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        
        // write voxels to file
        println!("Voxels = {:?}", voxels.clone().into_iter().sum::<usize>());

        let file = File::create("test_spn.spn").unwrap();
        let mut writer = BufWriter::new(file);

        for value in voxels {
            writeln!(writer, "{}", value);
        }
        writer.flush().unwrap();
    }   
}
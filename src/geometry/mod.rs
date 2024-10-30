mod base;
mod brick;
mod circle;
mod difference;
mod linear_extrusion;
mod rectangle;
mod sphere;
mod translation;
mod union;

// types
pub use base::{MapContainer, Point, TriangleFace};

// traits
pub use base::{BoundingBox, Frep, Primitive, Transform};

// // affine transformations
pub use translation::Translation;
pub use translation::translate;

// booleans
pub use difference::Difference;
pub use union::Union;

// geometries
pub use brick::Brick;
pub use circle::Circle;
pub use linear_extrusion::LinearExtrusion;
pub use rectangle::Rectangle;
pub use sphere::Sphere;

pub use base::save_csg_model;

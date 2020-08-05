mod object_data;
pub use object_data::*;

mod shape_builder;
pub use shape_builder::*;

pub mod shapes;

use ncollide3d::shape::Shape;

pub trait ObjectToShape {
    type ShapeType;
    fn to_shape(self) -> Self::ShapeType;
}

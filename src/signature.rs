use crate::Type;
use std::any::TypeId;

pub struct Signature {
    parameter: TypeId,
    result: TypeId,
    in_types: Vec<Type>,
}

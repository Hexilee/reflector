#![feature(
    unboxed_closures,
    fn_traits,
    specialization,
    const_fn,
    const_type_id,
    const_type_name
)]

mod attribute;
mod field;
mod func;
mod object;
mod signature;
mod r#type;
mod value;

pub use func::Func;
pub use object::Object;
pub use r#type::{Type, TypeKind};
pub use signature::Signature;

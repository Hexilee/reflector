// use crate::{Object, Type};
//
// pub struct ValueRef<'a> {
//     typ: Type,
//     obj: &'a dyn Object,
// }
//
// pub struct ValueMut<'a> {
//     typ: Type,
//     obj: &'a mut dyn Object,
// }
//
// pub struct Value {
//     typ: Type,
//     obj: Box<dyn Object>,
// }
//
// impl Value {
//     pub fn borrow(obj: &dyn Object) -> ValueRef {
//         ValueRef {
//             typ: obj.as_type(),
//             obj,
//         }
//     }
//     pub fn borrow_mut(obj: &mut dyn Object) -> ValueMut {
//         ValueMut {
//             typ: obj.as_type(),
//             obj,
//         }
//     }
//
//     pub fn boxed(obj: impl Object) -> Value {
//         let obj: Box<dyn Object> = Box::new(obj);
//         obj.into()
//     }
// }
//
// impl From<Box<dyn Object>> for Value {
//     fn from(obj: Box<dyn Object>) -> Self {
//         Value {
//             typ: obj.as_type(),
//             obj,
//         }
//     }
// }

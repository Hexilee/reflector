use std::any::{type_name, Any, TypeId};

#[derive(Debug, Clone)]
pub struct Type {
    pub id: TypeId,
    pub name: &'static str,
    pub kind: TypeKind,
}

impl Type {
    pub const fn new<T: Any>(kind: TypeKind) -> Self {
        Self {
            id: TypeId::of::<T>(),
            name: type_name::<T>(),
            kind,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Enum,
    Struct,
    Tuple,
    Func,
    Template,
    Unknown,
}

impl Default for TypeKind {
    fn default() -> Self {
        TypeKind::Unknown
    }
}

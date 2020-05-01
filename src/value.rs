use crate::{Func, Type, TypeKind};
use std::any::{Any, TypeId};
use std::ops::{Index, IndexMut};

pub trait Value: Any {
    fn any_ref(&self) -> &dyn Any;
    fn any_mut(&mut self) -> &mut dyn Any;
    fn any(self: Box<Self>) -> Box<dyn Any>;
    fn borrow(&self) -> &dyn Value;
    fn borrow_mut(&mut self) -> &mut dyn Value;
    fn boxed(self) -> Box<dyn Value>;
    fn typ(&self) -> Type;
    fn field(&self, name: &str) -> Option<&dyn Value>;
    fn field_mut(&mut self, name: &str) -> Option<&mut dyn Value>;
    fn into_field(self, name: &str) -> Option<Box<dyn Value>>;
    fn method(&self, name: &str) -> Option<Func>;
}

impl<T: Any> Value for T {
    fn any_ref(&self) -> &dyn Any {
        self
    }
    fn any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
    fn borrow(&self) -> &dyn Value {
        self
    }
    fn borrow_mut(&mut self) -> &mut dyn Value {
        self
    }
    fn boxed(self) -> Box<dyn Value> {
        Box::new(self)
    }

    default fn typ(&self) -> Type {
        Type::new::<Self>(TypeKind::default())
    }

    default fn field(&self, _: &str) -> Option<&dyn Value> {
        None
    }

    default fn field_mut(&mut self, _: &str) -> Option<&mut dyn Value> {
        None
    }

    default fn into_field(self, _: &str) -> Option<Box<dyn Value>> {
        None
    }

    default fn method(&self, _: &str) -> Option<Func> {
        None
    }
}

impl<K> Index<K> for dyn Value
where
    K: AsRef<str>,
{
    type Output = dyn Value;

    fn index(&self, index: K) -> &Self::Output {
        self.field(index.as_ref())
            .unwrap_or_else(|| panic!("field {} not found", index.as_ref()))
    }
}

impl<K> IndexMut<K> for dyn Value
where
    K: AsRef<str>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.field_mut(index.as_ref())
            .unwrap_or_else(|| panic!("field {} not found", index.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    struct A {
        b: B,
    }

    struct B {
        data: String,
    }

    impl Value for A {
        fn field(&self, name: &str) -> Option<&dyn Value> {
            match name {
                "b" => Some(&self.b),
                _ => None,
            }
        }

        fn field_mut(&mut self, name: &str) -> Option<&mut dyn Value> {
            match name {
                "b" => Some(&mut self.b),
                _ => None,
            }
        }

        fn into_field(self, name: &str) -> Option<Box<dyn Value>> {
            match name {
                "b" => Some(Box::new(self.b)),
                _ => None,
            }
        }
    }

    impl Value for B {
        fn field(&self, name: &str) -> Option<&dyn Value> {
            match name {
                "data" => Some(&self.data),
                _ => None,
            }
        }

        fn field_mut(&mut self, name: &str) -> Option<&mut dyn Value> {
            match name {
                "data" => Some(&mut self.data),
                _ => None,
            }
        }

        fn into_field(self, name: &str) -> Option<Box<dyn Value>> {
            match name {
                "data" => Some(Box::new(self.data)),
                _ => None,
            }
        }
    }

    #[test]
    fn index() {
        let data = "Hello, World";
        let a = A {
            b: B {
                data: data.to_string(),
            },
        }
        .boxed();
        assert_eq!(
            data,
            a["b"]["data"].any_ref().downcast_ref::<String>().unwrap()
        );
    }

    #[test]
    fn index_mut() {
        let data = "Hello, World";
        let mut a = A {
            b: B {
                data: data.to_string(),
            },
        }
        .boxed();
        assert_eq!(
            data,
            a["b"]["data"].any_ref().downcast_ref::<String>().unwrap()
        );
        *a["b"]["data"].any_mut().downcast_mut::<String>().unwrap() = "Hello, Foo".to_string();
        assert_eq!(
            "Hello, Foo",
            a["b"]["data"].any_ref().downcast_ref::<String>().unwrap()
        );
    }
}

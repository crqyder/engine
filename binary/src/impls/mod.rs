pub mod numeric;
pub use numeric::*;

pub mod other;
pub use other::*;

pub mod string;
pub use string::*;

pub mod sequence;
pub use sequence::*;

///
/// This macro is used to generate the implementations for wrapping generic rust types to
/// abstract away the serialization and deserialization over the wire.
///

#[macro_export]
macro_rules! generate {
    ($name:ident, <$($gen:ident: $gen_constraint:ident),*>, $type:ty $(,$lifetime:tt)?) => {
        #[allow(non_snake_case)]
        #[derive(PartialEq)]
        pub struct $name<$($lifetime,)? $($gen: $gen_constraint),*> {
            val: $type,
            $( $gen_constraint: std::marker::PhantomData<$gen>, )*
        }

        impl<$($lifetime,)? $($gen: $gen_constraint),*> $name<$($lifetime,)? $($gen),*> {
            pub fn new(val: $type) -> Self {
                Self {
                    val,
                    $( $gen_constraint: std::marker::PhantomData, )*
                }
            }

            pub fn get(self) -> $type {
                self.val
            }
        }

        impl<$($lifetime,)? $($gen: $gen_constraint),*> AsRef<$type> for $name<$($lifetime,)? $($gen),*> {
            fn as_ref(&self) -> &$type{
                &self.val
            }
        }

        impl<$($lifetime,)? $($gen: $gen_constraint),*> std::fmt::Debug for $name<$($lifetime,)? $($gen),*> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.val)
            }
        }

        impl<$($lifetime,)? $($gen: $gen_constraint),*> core::ops::Deref for $name<$($lifetime,)? $($gen),*> {
            type Target = $type;

            fn deref(&self) -> &Self::Target{
                &self.val
            }
        }

        impl<$($lifetime,)? $($gen: $gen_constraint),*> core::ops::DerefMut for $name<$($lifetime,)? $($gen),*> {
            fn deref_mut(&mut self) -> &mut Self::Target{
                &mut self.val
            }
        }

        impl<$($lifetime,)? $($gen: $gen_constraint),*> From<$type> for $name<$($lifetime,)? $($gen),*> {
            fn from(value: $type) -> $name<$($lifetime,)? $($gen),*> {
                Self::new(value)
            }
        }

        impl<$($lifetime,)? $($gen: $gen_constraint),*> From<$name<$($lifetime,)? $($gen),*>> for $type {
            fn from(value: $name<$($lifetime,)? $($gen),*>) -> $type {
                value.get()
            }
        }
    };
}

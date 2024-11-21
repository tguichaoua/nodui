//! Socket's field.

/// Create the type `SocketField`.
macro_rules! socket_field {
    (
        $( $name:ident($ty:ty) )*
    ) => {
        /// A mutably borrowed socket field value.
        pub enum SocketField<'a> {
            $(
                #[allow(missing_docs)]
                $name(&'a mut $ty),
            )*
        }

        $(
            impl<'a> From<&'a mut $ty> for SocketField<'a> {
                #[inline]
                fn from(value: &'a mut $ty) -> Self {
                    Self::$name(value)
                }
            }
        )*
    };
}

socket_field! {
    Bool(bool)

    F32(f32)
    F64(f64)

    I8(i8)
    I16(i16)
    I32(i32)
    I64(i64)

    U8(u8)
    U16(u16)
    U32(u32)
    U64(u64)
}

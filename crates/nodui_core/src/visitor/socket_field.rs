macro_rules! socket_field {
    (
        $( $name:ident($ty:ty) )*
    ) => {
        pub enum SocketField<'a> {
            $( $name(&'a mut $ty), )*
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

    I32(i32)

    // TODO: add all integer types
}

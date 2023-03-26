macro_rules! new_struct {
    ($name:ident { $($field_name:ident : $field_type:ty),* }) => {
        new_struct!($name { $($field_name: $field_type),* } derive());
    };

    ($name:ident { $($field_name:ident : $field_type:ty),* } derive($($derive_name:ident),*)) => {
        #[derive($($derive_name,)*)]
        pub struct $name {
            $(pub $field_name: $field_type,)*
        }

        impl $name {
            pub fn new($($field_name: $field_type,)*) -> $name{
                $name { $($field_name,)* }
            }
        }
    }
}

pub(crate) use new_struct;

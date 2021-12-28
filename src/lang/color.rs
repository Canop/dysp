use {
    super::*,
    std::str::FromStr,
};

macro_rules! colors {
    ($($name: ident,)*) => {
        #[derive(Debug, Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub enum Color {
            $($name,)*
        }
        impl Color {
            pub fn name(self) -> &'static str {
                match self {
                    $(Self::$name => stringify!($name),)*
                }
            }
        }
        impl FromStr for Color {
            type Err = SyntaxError;
            fn from_str(s: &str) -> Result<Self, SyntaxError> {
                match s {
                    $(stringify!($name) => Ok(Self::$name),)*
                    _ => Err(SyntaxError::UnknownColor(s.to_string())),
                }
            }
        }
    }
}

colors! {
    black,
    blue,
    green,
    red,
    yellow,
}

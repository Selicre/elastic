
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Buttons {
    pub current: u32,
}

macro_rules! buttons {
    ($($val:expr, $const:ident, $func:ident;)*) => {
        #[allow(dead_code)]
        impl Buttons {
            pub fn has(self, pressed: u32) -> bool {
                self.current & pressed != 0
            }
            $(
                pub const $const: u32 = 1<<$val;
                pub fn $func(self) -> bool { self.has(Self::$const) }
            )*
        }
    }
}

buttons! {
    0, LEFT, left;
    1, RIGHT, right;
    2, UP, up;
    3, DOWN, down;
    4, START, start;
    5, A, a;
    6, B, b;
    7, C, c;
}

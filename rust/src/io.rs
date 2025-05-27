use crate::display::Display;
use crate::numbers::{Byte, Num};
use std::fmt::Write;

pub struct End;
pub struct Item<T, Tail>(T, Tail);

pub trait List: Display {
    type Head: Num;
    type Tail: List;
}

impl List for End {
    type Head = Byte<0>;
    type Tail = End;
}
impl Display for End {
    fn fmt(_f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<T, Tail> List for Item<T, Tail>
where
    T: Num,
    Tail: List,
{
    type Head = T;
    type Tail = Tail;
}

impl<T, Tail> Display for Item<T, Tail>
where
    T: Display,
    Tail: List,
{
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Tail as Display>::fmt(f)?;
        f.write_char(' ')?;
        <T as Display>::fmt(f)
    }
}

pub trait InputOutput {
    type Read: Num;
    type Pop: InputOutput;

    type Write<N>: InputOutput
    where
        N: Num;

    type Output: List;
}

impl<I, O> InputOutput for (I, O)
where
    I: List,
    O: List,
{
    type Read = I::Head;
    type Pop = (I::Tail, O);
    type Write<N>
        = (I, Item<N, O>)
    where
        N: Num;

    type Output = O;
}

pub type Init<Input> = (Input, End);

#[macro_export]
macro_rules! input {
    (
        $input:literal $($then:tt)*
    ) => {
        $crate::io::Item<
            $crate::numbers::Byte<$input>,
            $crate::input!($($then)*)
        >
    };
    () => {
        $crate::io::End
    };
}

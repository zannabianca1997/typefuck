use std::fmt::Write;

use crate::display::Display;

use crate::numbers::{Byte, Num};

pub struct End;
pub struct Cell<T, Tail>(T, Tail);

pub trait List {
    type Head: Num;
    type Tail: List;

    fn fmt_left(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::Tail::fmt_left(f)?;
        Self::Head::fmt(f)?;
        f.write_char(' ')
    }

    fn fmt_right(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(' ')?;
        Self::Head::fmt(f)?;
        Self::Tail::fmt_right(f)
    }
}

impl List for End {
    type Head = Byte<0>;
    type Tail = End;

    fn fmt_left(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("... ")
    }

    fn fmt_right(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(" ...")
    }
}

impl<T, Tail> List for Cell<T, Tail>
where
    T: Num,
    Tail: List,
{
    type Head = T;
    type Tail = Tail;
}

pub trait Memory {
    type Get: Num;

    type Set<N>: Memory
    where
        N: Num;

    type Left: Memory;
    type Right: Memory;

    type Add: Memory;
    type Sub: Memory;
}

impl<Before, Here, After> Memory for (Before, Here, After)
where
    Before: List,
    Here: Num,
    After: List,
{
    type Get = Here;

    type Set<N>
        = (Before, N, After)
    where
        N: Num;

    type Left = (Before::Tail, Before::Head, Cell<Here, After>);
    type Right = (Cell<Here, Before>, After::Head, After::Tail);

    type Add = (Before, Here::Next, After);
    type Sub = (Before, Here::Prev, After);
}

impl<Before, Here, After> Display for (Before, Here, After)
where
    Before: List,
    Here: Num,
    After: List,
{
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Before::fmt_left(f)?;
        Here::fmt(f)?;
        After::fmt_right(f)
    }
}

pub type Empty = (End, Byte<0>, End);

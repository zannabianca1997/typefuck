use crate::{
    io::{self, InputOutput},
    memory::{self, Memory},
    numbers::Num,
};

pub trait State {
    type Here: Num;

    type Left: State;
    type Right: State;

    type Add: State;
    type Sub: State;

    type Input: State;
    type Output: State;

    type Written: io::List;
}

pub struct StateHolder<M, IO>(M, IO);

impl<M, IO> State for StateHolder<M, IO>
where
    M: Memory,
    IO: InputOutput,
{
    type Here = M::Get;

    type Left = StateHolder<M::Left, IO>;

    type Right = StateHolder<M::Right, IO>;

    type Add = StateHolder<M::Add, IO>;

    type Sub = StateHolder<M::Sub, IO>;

    type Input = StateHolder<M::Set<IO::Read>, IO::Pop>;
    type Output = StateHolder<M, IO::Write<M::Get>>;

    type Written = IO::Output;
}

pub type Initial<Input> = StateHolder<memory::Empty, io::Init<Input>>;

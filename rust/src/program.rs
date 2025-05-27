use std::marker::PhantomData;

use crate::{numbers::Num, state::State};

pub trait Instruction {
    type AndThen<Then>: Instruction + ?Sized
    where
        Then: Instruction + ?Sized;

    type Apply<S>: State + ?Sized
    where
        S: State + ?Sized;
}

pub struct End;
impl Instruction for End {
    type AndThen<Then>
        = Then
    where
        Then: Instruction + ?Sized;

    type Apply<S>
        = S
    where
        S: State + ?Sized;
}

pub struct Left<Then: ?Sized>(Then);

impl<Then> Instruction for Left<Then>
where
    Then: Instruction + ?Sized,
{
    type AndThen<T>
        = Left<Then::AndThen<T>>
    where
        T: Instruction + ?Sized;

    type Apply<S>
        = Then::Apply<S::Left>
    where
        S: State + ?Sized;
}

pub struct Right<Then: ?Sized>(Then);

impl<Then> Instruction for Right<Then>
where
    Then: Instruction + ?Sized,
{
    type AndThen<T>
        = Right<Then::AndThen<T>>
    where
        T: Instruction + ?Sized;

    type Apply<S>
        = Then::Apply<S::Right>
    where
        S: State + ?Sized;
}

pub struct Add<Then: ?Sized>(Then);

impl<Then> Instruction for Add<Then>
where
    Then: Instruction + ?Sized,
{
    type AndThen<T>
        = Add<Then::AndThen<T>>
    where
        T: Instruction + ?Sized;

    type Apply<S>
        = Then::Apply<S::Add>
    where
        S: State + ?Sized;
}

pub struct Sub<Then: ?Sized>(Then);

impl<Then> Instruction for Sub<Then>
where
    Then: Instruction + ?Sized,
{
    type AndThen<T>
        = Sub<Then::AndThen<T>>
    where
        T: Instruction + ?Sized;

    type Apply<S>
        = Then::Apply<S::Sub>
    where
        S: State + ?Sized;
}

pub struct Input<Then: ?Sized>(Then);

impl<Then> Instruction for Input<Then>
where
    Then: Instruction + ?Sized,
{
    type AndThen<T>
        = Input<Then::AndThen<T>>
    where
        T: Instruction + ?Sized;

    type Apply<S>
        = Then::Apply<S::Input>
    where
        S: State + ?Sized;
}

pub struct Output<Then: ?Sized>(Then);

impl<Then> Instruction for Output<Then>
where
    Then: Instruction + ?Sized,
{
    type AndThen<T>
        = Output<Then::AndThen<T>>
    where
        T: Instruction + ?Sized;

    type Apply<S>
        = Then::Apply<S::Output>
    where
        S: State + ?Sized;
}

pub struct Loop<Body: ?Sized, Then: ?Sized>(PhantomData<Body>, Then);

impl<Body, Then> Instruction for Loop<Body, Then>
where
    Body: Instruction + ?Sized,
    Then: Instruction + ?Sized,
{
    type AndThen<T>
        = Loop<Body, Then::AndThen<T>>
    where
        T: Instruction + ?Sized;

    type Apply<S>
        = <<S::Here as Num>::If<Then, Body::AndThen<Self>> as Instruction>::Apply<S>
    where
        S: State + ?Sized;
}

#[macro_export]
macro_rules! program {
    () => {
        $crate::program::End
    };
    ( > $( $then:tt )*) => {
        $crate::program::Right<$crate::program!($($then)*)>
    };
    ( < $( $then:tt )*) => {
        $crate::program::Left<$crate::program!($($then)*)>
    };
    ( + $( $then:tt )*) => {
        $crate::program::Add<$crate::program!($($then)*)>
    };
    ( - $( $then:tt )*) => {
        $crate::program::Sub<$crate::program!($($then)*)>
    };
    ( . $( $then:tt )*) => {
        $crate::program::Output<$crate::program!($($then)*)>
    };
    ( , $( $then:tt )*) => {
        $crate::program::Input<$crate::program!($($then)*)>
    };
    ( [ $( $body:tt )* ] $( $then:tt )*) => {
        $crate::program::Loop<$crate::program!($($body)*), $crate::program!($($then)*)>
    };
}

pub type Run<Program, Input> =
    <<Program as Instruction>::Apply<crate::state::Initial<Input>> as State>::Written;

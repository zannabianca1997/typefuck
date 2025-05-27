use std::marker::PhantomData;

pub trait Display {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn display() -> impl std::fmt::Display {
        struct DisplayWrapper<T: ?Sized>(PhantomData<T>);

        impl<T> std::fmt::Display for DisplayWrapper<T>
        where
            T: Display + ?Sized,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                T::fmt(f)
            }
        }

        DisplayWrapper::<Self>(PhantomData)
    }
}

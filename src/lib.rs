#![allow(unused)]
use core::f64;
use std::u16;

use rand::{rng, Rng};

fn monad() {
    let mut rng = rng();
    // Monadic Computational Effects
    let vec: Vec<_> = (0..=9).map(|_| rng.random_range(u16::MIN..=u16::MAX)).collect();
    // Overflow depends on the random state - wrap this in an optional monad to contain the
    // overflow side effect of unsigned addition
    let val = vec.get(4).and_then(|val| (*val).checked_add(586));
    println!("{val:?}");
}

/// Static mut representing our envirnoment that may affect computation
/// Rust makes me use unsafe with it, because comonads regulate envirnomental effects, and
/// envirnomental effects (when being done as mutable global statics) can cause memory unsafety
static mut HEIGHT: f64 = 15.;

/// Our comonad struct
#[derive(Debug, Clone)]
struct CoMonad<T: Clone, V: Clone> {
    value: T,
    env: V,
}

/// Function which adds one to an f64
fn add_one(x: f64) -> f64 {
    x + 1.
}

// extract : C(x) -> x
fn extract<T: Clone, V: Clone>(input: &CoMonad<T, V>) -> T {
    input.value.clone()
}

/// fmap : (Funcs(x) -> (y)) -> (Funcs(C(x)) -> (C(y)))
/// fmap as a pure function not associated with the [`CoMonad`] struct
fn fmap<O: Clone, T: Clone, V: Clone>(func: impl Fn(T) -> O) -> impl Fn(CoMonad<T, V>) -> CoMonad<O, V> {
    move |input: CoMonad<T, V>| {
        CoMonad {
            value: func(extract(&input)),
            env: input.env
        }
    }
}

/// duplicate as a pure function not associated with the [`CoMonad`] struct
fn duplicate<T: Clone, V: Clone>(input: CoMonad<T, V>) -> CoMonad<CoMonad<T, V>, V> {
    CoMonad {
        value: input.clone(),
        env: input.env.clone()
    }
}

/// extend as a pure function not associated with the [`CoMonad`] struct
fn extend<T: Clone, V: Clone, O: Clone>(func: &impl Fn(CoMonad<T, V>) -> O) -> impl Fn(CoMonad<T, V>) -> CoMonad<O, V> {
    move |input: CoMonad<T, V>| {
        fmap(func)(duplicate(input))
    }
}

/// Any function which takes in a CoMonad as an input from the previous functions automatically passes self in here.
impl<T: Clone, V: Clone> CoMonad<T, V> {
    // extract : C(x) -> x
    pub(self) fn extract(&self) -> T {
        self.value.clone()
    }

    /// fmap(self) : (Funcs(x) -> (y)) -> C(x) -> C(y)
    fn fmap<O: Clone>(&self, func: impl Fn(T) -> O) -> CoMonad<O, V> {
        CoMonad {
            value: func(self.extract()),
            env: self.env.clone()
        }
    }

    /// duplicate : C(x) -> C(C(x))
    fn duplicate(&self) -> CoMonad<CoMonad<T, V>, V> {
        CoMonad {
            value: self.clone(),
            env: self.env.clone()
        }
    }

    /// Essentially an [`fmap`] which preserves the envirnoment during computation, should it be
    /// useful for another computation later
    /// extend : (C(x) -> y) -> C(x) -> C(y)
    fn extend<O: Clone>(&self, func: impl Fn(CoMonad<T, V>) -> O) -> CoMonad<O, V> {
        self.duplicate().fmap(func)
    }
}

fn comonad() {
    // Consider the impure function x
    let vol = |radius: f64| {
        unsafe { f64::consts::PI * radius * HEIGHT }
    };

    // We can replace this with a pure function that captures the environment
    let vol = |radius: CoMonad<f64, f64>| {
        f64::consts::PI * radius.value * radius.env
    };
    // Comonad modifies input - Monad modifies the output
    // What we want to do now, is define a function that can take in a pure function that applies
    // to the pure input of our comonad, and performs our computation on that new input as the new
    // pure input to our function
    let comonad = CoMonad {
        value: 14.,
        env: unsafe { HEIGHT }
    };

    let added_one = comonad.fmap(add_one);
    let add_one_pure = |inp: CoMonad<f64, f64>| {
        inp.value + 1.
    };

    let res_0 = add_one_pure(comonad.extend(add_one_pure));

    let res_1 = add_one_pure(extend(&add_one_pure)(comonad));
    println!("{res_0} == {res_1}");
}

#[cfg(test)]
pub mod test {
    use crate::{comonad, monad};

    #[test]
    fn test_monad() {
        monad();
    }

    #[test]
    fn test_comonad() {
        comonad();
    }
}

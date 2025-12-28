// src/linear_algebra/traits.rs
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub, Mul, Div, Neg};

// Definimos nuestros propios traits de identidad para no depender de crates externos obligatoriamente
pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
}

// El Trait Maestro: Agrupa todo lo que una matriz necesita de sus elementos
pub trait Scalar: 
    Clone +          // Debe poder clonarse (pero no exigimos Copy)
    Debug + Display + 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> + 
    Neg<Output = Self> + 
    Zero + One 
{}

// Implementación automática: Si cumple los requisitos, es un Scalar.
impl<T> Scalar for T 
where 
    T: Clone + Debug + Display + 
       Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T> + 
       Zero + One 
{}
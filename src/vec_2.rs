#![allow(dead_code)]
use core::ops::{
    Index, IndexMut,
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
    BitOr, BitOrAssign, BitAnd, BitAndAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign,
    Neg,
};
use self::Axis::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}
impl Axis {
    pub fn swap(self) -> Self {
        match self {
            X => Y,
            Y => X,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2<T>(
    pub [T; 2]
);

impl<T> Index<Axis> for Vec2<T> {
    type Output = T;
    fn index(&self, coordinate: Axis) -> &Self::Output {
        &self.0[coordinate as usize]
    }
}
impl<T> IndexMut<Axis> for Vec2<T> {
    fn index_mut(&mut self, axis: Axis) -> &mut Self::Output {
        &mut self.0[axis as usize]
    }
}

macro_rules! impl_op_vec2 {
    ($type:ident, $($trait:ident::$method:ident) +) => {$(
        impl $trait<Vec2<$type>> for Vec2<$type> {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self::Output {
                Vec2([$trait::$method(self[X], rhs[X]), $trait::$method(self[Y], rhs[Y])])
            }
        }
        impl $trait<$type> for Vec2<$type> {
            type Output = Self;
            fn $method(self, rhs: $type) -> Self::Output {
                Vec2([$trait::$method(self[X], rhs), $trait::$method(self[Y], rhs)])
            }
        }
    )+};
}
macro_rules! impl_assign_vec2 {
    ($type:ident, $($trait:ident::$method:ident) +) => {$(
        impl $trait<Vec2<$type>> for Vec2<$type> {
            fn $method(self: &mut Vec2<$type>, rhs: Self) {
                $trait::$method(&mut self[X], rhs[X]);
                $trait::$method(&mut self[Y], rhs[Y]);
            }
        }
        impl $trait<$type> for Vec2<$type> {
            fn $method(self: &mut Vec2<$type>, rhs: $type) {
                $trait::$method(&mut self[X], rhs);
                $trait::$method(&mut self[Y], rhs);
            }
        }
    )+};
}

macro_rules! impl_vec2_aritmetic {
    ($($type:ident) +) => {$(
        impl_op_vec2!($type, Add::add Sub::sub Mul::mul Div::div Rem::rem);
        impl_assign_vec2!($type, AddAssign::add_assign SubAssign::sub_assign MulAssign::mul_assign DivAssign::div_assign RemAssign::rem_assign);
    )+};
}
impl_vec2_aritmetic!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64);

macro_rules! impl_vec2_bitwise {
    ($($type:ident) +) => {$(
        impl_op_vec2!($type, BitOr::bitor BitAnd::bitand BitXor::bitxor Shl::shl Shr::shr);
        impl_assign_vec2!($type, BitOrAssign::bitor_assign BitAndAssign::bitand_assign BitXorAssign::bitxor_assign ShlAssign::shl_assign ShrAssign::shr_assign);
    )+};
}
impl_vec2_bitwise!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);

macro_rules! impl_vec2_neg {
    ($($type:ident) +) => {$(
        impl Neg for Vec2<$type> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Vec2([- self[X], - self[Y]])
            }
        }
    )+};
}
impl_vec2_neg!(isize i8 i16 i32 i64 i128 f32 f64);

macro_rules! fn_cast_vec2 {
    ($($type:ident) +) => {$(
        pub fn $type(self) -> Vec2<$type> {
            Vec2([self[X] as $type, self[Y] as $type])
        }
    )+};
}

macro_rules! impl_cast_vec2 {
    ($($type:ident) +) => {$(
        impl Vec2<$type> {
            fn_cast_vec2!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64);
        }
    )+};
}
impl_cast_vec2!(usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64);

macro_rules! fn_float_vec2 {
    ($($method:ident) +) => {$(
        pub fn $method(self) -> Self {
            Vec2([self[X].$method(), self[Y].$method()])
        }
    )+};
}

macro_rules! impl_float_vec2 {
    ($($type:ident) +) => {$(
        impl Vec2<$type> {
            fn_float_vec2!(floor ceil round trunc);
        }
    )+};
}
impl_float_vec2!(f32 f64);

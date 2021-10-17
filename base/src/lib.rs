#![feature(const_fn_trait_bound)]

use num::traits::Signed;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct HexCoord<T> {
    q: T,
    r: T,
    s: T,
}

impl<T> HexCoord<T> {
    pub const fn from_qrs(q: T, r: T, s: T) -> Self {
        Self { q, r, s }
    }

    #[inline]
    pub const fn q(&self) -> T
    where
        T: Copy,
    {
        self.q
    }

    #[inline]
    pub const fn r(&self) -> T
    where
        T: Copy,
    {
        self.r
    }

    #[inline]
    pub const fn s(&self) -> T
    where
        T: Copy,
    {
        self.s
    }

    pub fn length(&self) -> T
    where
        T: Signed + Div<usize, Output = T>,
    {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }

    pub fn distance(self, other: Self) -> T
    where
        T: Signed + Div<usize, Output = T> + Sub<Output = T>,
    {
        (self - other).length()
    }
}

impl HexCoord<isize> {
    pub fn from_qr(q: isize, r: isize) -> Self {
        Self { q, r, s: -q - r }
    }

    pub fn neighbor(self, direction: isize) -> Self {
        self + hex_direction(direction)
    }
}

impl HexCoord<f32> {
    pub fn from_qr(q: f32, r: f32) -> Self {
        Self { q, r, s: -q - r }
    }
}

impl<T> Add for HexCoord<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

impl<T> Sub for HexCoord<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            q: self.q - other.q,
            r: self.r - other.r,
            s: self.s - other.s,
        }
    }
}

impl<T, S> Mul<S> for HexCoord<T>
where
    S: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: S) -> Self {
        Self {
            q: other * self.q,
            r: other * self.r,
            s: other * self.s,
        }
    }
}

const HEX_DIRECTIONS: [HexCoord<isize>; 6] = [
    HexCoord::from_qrs(1, 0, -1),
    HexCoord::from_qrs(1, -1, 0),
    HexCoord::from_qrs(0, -1, 1),
    HexCoord::from_qrs(-1, 0, 1),
    HexCoord::from_qrs(-1, 1, 0),
    HexCoord::from_qrs(0, 1, -1),
];

pub fn hex_direction(direction: isize) -> HexCoord<isize> {
    HEX_DIRECTIONS[direction.rem_euclid(6) as usize]
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_cube_directions() {
//         use super::{CubeCoord, cube_direction};
//
//         assert_eq!(cube_direction(0), CubeCoord::new(1, -1, 0));
//         assert_eq!(cube_direction(-1), CubeCoord::new(0, -1, 1));
//         assert_eq!(cube_direction(1), CubeCoord::new(1, 0, -1));
//     }
// }

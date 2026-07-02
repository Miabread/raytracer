use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point::new(x, y, z)
}

pub fn arrow(x: f64, y: f64, z: f64) -> Arrow {
    Arrow::new(x, y, z)
}

pub fn color(x: f64, y: f64, z: f64) -> Color {
    Color::new(x, y, z)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec3<T = ()>(f64, f64, f64, PhantomData<T>);

impl<T: Copy> Vec3<T> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, PhantomData)
    }

    pub fn of(a: f64) -> Vec3 {
        vec3(a, a, a)
    }

    pub fn map(self, mut func: impl FnMut(f64) -> f64) -> Self {
        Self::new(func(self.0), func(self.1), func(self.2))
    }
    pub fn zip<U: Copy>(self, rhs: Vec3<U>, mut func: impl FnMut(f64, f64) -> f64) -> Self {
        Self::new(
            func(self.0, rhs.0),
            func(self.1, rhs.1),
            func(self.2, rhs.2),
        )
    }

    pub fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot<U>(self, rhs: Vec3<U>) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    pub fn cross<U>(self, rhs: Vec3<U>) -> Self {
        Self::new(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("invalid vec3 index"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("invalid vec3 index"),
        }
    }
}

impl<T: Copy, U: Copy> Add<Vec3<U>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<U>) -> Self::Output {
        self.zip(rhs, |a, b| a + b)
    }
}

impl<T: Copy> Add<f64> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: f64) -> Self::Output {
        self.map(|a| a + rhs)
    }
}

impl<T: Copy> Add<Vec3<T>> for f64 {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        rhs.map(|a| a + self)
    }
}

impl<T: Copy, U: Copy> Sub<Vec3<U>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<U>) -> Self::Output {
        self.zip(rhs, |a, b| a - b)
    }
}

impl<T: Copy> Sub<f64> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: f64) -> Self::Output {
        self.map(|a| a - rhs)
    }
}

impl<T: Copy> Sub<Vec3<T>> for f64 {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        rhs.map(|a| self - a)
    }
}

impl<T: Copy, U: Copy> Mul<Vec3<U>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<U>) -> Self::Output {
        self.zip(rhs, |a, b| a * b)
    }
}

impl<T: Copy> Mul<f64> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

impl<T: Copy> Mul<Vec3<T>> for f64 {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        rhs.map(|a| a * self)
    }
}

impl<T: Copy> Div<f64> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: f64) -> Self::Output {
        self.map(|a| a / rhs)
    }
}

impl<T: Copy> Div<Vec3<T>> for f64 {
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        rhs.map(|a| self / a)
    }
}

impl Display for Vec3<()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec3({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointMarker;
pub type Point = Vec3<PointMarker>;
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "point({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ArrowMarker;
pub type Arrow = Vec3<ArrowMarker>;
impl Display for Arrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dir({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorMarker;
pub type Color = Vec3<ColorMarker>;
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.0, self.1, self.2)
    }
}

impl<T> Vec3<T> {
    pub fn vec3(self) -> Vec3 {
        Vec3::new(self.0, self.1, self.2)
    }

    pub fn point(self) -> Point {
        Point::new(self.0, self.1, self.2)
    }

    pub fn arrow(self) -> Arrow {
        Arrow::new(self.0, self.1, self.2)
    }

    pub fn color(self) -> Color {
        Color::new(self.0, self.1, self.2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn r(&self) -> f64 {
        self.0
    }
    pub fn g(&self) -> f64 {
        self.1
    }
    pub fn b(&self) -> f64 {
        self.2
    }
    pub fn u(&self) -> f64 {
        self.0
    }
    pub fn v(&self) -> f64 {
        self.1
    }
    pub fn w(&self) -> f64 {
        self.2
    }
}

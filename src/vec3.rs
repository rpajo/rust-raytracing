use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::{impl_binary_operations, impl_op_assign, impl_unary_operations};

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn from_float(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    // Returns the component of this vector along the specified
    // axis. For example, `some_vec.component(Axis::X)` returns
    // `some_vec.x`.
    pub fn component(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    // Sets the component of this vector along the specified
    // axis. For example, `some_vec.set_component(Axis::X, 1.0)`
    // sets `some_vec.x` to 1.0`.
    pub fn set_component(&mut self, axis: Axis, value: f32) {
        match axis {
            Axis::X => {
                self.x = value;
            }
            Axis::Y => {
                self.y = value;
            }
            Axis::Z => {
                self.z = value;
            }
        }
    }

    pub fn normalize(&self) -> Vec3 {
        self / self.length()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn min(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::ZERO
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl_binary_operations!(Vec3 Add add +);
impl_op_assign!(Vec3 AddAssign add_assign +);

impl_binary_operations!(Vec3 Sub sub -);
impl_op_assign!(Vec3 SubAssign sub_assign -);
impl_unary_operations!(Vec3 Neg neg -);

impl_binary_operations!(Vec3 Mul mul *);
impl_op_assign!(Vec3 MulAssign mul_assign *);

impl_binary_operations!(Vec3 Div div /);
impl_op_assign!(Vec3 DivAssign div_assign /);

// These are just aliases for clarity.
pub type Color3 = Vec3;
pub type Pos3 = Vec3;

impl Color3 {
    pub const RED: Color3 = Color3 {
        x: 255.0,
        y: 0.0,
        z: 0.0,
    };
    pub const GREEN: Color3 = Color3 {
        x: 0.0,
        y: 255.0,
        z: 0.0,
    };
    pub const BLUE: Color3 = Color3 {
        x: 0.0,
        y: 0.0,
        z: 255.0,
    };

    pub const WHITE: Color3 = Color3 {
        x: 255.0,
        y: 255.0,
        z: 255.0,
    };

    pub const BLACK: Color3 = Color3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn red(&self) -> f32 {
        self.x
    }
    pub fn green(&self) -> f32 {
        self.y
    }
    pub fn blue(&self) -> f32 {
        self.z
    }

    pub fn from_f32(r: f32, g: f32, b: f32) -> Color3 {
        Color3 {
            x: 255.0 * r,
            y: 255.0 * g,
            z: 255.0 * b,
        }
    }
}
#[allow(clippy::op_ref)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(&a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&a + b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + b, Vec3::new(3.0, 5.0, 7.0));

        // Test for RHS value type
        {
            let mut c = Vec3::ONE;
            c += a;
            assert_eq!(c, Vec3::new(1.0, 2.0, 3.0));
        }

        // Test for RHS borrowed reference
        {
            let mut c = Vec3::ONE;
            c += &a;
            assert_eq!(c, Vec3::new(1.0, 2.0, 3.0));
        }
    }

    #[test]
    fn subtract() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(&a - &b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(a - &b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(&a - b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(a - b, Vec3::new(-3.0, -3.0, -3.0));

        // Test for RHS value type
        {
            let mut c = Vec3::ONE;
            c -= a;
            assert_eq!(c, Vec3::new(1.0, 0.0, -1.0));
        }

        // Test for RHS borrowed reference
        {
            let mut c = Vec3::ONE;
            c -= &a;
            assert_eq!(c, Vec3::new(1.0, 0.0, -1.0));
        }
    }

    #[test]
    fn multiply() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(&a * &b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(a * &b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(&a * b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(a * b, Vec3::new(0.0, 4.0, 10.0));

        // Test for RHS value type
        {
            let mut c = Vec3::from_float(2.0);
            c *= a;
            assert_eq!(c, 2.0 * a);
        }

        // Test for RHS borrowed reference
        {
            let mut c = Vec3::from_float(2.0);
            c *= &a;
            assert_eq!(c, 2.0 * a);
        }
    }

    #[test]
    fn divide() {
        let a = Vec3::new(1.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(&a / &b, Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0));
        assert_eq!(a / &b, Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0));
        assert_eq!(&a / b, Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0));
        assert_eq!(a / b, Vec3::new(1.0 / 3.0, 1.0 / 4.0, 2.0 / 5.0));

        // Test for RHS value type
        {
            let mut c = Vec3::ONE;
            c /= a;
            assert_eq!(c, Vec3::new(1.0, 1.0, 0.5));
        }

        // Test for RHS borrowed reference
        {
            let mut c = Vec3::ONE;
            c /= &a;
            assert_eq!(c, Vec3::new(1.0, 1.0, 0.5));
        }
    }

    #[test]
    fn dot() {
        let a = Vec3::new(2.0, 3.0, 5.0);
        let b = Vec3::new(7.0, 11.0, 13.0);
        assert_eq!(Vec3::dot(&a, &b), 2.0 * 7.0 + 3.0 * 11.0 + 5.0 * 13.0);
    }

    #[test]
    fn cross() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(Vec3::cross(&a, &b), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn length() {
        let a = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(
            a.length(),
            ((3.0 * 3.0 + 2.0 * 2.0 + 1.0 * 1.0) as f32).sqrt()
        );

        let b = Vec3::from_float(0.0);
        assert_eq!(b.length(), 0.0);
    }

    #[test]
    fn normalize() {
        let a = Vec3::new(3.0, 2.0, 1.0);
        let len = a.length();
        assert!((a.normalize().length() - 1.0).abs() < 0.01);
        assert_eq!(a.normalize(), a / len);
    }

    #[test]
    fn component() {
        let a = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(a.component(Axis::X), a.x);
        assert_eq!(a.component(Axis::Y), a.y);
        assert_eq!(a.component(Axis::Z), a.z);
    }

    #[test]
    fn set_component() {
        let mut a = Vec3::new(3.0, 2.0, 1.0);
        a.set_component(Axis::X, 4.0);
        assert_eq!(a, Vec3::new(4.0, 2.0, 1.0));

        a.set_component(Axis::Y, 5.0);
        assert_eq!(a, Vec3::new(4.0, 5.0, 1.0));

        a.set_component(Axis::Z, 6.0);
        assert_eq!(a, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn min() {
        let tiny_x = Vec3::new(0.00001, 1000.0, 1000.0);
        let tiny_y = Vec3::new(1000.0, 0.00001, 1000.0);
        let tiny_z = Vec3::new(1000.0, 1000.0, 0.00001);
        assert_eq!(tiny_x.min(&tiny_y).min(&tiny_z), Vec3::from_float(0.00001));
    }

    #[test]
    fn max() {
        let big_x = Vec3::new(1000.0, 0.00001, 0.00001);
        let big_y = Vec3::new(0.00001, 1000.0, 0.00001);
        let big_z = Vec3::new(0.00001, 0.00001, 1000.0);
        assert_eq!(big_x.max(&big_y).max(&big_z), Vec3::from_float(1000.0));
    }
}

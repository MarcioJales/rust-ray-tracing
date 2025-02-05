use std::{cell::Ref, ops::{Add, Div, Mul, Sub}};
use crate::random::{random, random_within};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn dot(self, u: Vec3) -> f64 {
        self.0 * u.0 + self.1 * u.1 + self.2 * u.2
    }

    pub fn cross(self, u: Vec3) -> Vec3 {
        Vec3(
            self.1 * u.2 - self.2 * u.1,
            self.2 * u.0 - self.0 * u.2,
            self.0 * u.1 - self.1 * u.0
        )
    }

    fn length(&self) -> f64 {
        let squared = self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
        squared.sqrt()
    }

    pub fn unit(self) -> Vec3 {
        let length = self.length();
        self / length
    }

    fn random() -> Vec3 {
        Vec3(random(), random(), random())
    }

    fn random_within(min: f64, max: f64) -> Vec3 {
        Vec3(random_within(min, max), random_within(min, max), random_within(min, max))
    }

    /* 
    ** This method applies "rejection method":
    ** If the reflected ray lies outside the valid sphere (radius = 1), we discard it and calculate again.
    ** At the end, we normalize it.
    ** On the discussion about why we discard rays outside the sphere:
    ** https://github.com/RayTracing/raytracing.github.io/discussions/1369
     */
    pub fn random_unit() -> Vec3 {
        loop {
            let reflected = Self::random_within(-1.0, 1.0);
            let len = reflected.length();
            /* The first comparison is to handle a small floating-point abstraction leak */
            if 1e-160 < len && len <= 1. {
                return reflected / len;
            }
        }
    }

    pub fn near_zero(self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        return (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s) 
    }

    pub fn reflect(inciding: &Vec3, normal: &Vec3) -> Vec3 {
        return inciding - 2.0 * (inciding.dot(*normal) * normal);
    }

}

impl Add for Vec3 {
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 + rhs.0;
        res.1 = self.1 + rhs.1;
        res.2 = self.2 + rhs.2;

        res
    }

    type Output = Vec3;
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 + rhs.0;
        res.1 = self.1 + rhs.1;
        res.2 = self.2 + rhs.2;

        res
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 + rhs.0;
        res.1 = self.1 + rhs.1;
        res.2 = self.2 + rhs.2;

        res
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 + rhs.0;
        res.1 = self.1 + rhs.1;
        res.2 = self.2 + rhs.2;

        res
    }
}

impl Sub for Vec3 {
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 - rhs.0;
        res.1 = self.1 - rhs.1;
        res.2 = self.2 - rhs.2;

        res
    }

    type Output = Vec3;
}

impl Sub for &Vec3 {
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 - rhs.0;
        res.1 = self.1 - rhs.1;
        res.2 = self.2 - rhs.2;

        res
    }

    type Output = Vec3;
}

impl Sub<Vec3> for &Vec3 {
    fn sub(self, rhs: Vec3) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 - rhs.0;
        res.1 = self.1 - rhs.1;
        res.2 = self.2 - rhs.2;

        res
    }

    type Output = Vec3;
}

impl Sub<&Vec3> for Vec3 {
    fn sub(self, rhs: &Vec3) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 - rhs.0;
        res.1 = self.1 - rhs.1;
        res.2 = self.2 - rhs.2;

        res
    }

    type Output = Vec3;
}

impl Mul for Vec3 {
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 * rhs.0;
        res.1 = self.1 * rhs.1;
        res.2 = self.2 * rhs.2;

        res
    }

    type Output = Vec3;
}

impl Mul for &Vec3 {
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 * rhs.0;
        res.1 = self.1 * rhs.1;
        res.2 = self.2 * rhs.2;

        res
    }

    type Output = Vec3;
}


impl Mul<Vec3> for f64 {
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self * rhs.0;
        res.1 = self * rhs.1;
        res.2 = self * rhs.2;

        res
    }

    type Output = Vec3;
}

impl Mul<&Vec3> for f64 {
    fn mul(self, rhs: &Vec3) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self * rhs.0;
        res.1 = self * rhs.1;
        res.2 = self * rhs.2;

        res
    }

    type Output = Vec3;
}


impl Div for Vec3 {
    fn div(self, rhs: Self) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 / rhs.0;
        res.1 = self.1 / rhs.1;
        res.2 = self.2 / rhs.2;

        res
    }

    type Output = Vec3;
}

impl Div for &Vec3 {
    fn div(self, rhs: Self) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 / rhs.0;
        res.1 = self.1 / rhs.1;
        res.2 = self.2 / rhs.2;

        res
    }

    type Output = Vec3;
}

impl Div<f64> for Vec3 {
    fn div(self, rhs: f64) -> Self::Output {
        let mut res = Vec3(0.0, 0.0, 0.0);

        res.0 = self.0 / rhs;
        res.1 = self.1 / rhs;
        res.2 = self.2 / rhs;

        res
    }

    type Output = Vec3;
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 && self.1 == other.1 && self.2 == other.2 {
            return true;
        }
        else {
            return false;
        };
    }
}

/*******************/
// TEST SECTION
/*******************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let v1 = Vec3(2.0, 6.0, 9.0);

        let v2 = Vec3(1.0, 1.0, 1.0);

        let v3 = v1 + v2;

        assert_eq!(v3, Vec3(3.0, 7.0, 10.0));
        assert_eq!(v3, Vec3(3.0, 7.0, 10.0));
    }


    #[test]
    fn sub() {
        let v1 = Vec3(2.0, 6.0, 9.0);

        let v2 = Vec3(1.0, 1.0, 1.0);

        let v3 = v1 - v2;

        assert_eq!(v3, Vec3(1.0, 5.0, 8.0));
    }


    #[test]
    fn mul() {
        let v1 = Vec3(2.0, 6.0, 9.0);

        let v2 = Vec3(1.0, 1.5, 2.0);

        let v3 = v1 * v2;
        let v4 = 1.5 * v3.clone();

        assert_eq!(v3, Vec3(2.0, 9.0, 18.0));
        assert_eq!(v4, Vec3(3.0, 13.5, 27.0));
    }


    #[test]
    fn div() {
        let v1 = Vec3(2.0, 6.0, 9.0);

        let v2 = Vec3(2.0, 2.0, 2.0);

        let v3 = v1 / v2;
        let v4 = v3.clone() / 2.0;

        assert_eq!(v3, Vec3(1.0, 3.0, 4.5));
        assert_eq!(v4, Vec3(0.5, 1.5, 2.25));
    }

    #[test]
    fn dot() {
        let v1 = Vec3(1.5, 0.6, 1.2);

        let v2 = Vec3(1.0, 2.5, 2.2);

        assert_float_absolute_eq!(v1.dot(v2), 5.64) 
    }


    #[test]
    fn cross() {
        let v1 = Vec3(1.5, 0.6, 1.2);

        let v2 = Vec3(1.0, 2.5, 2.2);

        let v3 = v1.cross(v2);

        assert_float_absolute_eq!(v3.x(), -1.68);
        assert_float_absolute_eq!(v3.y(), -2.1);
        assert_float_absolute_eq!(v3.z(), 3.15);
    }

    #[test]
    fn unit_vector() {
        let v1 = Vec3(1.2, 2.0, 0.5);

        let unit = v1.unit();

        assert_float_absolute_eq!(unit.0, 0.503066);
        assert_float_absolute_eq!(unit.1, 0.838444);
        assert_float_absolute_eq!(unit.2, 0.209611);

    }
}
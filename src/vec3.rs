use std::ops::{Add, Index, IndexMut, Mul, Sub, Div};

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    fn dot(self, u: Vec3) -> f64 {
        self.e[0] + u.e[0] + self.e[1] + u.e[1] + self.e[2] + u.e[2]
    }

    fn cross(self, u: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * u.e[2] - self.e[2] * u.e[1],
                self.e[2] * u.e[0] - self.e[0] * u.e[2],
                self.e[0] * u.e[1] - self.e[1] * u.e[0]
            ]
        }
    }

    fn length(&self) -> f64 {
        let squared = self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
        squared.sqrt()
    }

    fn unit(self) -> Vec3 {
        let length = self.length();
        (1.0/length) * self 
    }
}

impl Add for Vec3 {
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Vec3 {
            e: [0.0, 0.0, 0.0]
        };

        res.e[0] = self.e[0] + rhs.e[0];
        res.e[1] = self.e[1] + rhs.e[1];
        res.e[2] = self.e[2] + rhs.e[2];

        res
    }

    type Output = Vec3;
}

impl Sub for Vec3 {
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Vec3 {
            e: [0.0, 0.0, 0.0]
        };

        res.e[0] = self.e[0] - rhs.e[0];
        res.e[1] = self.e[1] - rhs.e[1];
        res.e[2] = self.e[2] - rhs.e[2];

        res
    }

    type Output = Vec3;
}


impl Mul for Vec3 {
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Vec3 {
            e: [0.0, 0.0, 0.0]
        };

        res.e[0] = self.e[0] * rhs.e[0];
        res.e[1] = self.e[1] * rhs.e[1];
        res.e[2] = self.e[2] * rhs.e[2];

        res
    }

    type Output = Vec3;
}

impl Mul<Vec3> for f64 {
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut res = Vec3 {
            e: [0.0, 0.0, 0.0]
        };

        res.e[0] = self * rhs.e[0];
        res.e[1] = self * rhs.e[1];
        res.e[2] = self * rhs.e[2];

        res
    }

    type Output = Vec3;
}


impl Div for Vec3 {
    fn div(self, rhs: Self) -> Self::Output {
        let mut res = Vec3 {
            e: [0.0, 0.0, 0.0]
        };

        res.e[0] = self.e[0] / rhs.e[0];
        res.e[1] = self.e[1] / rhs.e[1];
        res.e[2] = self.e[2] / rhs.e[2];

        res
    }

    type Output = Vec3;
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        if self.e[0] == other.e[0] && self.e[1] == other.e[1] && self.e[2] == other.e[2] {
            return true;
        }
        else {
            return false;
        };
    }
}

impl Index<usize> for Vec3 {
    fn index(&self, index: usize) -> &Self::Output {
        return &(self.e[index]);
    }

    type Output = f64;
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut (self.e[index]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let v1 = Vec3{
            e: [2.0, 6.0, 9.0],
        };

        let v2 = Vec3{
            e: [1.0, 1.0, 1.0],
        };

        let v3 = v1 + v2;

        assert_eq!(v3.e, [3.0, 7.0, 10.0]);
        assert_eq!(v3.e, [3.0, 7.0, 10.0]);
    }


    #[test]
    fn sub() {
        let v1 = Vec3{
            e: [2.0, 6.0, 9.0],
        };

        let v2 = Vec3{
            e: [1.0, 1.0, 1.0],
        };

        let v3 = v1 - v2;

        assert_eq!(v3.e, [1.0, 5.0, 8.0]);
    }


    #[test]
    fn mul() {
        let v1 = Vec3{
            e: [2.0, 6.0, 9.0],
        };

        let v2 = Vec3{
            e: [1.0, 1.5, 2.0],
        };

        let v3 = v1 * v2;
        let v4 = 1.5 * v3.clone();

        assert_eq!(v3.e, [2.0, 9.0, 18.0]);
        assert_eq!(v4.e, [3.0, 13.5, 27.0]);
    }


    #[test]
    fn div() {
        let v1 = Vec3{
            e: [2.0, 6.0, 9.0],
        };

        let v2 = Vec3{
            e: [2.0, 2.0, 2.0],
        };

        let v3 = v1 / v2;

        assert_eq!(v3.e, [1.0, 3.0, 4.5]);
    }

    #[test]
    fn index() {
        let v1 = Vec3 {
            e: [1.5, 3.6, 6.8]
        };

        let i = 2;

        assert_eq!(v1[i], v1.e[i])
    }

    #[test]
    fn dot() {
        let v1 = Vec3 {
            e: [1.5, 0.6, 1.2]
        };

        let v2 = Vec3 {
            e: [1.0, 2.5, 2.2]
        };

        assert_eq!(v1.dot(v2), 9.0) 
    }


    #[test]
    fn cross() {
        let v1 = Vec3 {
            e: [1.5, 0.6, 1.2]
        };

        let v2 = Vec3 {
            e: [1.0, 2.5, 2.2]
        };

        let v3 = v1.cross(v2);

        assert_float_absolute_eq!(v3.x(), -1.68);
        assert_float_absolute_eq!(v3.y(), -2.1);
        assert_float_absolute_eq!(v3.z(), 3.15);
    }

    #[test]
    fn unit_vector() {
        let v1 = Vec3 {
            e: [1.2, 2.0, 0.5]
        };

        let unit = v1.unit();

        assert_float_absolute_eq!(unit.e[0], 0.503066);
        assert_float_absolute_eq!(unit.e[1], 0.838444);
        assert_float_absolute_eq!(unit.e[2], 0.209611);

    }
}
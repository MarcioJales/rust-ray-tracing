use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Debug, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    fn x(self) -> f64 {
        self.e[0]
    }

    fn y(self) -> f64 {
        self.e[1]
    }

    fn z(self) -> f64 {
        self.e[2]
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
    fn sum_sub() {
        let v1 = Vec3{
            e: [2.0, 6.0, 9.0],
        };

        let v2 = Vec3{
            e: [1.0, 1.0, 1.0],
        };

        let v3 = v1.clone() + v2.clone();
        let v4 = v1 - v2;

        assert_eq!(v3.e, [3.0, 7.0, 10.0]);
        assert_eq!(v4.e, [1.0, 5.0, 8.0]);
    }

    #[test]
    fn index() {
        let v1 = Vec3 {
            e: [1.5, 3.6, 6.8]
        };

        let i = 2;

        assert_eq!(v1[i], v1.e[i])
    }
}
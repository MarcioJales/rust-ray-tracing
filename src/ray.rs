use crate::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3
}

impl Ray {
    pub fn origin(self) -> Vec3 { 
        self.orig
    }

    pub fn direction(self) -> Vec3 { 
        self.dir
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}

/*******************/
// TEST SECTION
/*******************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_at() {

        let r = Ray {
            orig: Vec3 {
                e: [1.0, 1.0, 0.0]
            },
            dir: Vec3 {
                e: [1.0, 2.2, 0.5]
            }
        };

        let ray = r.at(5.0);

        assert_float_absolute_eq!(ray.e[0], 6.0);
        assert_float_absolute_eq!(ray.e[1], 12.0);
        assert_float_absolute_eq!(ray.e[2], 2.5)
    }
}
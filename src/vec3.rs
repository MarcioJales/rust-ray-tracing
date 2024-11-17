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
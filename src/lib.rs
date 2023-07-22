use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

/**
 * Vector Addition
 */

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

/**
 * Scalar Multiply/Divide
 */

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        return Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, scalar: Vec3) -> Vec3 {
        return Vec3 {
            x: self * scalar.x,
            y: self * scalar.y,
            z: self * scalar.z,
        };
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        return Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        };
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, scalar: Vec3) -> Vec3 {
        return Vec3 {
            x: self / scalar.x,
            y: self / scalar.y,
            z: self / scalar.z,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn vector_addition() {
        let v1 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 2.0,
        };
        let v2 = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let result;

        // Vec3 + Vec3
        result = v1 + v2;
        assert_eq!(
            Vec3 {
                x: 3.0,
                y: 5.0,
                z: 7.0
            },
            result
        );
    }

    #[test]
    fn scalar_multiplication() {
        let v1 = Vec3 {
            x: -1.0,
            y: 0.0,
            z: 2.0,
        };
        let mut result: Vec3;

        // Vec3 * f32
        result = v1 * 2.0;
        assert_eq!(
            Vec3 {
                x: -2.0,
                y: 0.0,
                z: 4.0,
            },
            result
        );

        // f32 * Vec3
        result = 2.0 * v1;
        assert_eq!(
            Vec3 {
                x: -2.0,
                y: 0.0,
                z: 4.0,
            },
            result
        );

        // Vec3 / f32
        result = v1 / 2.0;
        assert_eq!(
            Vec3 {
                x: -0.5,
                y: 0.0,
                z: 1.0,
            },
            result
        );

        // f32 / Vec3
        result = 2.0 / v1;
        println!("${:?}", result);
        assert_eq!(
            Vec3 {
                x: -2.0,
                y: f32::INFINITY,
                z: 1.0,
            },
            result
        );
    }
}

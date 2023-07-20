use std::ops;

#[derive(Debug, Copy, Clone)]
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
        println!("> Vec3.add(Vec3) was called");

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
        println!("> Vec3.mul(f32) was called");

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
        println!("> f32.mul(Vec3) was called");

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
        println!("> Vec3.div(f32) was called");

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
        println!("> f32.div(Vec3) was called");

        return Vec3 {
            x: self / scalar.x,
            y: self / scalar.y,
            z: self / scalar.z,
        };
    }
}

fn main() {
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

    let mut v3;

    // Vec3 + Vec3
    v3 = v1 + v2;
    println!("{:?}", v3);

    // Vec3 * f32
    v3 = v1 * 2.0;
    println!("{:?}", v3);

    // f32 * Vec3
    v3 = 2.0 * v1;
    println!("{:?}", v3);

    // Vec3 / f32
    v3 = v1 / 2.0;
    println!("{:?}", v3);

    // f32 / Vec3
    v3 = 2.0 / v1;
    println!("{:?}", v3);
}

use serde::{Deserialize, Serialize};

/// Vektor 3D
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    pub const UP: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const DOWN: Vec3 = Vec3 { x: 0.0, y: -1.0, z: 0.0 };
    pub const FORWARD: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
    pub const BACK: Vec3 = Vec3 { x: 0.0, y: 0.0, z: -1.0 };
    pub const RIGHT: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const LEFT: Vec3 = Vec3 { x: -1.0, y: 0.0, z: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.length();
        if len > 1e-6 {
            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Vec3::ZERO
        }
    }

    pub fn distance(self, other: Vec3) -> f32 {
        (self - other).length()
    }

    pub fn lerp(self, other: Vec3, t: f32) -> Vec3 {
        self + (other - self) * t
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

/// Kuaternion untuk Rotasi 3D Halus Bebas Gimbal Lock
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Quat = Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Membuat Kuaternion dari Sudut Euler (Yaw, Pitch, Roll dalam radian)
    pub fn from_euler(yaw: f32, pitch: f32, roll: f32) -> Self {
        let cy = (yaw * 0.5).cos();
        let sy = (yaw * 0.5).sin();
        let cp = (pitch * 0.5).cos();
        let sp = (pitch * 0.5).sin();
        let cr = (roll * 0.5).cos();
        let sr = (roll * 0.5).sin();

        Quat {
            w: cr * cp * cy + sr * sp * sy,
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
        }
    }

    /// Membuat Kuaternion dari Sumbu & Sudut
    pub fn from_axis_angle(axis: Vec3, angle_rad: f32) -> Self {
        let norm_axis = axis.normalize();
        let half_angle = angle_rad * 0.5;
        let s = half_angle.sin();
        Quat {
            x: norm_axis.x * s,
            y: norm_axis.y * s,
            z: norm_axis.z * s,
            w: half_angle.cos(),
        }
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 1e-6 {
            Quat {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        } else {
            Quat::IDENTITY
        }
    }

    /// Spherical Linear Interpolation (Slerp) untuk Animasi Skeletal 3D
    pub fn slerp(self, mut other: Quat, t: f32) -> Quat {
        let mut dot = self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;

        // Jika dot negatif, ambil arah rotasi terpendek
        if dot < 0.0 {
            other = Quat { x: -other.x, y: -other.y, z: -other.z, w: -other.w };
            dot = -dot;
        }

        if dot > 0.9995 {
            // Linear interpolation jika sangat dekat
            let res = Quat {
                x: self.x + (other.x - self.x) * t,
                y: self.y + (other.y - self.y) * t,
                z: self.z + (other.z - self.z) * t,
                w: self.w + (other.w - self.w) * t,
            };
            return res.normalize();
        }

        let theta_0 = dot.clamp(-1.0, 1.0).acos();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();

        let s0 = (theta_0 - theta).cos() - dot * sin_theta / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;

        Quat {
            x: (s0 * self.x) + (s1 * other.x),
            y: (s0 * self.y) + (s1 * other.y),
            z: (s0 * self.z) + (s1 * other.z),
            w: (s0 * self.w) + (s1 * other.w),
        }.normalize()
    }

    /// Konversi Kuaternion ke Matriks Transformasi Rotasi 4x4
    pub fn to_mat4(self) -> Mat4 {
        let x2 = self.x + self.x;
        let y2 = self.y + self.y;
        let z2 = self.z + self.z;
        let xx = self.x * x2;
        let xy = self.x * y2;
        let xz = self.x * z2;
        let yy = self.y * y2;
        let yz = self.y * z2;
        let zz = self.z * z2;
        let wx = self.w * x2;
        let wy = self.w * y2;
        let wz = self.w * z2;

        Mat4([
            1.0 - (yy + zz), xy + wz, xz - wy, 0.0,
            xy - wz, 1.0 - (xx + zz), yz + wx, 0.0,
            xz + wy, yz - wx, 1.0 - (xx + yy), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }
}

/// Matriks Transformasi 4x4 Row-Major
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Mat4(pub [f32; 16]);

impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);

    pub fn from_translation(v: Vec3) -> Self {
        Mat4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            v.x, v.y, v.z, 1.0,
        ])
    }

    pub fn from_scale(s: Vec3) -> Self {
        Mat4([
            s.x, 0.0, 0.0, 0.0,
            0.0, s.y, 0.0, 0.0,
            0.0, 0.0, s.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    /// Matriks Kamera Perspektif (Field-Of-View, Aspect Ratio, Near, Far)
    pub fn perspective_fov(fov_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_rad * 0.5).tan();
        let range_inv = 1.0 / (near - far);

        Mat4([
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (near + far) * range_inv, -1.0,
            0.0, 0.0, (2.0 * near * far) * range_inv, 0.0,
        ])
    }

    /// Matriks Kamera LookAt (Eye Position, Target Center, Up Vector)
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Mat4([
            s.x, u.x, -f.x, 0.0,
            s.y, u.y, -f.y, 0.0,
            s.z, u.z, -f.z, 0.0,
            -s.dot(eye), -u.dot(eye), f.dot(eye), 1.0,
        ])
    }

    pub fn multiply(&self, rhs: &Mat4) -> Mat4 {
        let mut out = [0.0f32; 16];
        for i in 0..4 {
            for j in 0..4 {
                out[i * 4 + j] = self.0[i * 4 + 0] * rhs.0[0 * 4 + j]
                    + self.0[i * 4 + 1] * rhs.0[1 * 4 + j]
                    + self.0[i * 4 + 2] * rhs.0[2 * 4 + j]
                    + self.0[i * 4 + 3] * rhs.0[3 * 4 + j];
            }
        }
        Mat4(out)
    }

    pub fn transform_point(&self, p: Vec3) -> Vec3 {
        let x = p.x * self.0[0] + p.y * self.0[4] + p.z * self.0[8] + self.0[12];
        let y = p.x * self.0[1] + p.y * self.0[5] + p.z * self.0[9] + self.0[13];
        let z = p.x * self.0[2] + p.y * self.0[6] + p.z * self.0[10] + self.0[14];
        let w = p.x * self.0[3] + p.y * self.0[7] + p.z * self.0[11] + self.0[15];
        if w.abs() > 1e-6 {
            Vec3::new(x / w, y / w, z / w)
        } else {
            Vec3::new(x, y, z)
        }
    }
}

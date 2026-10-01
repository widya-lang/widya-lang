use crate::game3d::math3d::Vec3;
use serde::{Deserialize, Serialize};

/// Axis-Aligned Bounding Box (AABB)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn extents(&self) -> Vec3 {
        (self.max - self.min) * 0.5
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x
            && self.min.y <= other.max.y && self.max.y >= other.min.y
            && self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}

/// Sphere Collider 3D
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SphereCollider {
    pub center: Vec3,
    pub radius: f32,
}

impl SphereCollider {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn intersects_sphere(&self, other: &SphereCollider) -> bool {
        let dist_sq = (self.center - other.center).length_squared();
        let rad_sum = self.radius + other.radius;
        dist_sq <= (rad_sum * rad_sum)
    }

    pub fn intersects_aabb(&self, aabb: &AABB) -> bool {
        let closest = Vec3::new(
            self.center.x.clamp(aabb.min.x, aabb.max.x),
            self.center.y.clamp(aabb.min.y, aabb.max.y),
            self.center.z.clamp(aabb.min.z, aabb.max.z),
        );
        (self.center - closest).length_squared() <= (self.radius * self.radius)
    }
}

/// Capsule Collider 3D (Untuk Karakter Manusia)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CapsuleCollider {
    pub p1: Vec3, // Titik bawah
    pub p2: Vec3, // Titik atas
    pub radius: f32,
}

impl CapsuleCollider {
    pub fn new(p1: Vec3, p2: Vec3, radius: f32) -> Self {
        Self { p1, p2, radius }
    }
}

/// Sinar Ray 3D untuk Tembakan Senjata & Raycast Pick
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray3D {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaycastHit3D {
    pub hit: bool,
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

impl Ray3D {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Uji Tembus Sinar ke Bola (Ray-Sphere Intersection)
    pub fn intersects_sphere(&self, sphere: &SphereCollider) -> Option<RaycastHit3D> {
        let oc = self.origin - sphere.center;
        let b = oc.dot(self.direction);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - c;

        if discriminant < 0.0 {
            return None;
        }

        let t = -b - discriminant.sqrt();
        if t > 0.0 {
            let hit_point = self.origin + self.direction * t;
            let normal = (hit_point - sphere.center).normalize();
            Some(RaycastHit3D {
                hit: true,
                point: hit_point,
                normal,
                distance: t,
            })
        } else {
            None
        }
    }

    /// Uji Tembus Sinar ke Kotak AABB (Ray-AABB Slab Method)
    pub fn intersects_aabb(&self, aabb: &AABB) -> Option<RaycastHit3D> {
        let mut t_min = (aabb.min.x - self.origin.x) / self.direction.x.max(1e-6);
        let mut t_max = (aabb.max.x - self.origin.x) / self.direction.x.max(1e-6);

        if t_min > t_max { std::mem::swap(&mut t_min, &mut t_max); }

        let mut ty_min = (aabb.min.y - self.origin.y) / self.direction.y.max(1e-6);
        let mut ty_max = (aabb.max.y - self.origin.y) / self.direction.y.max(1e-6);

        if ty_min > ty_max { std::mem::swap(&mut ty_min, &mut ty_max); }

        if (t_min > ty_max) || (ty_min > t_max) {
            return None;
        }

        if ty_min > t_min { t_min = ty_min; }
        if ty_max < t_max { t_max = ty_max; }

        let mut tz_min = (aabb.min.z - self.origin.z) / self.direction.z.max(1e-6);
        let mut tz_max = (aabb.max.z - self.origin.z) / self.direction.z.max(1e-6);

        if tz_min > tz_max { std::mem::swap(&mut tz_min, &mut tz_max); }

        if (t_min > tz_max) || (tz_min > t_max) {
            return None;
        }

        if tz_min > t_min { t_min = tz_min; }

        if t_min > 0.0 {
            let point = self.origin + self.direction * t_min;
            Some(RaycastHit3D {
                hit: true,
                point,
                normal: Vec3::UP,
                distance: t_min,
            })
        } else {
            None
        }
    }
}

/// Benda Tegar Fisika 3D (3D Rigid Body)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RigidBody3D {
    pub position: Vec3,
    pub linear_velocity: Vec3,
    pub mass: f32,
    pub restitution: f32, // Bounciness 0.0 (tidak memantul) .. 1.0 (pantulan sempurna)
    pub friction: f32,
    pub is_static: bool,
    pub use_gravity: bool,
    pub collider_radius: f32,
}

impl Default for RigidBody3D {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            linear_velocity: Vec3::ZERO,
            mass: 1.0,
            restitution: 0.5,
            friction: 0.2,
            is_static: false,
            use_gravity: true,
            collider_radius: 0.5,
        }
    }
}

impl RigidBody3D {
    pub fn new_dynamic(pos: Vec3, mass: f32, radius: f32) -> Self {
        Self {
            position: pos,
            mass,
            collider_radius: radius,
            ..Default::default()
        }
    }

    pub fn new_static(pos: Vec3, radius: f32) -> Self {
        Self {
            position: pos,
            is_static: true,
            use_gravity: false,
            collider_radius: radius,
            ..Default::default()
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        if !self.is_static && self.mass > 0.0 {
            self.linear_velocity = self.linear_velocity + (force / self.mass);
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vec3) {
        if !self.is_static && self.mass > 0.0 {
            self.linear_velocity = self.linear_velocity + (impulse / self.mass);
        }
    }
}

/// Simulasi Langkah Fisika 3D (60/120 Hz Fixed Timestep Step)
pub fn langkah_simulasi_fisika_3d(bodies: &mut [RigidBody3D], dt: f32, gravity: Vec3) {
    let num_bodies = bodies.len();

    // 1. Integrasi Kecepatan & Posisi
    for body in bodies.iter_mut() {
        if body.is_static {
            continue;
        }

        if body.use_gravity {
            body.linear_velocity = body.linear_velocity + (gravity * dt);
        }

        body.position = body.position + (body.linear_velocity * dt);

        // Ground collision plane sederhana di Y = 0
        if body.position.y - body.collider_radius < 0.0 {
            body.position.y = body.collider_radius;
            if body.linear_velocity.y < 0.0 {
                body.linear_velocity.y = -body.linear_velocity.y * body.restitution;
                // Gesekan tanah
                body.linear_velocity.x *= 1.0 - (body.friction * dt * 5.0).clamp(0.0, 1.0);
                body.linear_velocity.z *= 1.0 - (body.friction * dt * 5.0).clamp(0.0, 1.0);
            }
        }
    }

    // 2. Resolusi Tabrakan Antar Objek (Sphere-Sphere Impulse Response)
    for i in 0..num_bodies {
        for j in (i + 1)..num_bodies {
            let (left, right) = bodies.split_at_mut(j);
            let b1 = &mut left[i];
            let b2 = &mut right[0];

            let delta = b2.position - b1.position;
            let dist_sq = delta.length_squared();
            let min_dist = b1.collider_radius + b2.collider_radius;

            if dist_sq < (min_dist * min_dist) && dist_sq > 1e-6 {
                let dist = dist_sq.sqrt();
                let normal = delta / dist;
                let penetration = min_dist - dist;

                // Pisahkan posisi agar tidak tembus
                if !b1.is_static && !b2.is_static {
                    b1.position = b1.position - (normal * (penetration * 0.5));
                    b2.position = b2.position + (normal * (penetration * 0.5));
                } else if !b1.is_static {
                    b1.position = b1.position - (normal * penetration);
                } else if !b2.is_static {
                    b2.position = b2.position + (normal * penetration);
                }

                // Hitung impuls elastis
                let rel_vel = b2.linear_velocity - b1.linear_velocity;
                let vel_along_normal = rel_vel.dot(normal);

                if vel_along_normal < 0.0 {
                    let restitution = b1.restitution.min(b2.restitution);
                    let inv_mass1 = if b1.is_static { 0.0 } else { 1.0 / b1.mass };
                    let inv_mass2 = if b2.is_static { 0.0 } else { 1.0 / b2.mass };
                    let impulse_scalar = -(1.0 + restitution) * vel_along_normal / (inv_mass1 + inv_mass2);

                    let impulse = normal * impulse_scalar;
                    if !b1.is_static {
                        b1.linear_velocity = b1.linear_velocity - (impulse * inv_mass1);
                    }
                    if !b2.is_static {
                        b2.linear_velocity = b2.linear_velocity + (impulse * inv_mass2);
                    }
                }
            }
        }
    }
}

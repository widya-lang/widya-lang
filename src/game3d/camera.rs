use crate::game3d::math3d::{Mat4, Vec3};
use crate::game3d::physics3d::{SphereCollider, AABB};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera3D {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov_degrees: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Default for Camera3D {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 5.0, -10.0),
            target: Vec3::ZERO,
            up: Vec3::UP,
            fov_degrees: 60.0,
            aspect_ratio: 16.0 / 9.0,
            near_plane: 0.1,
            far_plane: 1000.0,
        }
    }
}

impl Camera3D {
    pub fn new_perspective(pos: Vec3, target: Vec3, fov_deg: f32, aspect: f32) -> Self {
        Self {
            position: pos,
            target,
            fov_degrees: fov_deg,
            aspect_ratio: aspect,
            ..Default::default()
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.target, self.up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let fov_rad = self.fov_degrees.to_radians();
        Mat4::perspective_fov(fov_rad, self.aspect_ratio, self.near_plane, self.far_plane)
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.view_matrix().multiply(&self.projection_matrix())
    }

    /// Pemeriksaan Frustum Culling Sederhana untuk Mengabaikan Objek di Luar Pandangan
    pub fn is_sphere_in_frustum(&self, sphere: &SphereCollider) -> bool {
        let to_sphere = sphere.center - self.position;
        let forward = (self.target - self.position).normalize();
        let forward_dist = to_sphere.dot(forward);

        // Jika di belakang near plane atau di luar far plane
        if forward_dist + sphere.radius < self.near_plane || forward_dist - sphere.radius > self.far_plane {
            return false;
        }

        true
    }
}

/// 3D Spatial Octree untuk Akselerasi Rendering & Culling Jutaan Objek
#[derive(Debug, Clone)]
pub struct OctreeNode<T: Clone> {
    pub bounds: AABB,
    pub items: Vec<(Vec3, T)>,
    pub children: Option<Box<[OctreeNode<T>; 8]>>,
}

impl<T: Clone> OctreeNode<T> {
    pub fn new(bounds: AABB) -> Self {
        Self {
            bounds,
            items: Vec::new(),
            children: None,
        }
    }

    pub fn insert(&mut self, pos: Vec3, item: T) {
        if self.items.len() < 8 && self.children.is_none() {
            self.items.push((pos, item));
            return;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.bounds.intersects(&AABB::new(pos - Vec3::new(0.1, 0.1, 0.1), pos + Vec3::new(0.1, 0.1, 0.1))) {
                    child.insert(pos, item.clone());
                    break;
                }
            }
        }
    }

    fn subdivide(&mut self) {
        let center = self.bounds.center();
        let min = self.bounds.min;
        let max = self.bounds.max;

        let octants = [
            AABB::new(min, center),
            AABB::new(Vec3::new(center.x, min.y, min.z), Vec3::new(max.x, center.y, center.z)),
            AABB::new(Vec3::new(min.x, center.y, min.z), Vec3::new(center.x, max.y, center.z)),
            AABB::new(Vec3::new(center.x, center.y, min.z), Vec3::new(max.x, max.y, center.z)),
            AABB::new(Vec3::new(min.x, min.y, center.z), Vec3::new(center.x, center.y, max.z)),
            AABB::new(Vec3::new(center.x, min.y, center.z), Vec3::new(max.x, center.y, max.z)),
            AABB::new(Vec3::new(min.x, center.y, center.z), Vec3::new(center.x, max.y, max.z)),
            AABB::new(center, max),
        ];

        let children = Box::new([
            OctreeNode::new(octants[0]),
            OctreeNode::new(octants[1]),
            OctreeNode::new(octants[2]),
            OctreeNode::new(octants[3]),
            OctreeNode::new(octants[4]),
            OctreeNode::new(octants[5]),
            OctreeNode::new(octants[6]),
            OctreeNode::new(octants[7]),
        ]);

        self.children = Some(children);

        let old_items = std::mem::take(&mut self.items);
        for (p, item) in old_items {
            self.insert(p, item);
        }
    }
}

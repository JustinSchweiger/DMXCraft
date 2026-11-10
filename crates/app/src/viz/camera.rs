//! Orbiting camera (as in Easy View): rotate, pan, zoom.

use glam::{Mat4, Vec2, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbitCamera {
    pub target: Vec3,
    /// Degrees around the vertical axis.
    pub yaw: f32,
    /// Degrees, positive = from above.
    pub pitch: f32,
    pub distance: f32,
    /// Vertical field of view (degrees).
    pub fov: f32,
}

impl OrbitCamera {
    /// View at an angle from the front and above onto a room of this size.
    pub fn for_room(size: [f32; 3]) -> Self {
        let [w, h, d] = size;
        Self {
            target: Vec3::new(0.0, h * 0.3, 0.0),
            yaw: 215.0,
            pitch: 32.0,
            distance: (w * w + d * d).sqrt() * 1.05 + h,
            fov: 45.0,
        }
    }

    /// From a saved camera point.
    pub fn from_preset(p: &dmxcraft_core::venue::CameraPreset) -> Self {
        Self {
            target: Vec3::from(p.target),
            yaw: p.yaw,
            pitch: p.pitch,
            distance: p.distance,
            fov: p.fov,
        }
    }

    /// Save as a camera point.
    pub fn to_preset(self, name: impl Into<String>) -> dmxcraft_core::venue::CameraPreset {
        let r = |v: f32| (v * 1000.0).round() / 1000.0;
        dmxcraft_core::venue::CameraPreset {
            name: name.into(),
            target: self.target.to_array().map(r),
            yaw: r(self.yaw),
            pitch: r(self.pitch),
            distance: r(self.distance),
            fov: r(self.fov),
        }
    }

    /// Almost equal (for marking the active camera point).
    pub fn near(&self, o: &Self) -> bool {
        let dyaw = ((self.yaw - o.yaw + 540.0).rem_euclid(360.0) - 180.0).abs();
        dyaw < 0.5
            && (self.pitch - o.pitch).abs() < 0.5
            && (self.distance - o.distance).abs() < 0.05
            && self.target.distance(o.target) < 0.05
            && (self.fov - o.fov).abs() < 0.5
    }

    /// Intermediate position for camera moves (`k` 0–1, rotation the short way).
    pub fn lerp(&self, o: &Self, k: f32) -> Self {
        let dyaw = (o.yaw - self.yaw + 540.0).rem_euclid(360.0) - 180.0;
        Self {
            target: self.target.lerp(o.target, k),
            yaw: (self.yaw + dyaw * k).rem_euclid(360.0),
            pitch: self.pitch + (o.pitch - self.pitch) * k,
            distance: self.distance + (o.distance - self.distance) * k,
            fov: self.fov + (o.fov - self.fov) * k,
        }
    }

    pub fn eye(&self) -> Vec3 {
        let (y, p) = (self.yaw.to_radians(), self.pitch.to_radians());
        self.target + Vec3::new(p.cos() * y.sin(), p.sin(), p.cos() * y.cos()) * self.distance
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye(), self.target, Vec3::Y)
    }

    pub fn proj(&self, aspect: f32) -> Mat4 {
        Mat4::perspective_rh(self.fov.to_radians(), aspect.max(0.01), 0.05, 500.0)
    }

    pub fn rotate(&mut self, d: Vec2) {
        self.yaw = (self.yaw - d.x * 0.35).rem_euclid(360.0);
        self.pitch = (self.pitch + d.y * 0.35).clamp(-89.0, 89.0);
    }

    /// Pan in the image plane (pixels → meters depending on the distance).
    pub fn pan(&mut self, d: Vec2, viewport_h: f32) {
        let k = self.distance * (self.fov.to_radians() * 0.5).tan() * 2.0 / viewport_h.max(1.0);
        let v = self.view();
        let right = Vec3::new(v.x_axis.x, v.y_axis.x, v.z_axis.x);
        let up = Vec3::new(v.x_axis.y, v.y_axis.y, v.z_axis.y);
        self.target += (-right * d.x + up * d.y) * k;
    }

    pub fn zoom(&mut self, factor: f32) {
        self.distance = (self.distance * factor).clamp(0.5, 300.0);
    }

    /// Ray through a point in the image (0–1, top left = 0,0).
    pub fn ray(&self, uv: Vec2, aspect: f32) -> (Vec3, Vec3) {
        let inv = (self.proj(aspect) * self.view()).inverse();
        let ndc = Vec2::new(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0);
        let near = inv.project_point3(ndc.extend(0.0));
        let far = inv.project_point3(ndc.extend(1.0));
        (near, (far - near).normalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_through_center_hits_target() {
        let c = OrbitCamera::for_room([10.0, 4.0, 12.0]);
        let (o, d) = c.ray(Vec2::splat(0.5), 1.5);
        let to = (c.target - o).normalize();
        assert!(d.dot(to) > 0.999);
    }
}

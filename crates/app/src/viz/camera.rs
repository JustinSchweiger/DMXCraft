//! Kreisende Kamera (wie in Easy View): drehen, verschieben, zoomen.

use glam::{Mat4, Vec2, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbitCamera {
    pub target: Vec3,
    /// Grad um die Hochachse.
    pub yaw: f32,
    /// Grad, positiv = von oben.
    pub pitch: f32,
    pub distance: f32,
    /// Öffnungswinkel senkrecht (Grad).
    pub fov: f32,
}

impl OrbitCamera {
    /// Blick schräg von vorn oben auf einen Raum dieser Größe.
    pub fn for_room(size: [f32; 3]) -> Self {
        let [w, h, d] = size;
        Self {
            target: Vec3::new(0.0, h * 0.3, 0.0),
            yaw: 140.0,
            pitch: 32.0,
            distance: (w * w + d * d).sqrt() * 1.05 + h,
            fov: 45.0,
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

    /// Verschieben in der Bildebene (Pixel → Meter abhängig vom Abstand).
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

    /// Strahl durch einen Punkt im Bild (0–1, oben links = 0,0).
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

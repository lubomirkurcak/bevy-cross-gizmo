use bevy::prelude::*;

pub trait BevyCrossGizmo {
    fn cross(&mut self, position: Vec3, size: f32, color: Color);
    fn cross_2d(&mut self, position: Vec2, size: f32, color: Color);
}

impl<'w, 's, T: GizmoConfigGroup> BevyCrossGizmo for Gizmos<'w, 's, T> {
    fn cross(&mut self, position: Vec3, size: f32, color: Color) {
        self.line(
            position + Vec3::new(size, 0.0, 0.0),
            position - Vec3::new(size, 0.0, 0.0),
            color,
        );
        self.line(
            position + Vec3::new(0.0, size, 0.0),
            position - Vec3::new(0.0, size, 0.0),
            color,
        );
        self.line(
            position + Vec3::new(0.0, 0.0, size),
            position - Vec3::new(0.0, 0.0, size),
            color,
        );
    }

    fn cross_2d(&mut self, position: Vec2, size: f32, color: Color) {
        self.line_2d(
            position + Vec2::new(size, 0.0),
            position - Vec2::new(size, 0.0),
            color,
        );
        self.line_2d(
            position + Vec2::new(0.0, size),
            position - Vec2::new(0.0, size),
            color,
        );
    }
}

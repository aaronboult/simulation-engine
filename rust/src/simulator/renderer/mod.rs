pub mod geometry;
use geometry::{Matrix, Vector2D, Vector3D};

#[allow(dead_code)]
pub enum RenderMode {
    R2D,
    R3D
}

pub struct Camera {
    transform: Transform,
    view_plane_top_left: Vector2D,
    view_plane_bottom_right: Vector2D,
    view_plane_z: f32
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            view_plane_top_left: Vector2D::new(-1.0, 1.0),
            view_plane_bottom_right: Vector2D::new(1.0, -1.0),
            view_plane_z: -1.0
        }
    }
}

pub struct Renderer {
    mode: RenderMode,
    camera: Camera
}

impl Renderer {
    pub fn new(mode: RenderMode) -> Self {
        Self {
            mode,
            camera: Camera::new()
        }
    }

    pub fn project_to_screen(&self, transform: &Transform, vertex: &Vector3D, window_size: (u32, u32)) -> (i32, i32) {
        match self.mode {
            RenderMode::R2D => {
                (-1, -1)
            },
            RenderMode::R3D => {
                self.calculate_3d_projection(transform, vertex, window_size)
            }
        }
    }

    fn calculate_3d_projection(&self, transform: &Transform, vertex: &Vector3D, window_size: (u32, u32)) -> (i32, i32) {

        let point = Vector3D::new(
            vertex.x,
            vertex.y,
            vertex.z
        );

        let rotation = transform.rotation * (std::f32::consts::PI / 180.0);
    
        // scale the initial point
        let projected_point = Matrix::from(point * transform.scale);

        let x_rotation_matrix = Matrix::from_vec(3, 3, vec![
            1.0, 0.0, 0.0,
            0.0, rotation.x.cos(), -rotation.x.sin(),
            0.0, rotation.x.sin(), rotation.x.cos()
        ]);

        let y_rotation_matrix = Matrix::from_vec(3, 3, vec![
            rotation.y.cos(), 0.0, -rotation.y.sin(),
            0.0, 1.0, 0.0,
            rotation.y.sin(), 0.0, rotation.y.cos()
        ]);

        let z_rotation_matrix = Matrix::from_vec(3, 3, vec![
            rotation.z.cos(), -rotation.z.sin(), 0.0,
            rotation.z.sin(), rotation.z.cos(), 0.0,
            0.0, 0.0, 1.0
        ]);

        let rotation_matrix = x_rotation_matrix * y_rotation_matrix * z_rotation_matrix;

        // multiplying rotation matrix with 3d column vector 
        // produces another 3d column vector
        let rotated_projection = Vector3D::from(rotation_matrix * projected_point)
            + transform.position
            - self.camera.transform.position;

        let recording_screen_size = self.camera.view_plane_bottom_right
            - self.camera.view_plane_top_left;

        let mut x: f32 = -1.0;
        let mut y: f32 = -1.0;

        if rotated_projection.z != 0.0 {
            x = (rotated_projection.x * (window_size.0 as f32)) / (rotated_projection.z / recording_screen_size.x) * self.camera.view_plane_z;
            y = (rotated_projection.y * (window_size.1 as f32)) / (rotated_projection.z / recording_screen_size.y) * self.camera.view_plane_z;
        }

        (
            x as i32 + (window_size.0 / 2) as i32,
            y as i32 + (window_size.1 / 2) as i32,
        )
    
    }
}

#[derive(Default)]
pub struct Transform {
    pub position: Vector3D,
    pub rotation: Vector3D,
    pub scale: Vector3D
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector3D::ZERO,
            rotation: Vector3D::ZERO,
            scale: Vector3D::ONE
        }
    }
}
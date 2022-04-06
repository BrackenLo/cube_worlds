//====================================================================

//====================================================================

pub struct Camera {
    position: glam::Vec3,
    yaw: f32,   //Left and right
    pitch: f32, //Up and down
}

impl Camera {
    pub fn new(position: glam::Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            position,
            yaw, 
            pitch
        }
    }

    pub fn build_matrix(&self) -> glam::Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        glam::Mat4::look_at_rh(
            self.position, 
            self.position - glam::Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(), 
            glam::Vec3::Y
        )

        /*glam::Mat4::look_at_rh(
            self.position,
            glam::Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(), 
            glam::Vec3::Y,
        )*/
    }
}

pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(
        width: u32, height: u32, fovy: f32, znear: f32, zfar: f32
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn build_matrix(&self) -> glam::Mat4 {
        glam::Mat4::perspective_rh_gl(
            self.fovy, 
            self.aspect, 
            self.znear, 
            self.zfar
        )

        //glam::Mat4::perspective_rh(fov_y_radians, aspect_ratio, z_near, z_far)

        //glam::Mat4::perspective
    }
}

/* 
impl Camera {
    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {

        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = glam::Mat4::perspective_rh_gl(self.fovy, self.aspect, self.znear, self.zfar);

        return proj * view;
    }

    pub fn update_camera_position(&mut self) {

        let forward_norm = (self.target - self.eye).normalize();

        self.eye -= forward_norm * 0.4;

    }
}*/

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}
impl CameraUniform {
    pub fn _new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn from_camera(camera: &Camera, projection: &Projection) -> Self {
        Self {
            //view_proj: camera.build_view_projection_matrix().to_cols_array_2d(),
            view_proj: (projection.build_matrix() * camera.build_matrix()).to_cols_array_2d(),
        }
    }

    pub fn _update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_proj = (projection.build_matrix() * camera.build_matrix()).to_cols_array_2d();
    }
}

//====================================================================

pub struct CameraController {
    look_dir: glam::Vec2,
    move_dir: glam::Vec3,

    look_speed: f32,
    move_speed: f32,
}

impl CameraController {
    pub fn new(look_speed: f32, move_speed: f32) -> Self {
        Self {
            look_dir: glam::Vec2::ZERO,
            move_dir: glam::Vec3::ZERO,
            look_speed,
            move_speed,
        }
    }

    pub fn update(&mut self, camera: &mut Camera, inputs: &crate::core::state::InputController) {

        use winit::event::*;

        if inputs.key_pressed(&VirtualKeyCode::W) { self.move_dir.z -= self.move_speed }    //Forward
        if inputs.key_pressed(&VirtualKeyCode::S) { self.move_dir.z += self.move_speed }    //Backward
        if inputs.key_pressed(&VirtualKeyCode::A) { self.move_dir.x += self.move_speed }    //Left
        if inputs.key_pressed(&VirtualKeyCode::D) { self.move_dir.x -= self.move_speed }    //Right

        if inputs.key_pressed(&VirtualKeyCode::Space)   { self.move_dir.y += self.move_speed }  //Up
        if inputs.key_pressed(&VirtualKeyCode::LShift)  { self.move_dir.y -= self.move_speed }  //Down

        if inputs.key_pressed(&VirtualKeyCode::Up)      { self.look_dir.y -= self.look_speed }  //Look up
        if inputs.key_pressed(&VirtualKeyCode::Down)    { self.look_dir.y += self.look_speed }  //Look down
        if inputs.key_pressed(&VirtualKeyCode::Left)    { self.look_dir.x -= self.look_speed }  //Look left
        if inputs.key_pressed(&VirtualKeyCode::Right)   { self.look_dir.x += self.look_speed }  //Look right

        //let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        let forward = glam::Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = glam::Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward * self.move_dir.z;
        camera.position += right * self.move_dir.x;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += self.move_dir.y;

        // Rotate
        //camera.yaw += Rad(self.rotate_horizontal) * self.sensitivity * dt;
        //camera.pitch += Rad(-self.rotate_vertical) * self.sensitivity * dt;

        camera.yaw += self.look_dir.x.to_radians();
        camera.pitch += self.look_dir.y.to_radians();

        /*if self.move_dir != glam::Vec3::ZERO {
            println!("Camera pos = {}", camera.position);
        }*/

        self.look_dir = glam::Vec2::ZERO;
        self.move_dir = glam::Vec3::ZERO;
        
        let high = -65f32.to_radians();
        let low = 65f32.to_radians();

        // Keep the camera's angle from going too high/low.
        if camera.pitch < high {
            camera.pitch = high;
        } else if camera.pitch > low {
            camera.pitch = low;
        }

    }
}

//====================================================================
use std::f32::consts::{PI, TAU};

use ultraviolet::{projection, Mat4, Vec3};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{DeviceEvent, ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
};

#[inline]
fn clamp(mut x: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

pub trait Camera {
    fn resize(&mut self, size: PhysicalSize<u32>, fov: f32, near: f32);
    /// Return indicates whether the camera changed internally.
    fn update(&mut self, event: &Event<()>) -> bool;
    fn eye(&self) -> Vec3;
    fn matrix(&self) -> Mat4;
}

pub struct ArcballCamera {
    distance: f32,
    speed: f32,
    yaw: f32,
    pitch: f32,
    mouse_button_pressed: bool,

    projection_mat: Mat4,
}

impl ArcballCamera {
    pub fn new(distance: f32, speed: f32) -> Self {
        Self {
            distance,
            speed,
            yaw: 0.0,
            pitch: 0.0,
            mouse_button_pressed: false,

            projection_mat: Mat4::identity(),
        }
    }

    fn add_yaw(&mut self, dyaw: f32) {
        self.yaw = (self.yaw + dyaw) % TAU;
    }

    fn add_pitch(&mut self, dpitch: f32) {
        self.pitch = clamp(self.pitch + dpitch, (-PI / 2.0) + 0.001, (PI / 2.0) - 0.001);
    }
}

impl Camera for ArcballCamera {
    fn resize(&mut self, size: PhysicalSize<u32>, fov: f32, near: f32) {
        self.projection_mat = projection::perspective_infinite_z_wgpu_dx(
            fov,
            size.width as f32 / size.height as f32,
            near,
        );
    }

    fn update(&mut self, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        MouseScrollDelta::LineDelta(_, delta) => {
                            self.distance = (self.distance - delta * self.speed * 10.0).max(0.001);
                        }
                        &MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                            self.distance = (self.distance - y as f32 * self.speed).max(0.001);
                        }
                    }
                    true
                }
                &WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        if state == ElementState::Pressed {
                            self.mouse_button_pressed = true;
                        } else {
                            self.mouse_button_pressed = false;
                        }
                    }
                    false
                }
                WindowEvent::CursorLeft { .. } => {
                    self.mouse_button_pressed = false;
                    false
                }
                _ => false,
            },
            Event::DeviceEvent { event, .. } => match event {
                &DeviceEvent::MouseMotion { delta: (x, y) } => {
                    if self.mouse_button_pressed {
                        self.add_yaw(-x as f32 / 200.0);
                        self.add_pitch(-y as f32 / 200.0);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn eye(&self) -> Vec3 {
        self.distance
            * Vec3::new(
                self.yaw.sin() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.cos() * self.pitch.cos(),
            )
    }

    fn matrix(&self) -> Mat4 {
        Mat4::look_at(self.eye(), Vec3::zero(), Vec3::unit_y())
    }
}

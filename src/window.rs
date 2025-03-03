use std::{collections::HashMap, ffi::CString, sync::{Arc, LazyLock, Mutex}, time::Instant};

use gl::{BlendFunc, Enable};
use glam::{vec2, vec3, Mat4, Vec2, Vec3};
use glfw::{Action, Context, CursorMode, Glfw, GlfwReceiver, Key, MouseButton, PWindow, WindowEvent};

use crate::camera::{Camera, PROJ_MATRIX};

pub struct Window{
    pub w: u32,
    pub h: u32,
    pub window: PWindow,
    glfw: Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
    pub clear_color: Vec3,
    pub last_mouse_pos: Vec2,
    pub mouse_pos: Vec2,
    pub mouse_buttons: [bool; 8],
    pub mouse_scroll: [f32; 2],
    pub keyboard: HashMap<Key, Action>,
    pub dt: f32,
    pub time: f32,
    last_time: Instant,
    pub camera: Camera,
}

impl Window{
    pub fn new(w: u32, h: u32) -> Self{
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut window, events) = glfw.create_window(w, h, "What a pretty sight!", glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        window.make_current();
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);

        let mut keyboard = HashMap::new();

        // Populate key_state with all possible Key variants
        for key in all_keys() {
            keyboard.insert(key, Action::Release);
        }

        unsafe {
            Enable(gl::DEPTH_TEST);
            Enable(gl::BLEND);
            BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            Enable(gl::FRAMEBUFFER_SRGB);
        }

        Window {
            w,
            h,
            window,
            glfw,
            events,
            clear_color: vec3(0.01, 0.02, 0.03),
            last_mouse_pos: Vec2::ZERO,
            mouse_pos: Vec2::ZERO,
            mouse_buttons: [false; 8],
            mouse_scroll: [0.; 2],
            keyboard,
            dt: 0.,
            time: 0.,
            last_time: Instant::now(),
            camera: Camera::new(),
        }
    }

    pub fn should_close(&self) -> bool{
        self.window.should_close()
    }

    pub fn lock_cursor(&mut self){
        if self.window.get_cursor_mode() == CursorMode::Normal{
            self.window.set_cursor_mode(CursorMode::Hidden);
            self.window.set_cursor_pos(self.w as f64 / 2.0, self.h as f64 / 2.0);
        }
        else{
            self.window.set_cursor_mode(CursorMode::Normal);
        }

        //self.camera.fi
    }

    pub fn update(&mut self){
        let current_time = Instant::now();
        let elapsed = current_time.duration_since(self.last_time);
        self.dt = elapsed.as_secs_f32();

        self.last_time = current_time;

        self.camera.scroll_callback(self.mouse_scroll[1]*3.);

        self.time += self.dt;

        unsafe{
            PROJ_MATRIX = self.camera.view;
        }
        self.camera.update_matrix(self.w as f32, self.h as f32);

        self.window.swap_buffers();

        self.mouse_scroll[0] = 0.;
        self.mouse_scroll[1] = 0.;
        self.process_events();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn clear_screen(&self){
        unsafe{
            gl::ClearColor(self.clear_color.x, self.clear_color.y, self.clear_color.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn process_events(&mut self) {
        self.glfw.poll_events();

        self.last_mouse_pos = self.mouse_pos;
        self.mouse_pos = vec2(self.window.get_cursor_pos().0 as f32, self.window.get_cursor_pos().1 as f32);

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        self.w = width as u32;
                        self.h = height as u32;

                        gl::Viewport(0, 0, width, height);
                    }
                }

                glfw::WindowEvent::Key(key, _, action, _) => {
                    self.keyboard.insert(key, action);
                    match key{
                        Key::Escape => {
                            self.window.set_should_close(true)
                        }
                        _ => ()
                    }
                }

                glfw::WindowEvent::CursorPos(_, _) => {
                    if self.window.get_cursor_mode() == CursorMode::Hidden{
                        self.window.set_cursor_pos(self.w as f64 / 2.0, self.h as f64 / 2.0);
                        
                        let xoff = self.w as f32/2. - self.mouse_pos.x;
                        let yoff = self.h as f32/2. - self.mouse_pos.y;
                        
                        self.camera.process_mouse_movement(-xoff, yoff, true);
                    }
                }

                glfw::WindowEvent::MouseButton(button, action, _) => {
                    let is_pressed = if action == glfw::Action::Press {true} else {false};

                    match button {
                        glfw::MouseButton::Button1 => { 
                            self.mouse_buttons[0] = is_pressed;
                        }
                        glfw::MouseButton::Button2 => { 
                            self.mouse_buttons[1] = is_pressed;
                        }
                        glfw::MouseButton::Button3 => { 
                            self.mouse_buttons[2] = is_pressed;
                        }
                        glfw::MouseButton::Button4 => { 
                            self.mouse_buttons[3] = is_pressed;
                        }
                        glfw::MouseButton::Button5 => { 
                            self.mouse_buttons[4] = is_pressed;
                        }
                        glfw::MouseButton::Button6 => { 
                            self.mouse_buttons[5] = is_pressed;
                        }
                        glfw::MouseButton::Button7 => { 
                            self.mouse_buttons[6] = is_pressed;
                        }
                        glfw::MouseButton::Button8 => { 
                            self.mouse_buttons[7] = is_pressed;
                        }
                    }
                }

                glfw::WindowEvent::Scroll(x, y) => {
                    self.mouse_scroll[0] = x as f32;
                    self.mouse_scroll[1] = y as f32;
                }

                _ => {}
            }
        }
    }

    pub fn set_caption(&mut self, caption: &str){
        self.window.set_title(caption);
    }
}

// Well this was painful...
pub fn all_keys() -> Vec<Key>{
    vec![
        Key::Space,
        Key::Apostrophe,
        Key::Comma,
        Key::Minus,
        Key::Period,
        Key::Slash,
        Key::Num0,
        Key::Num1,
        Key::Num2,
        Key::Num3,
        Key::Num4,
        Key::Num5,
        Key::Num6,
        Key::Num7,
        Key::Num8,
        Key::Num9,
        Key::Semicolon,
        Key::Equal,
        Key::A,
        Key::B,
        Key::C,
        Key::D,
        Key::E,
        Key::F,
        Key::G,
        Key::H,
        Key::I,
        Key::J,
        Key::K,
        Key::L,
        Key::M,
        Key::N,
        Key::O,
        Key::P,
        Key::Q,
        Key::R,
        Key::S,
        Key::T,
        Key::U,
        Key::V,
        Key::W,
        Key::X,
        Key::Y,
        Key::Z,
        Key::LeftBracket,
        Key::Backslash,
        Key::RightBracket,
        Key::GraveAccent,
        Key::World1,
        Key::World2,
        Key::Escape,
        Key::Enter,
        Key::Tab,
        Key::Backspace,
        Key::Insert,
        Key::Delete,
        Key::Right,
        Key::Left,
        Key::Down,
        Key::Up,
        Key::PageUp,
        Key::PageDown,
        Key::Home,
        Key::End,
        Key::CapsLock,
        Key::ScrollLock,
        Key::NumLock,
        Key::PrintScreen,
        Key::Pause,
        Key::F1,
        Key::F2,
        Key::F3,
        Key::F4,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::F10,
        Key::F11,
        Key::F12,
        Key::F13,
        Key::F14,
        Key::F15,
        Key::F16,
        Key::F17,
        Key::F18,
        Key::F19,
        Key::F20,
        Key::F21,
        Key::F22,
        Key::F23,
        Key::F24,
        Key::F25,
        Key::Kp0,
        Key::Kp1,
        Key::Kp2,
        Key::Kp3,
        Key::Kp4,
        Key::Kp5,
        Key::Kp6,
        Key::Kp7,
        Key::Kp8,
        Key::Kp9,
        Key::KpDecimal,
        Key::KpDivide,
        Key::KpMultiply,
        Key::KpSubtract,
        Key::KpAdd,
        Key::KpEnter,
        Key::KpEqual,
        Key::LeftShift,
        Key::LeftControl,
        Key::LeftAlt,
        Key::LeftSuper,
        Key::RightShift,
        Key::RightControl,
        Key::RightAlt,
        Key::RightSuper,
        Key::Menu,
    ]
}

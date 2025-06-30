use std::{collections::HashMap, time::Instant};

use gl::{BlendFunc, Clear, ClearColor, Enable, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT};
use glam::{vec2, vec3, Vec2, Vec3};
use glfw::{Action, Context, CursorMode, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use imgui::Ui;

use crate::{camera::{Camera, PROJ_MATRIX}, ImguiRenderer};

pub struct Window{
    pub w: u32,
    pub h: u32,
    pub window: PWindow,
    glfw: Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
    clear_color: Vec3,
    pub last_mouse_pos: Vec2,
    pub mouse_pos: Vec2,
    pub mouse_buttons: [bool; 8],
    pub mouse_scroll: [f32; 2],
    pub keyboard: HashMap<Key, Action>,
    pub dt: f32,
    pub time: f32,
    last_time: Instant,
    pub camera: Camera,

    pub imgui: imgui::Context,
    pub imgui_renderer: ImguiRenderer,
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

        let mut keyboard: HashMap<Key, Action> = HashMap::new();
        keyboard.insert(Key::W, Action::Release);
        keyboard.insert(Key::S, Action::Release);
        keyboard.insert(Key::D, Action::Release);
        keyboard.insert(Key::A, Action::Release);
        keyboard.insert(Key::Space, Action::Release);
        keyboard.insert(Key::LeftControl, Action::Release);

        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        
        window.make_current();

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let imgui_renderer = ImguiRenderer::new(&mut imgui, |s| {
            glfw.get_proc_address_raw(s) as *const _
        });

        let clear_color = vec3(0.15, 0.17, 0.21);

        unsafe {
            Enable(gl::DEPTH_TEST);
            Enable(gl::BLEND);
            BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            ClearColor(clear_color.x, clear_color.y, clear_color.z, 1.0);
        }

        Window {
            w,
            h,
            window,
            glfw,
            events,
            clear_color,
            last_mouse_pos: Vec2::ZERO,
            mouse_pos: Vec2::ZERO,
            mouse_buttons: [false; 8],
            mouse_scroll: [0.; 2],
            keyboard,
            dt: 0.,
            time: 0.,
            last_time: Instant::now(),
            camera: Camera::new(),

            imgui,
            imgui_renderer,
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
    }

    pub fn set_clear_color(&mut self, color: Vec3){
        self.clear_color = color;
        unsafe{
            ClearColor(self.clear_color.x, self.clear_color.y, self.clear_color.z, 1.0);
        }
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
    }

    pub fn clear_screen(&self){
        unsafe{
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
    }

    pub fn update_imgui(&mut self) { // use this AFTER window.update()!!!
        let io = self.imgui.io_mut();
        
        let width = self.w as f32;
        let height = self.h as f32;

        io.display_size = [width, height];
        io.update_delta_time(std::time::Duration::from_secs_f32(self.dt));
        io.mouse_pos = [self.mouse_pos.x, self.mouse_pos.y];
        io.mouse_down[0] = self.mouse_buttons[0];
        io.mouse_down[1] = self.mouse_buttons[1];
        io.mouse_down[2] = self.mouse_buttons[2];
    }

    pub fn imgui_frame(&mut self) -> &mut Ui {
        let io = self.imgui.io_mut();

        io.delta_time = self.dt as f32;

        io.display_size = [self.w as f32, self.h as f32];

        self.imgui.frame()
    }

    pub fn is_pressing(&mut self, key: Key) -> bool{
        if !self.keyboard.contains_key(&key){
            self.keyboard.insert(key, Action::Release);
        }
        self.keyboard[&key] != Action::Release
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

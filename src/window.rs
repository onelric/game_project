use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::WindowSettings;

pub struct Window {
    opengl: OpenGL,
    window: GlutinWindow,
}

impl Window {
    pub fn new() -> Self {
        let opengl = OpenGL::V4_5;
        let window: GlutinWindow = WindowSettings::new("Rust piston project", [1280, 720])
            .graphics_api(opengl)
            .resizable(true)
            .build()
            .unwrap();

        Self { opengl, window }
    }

    pub fn get_opengl(&self) -> OpenGL {
        self.opengl
    }

    pub fn get_glutin_window(&mut self) -> &mut GlutinWindow {
        &mut self.window
    }
}

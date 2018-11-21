use crate::error::{Error, Result};
use web_sys::{console, WebGlRenderingContext, WebGlProgram, WebGlShader};
use web_sys::WebGlRenderingContext as gl;

/// Provides all of the common functionality necessary to drive WebGL.
#[derive(Clone)]
pub struct Gl {
    ctx: WebGlRenderingContext,
}

impl Gl {
    /// Creates a new WebGL wrapper from a WebGL context.
    pub fn new(ctx: WebGlRenderingContext) -> Gl {
        Gl { ctx }
    }

    pub fn clear(&self, r: f32, g: f32, b: f32) {
        self.ctx.clear_color(r, g, b, 1.0);
        self.ctx.clear(gl::COLOR_BUFFER_BIT);
    }

    pub fn check_error(&self, context: &'static str) -> Result<()> {
        match self.ctx.get_error() {
            gl::NO_ERROR => Ok(()),
            code => Err(Error::Gl { code, context }),
        }
    }

    pub fn make_error(&self, context: &'static str) -> Error {
        Error::Gl { code: self.ctx.get_error(), context }
    }

    pub fn create_shader(
        &self,
        header: &str,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<Shader> {
        let program = match self.ctx.create_program() {
            Some(handle) => handle,
            None => return Err(self.make_error("failed to allocate shader program")),
        };

        let vertex = match self.ctx.create_shader(gl::VERTEX_SHADER) {
            Some(handle) => handle,
            None => return Err(self.make_error("failed to allocate vertex shader")),
        };

        self.ctx.shader_source(&vertex, &format!("{}{}", header, vertex_src));

        let fragment = match self.ctx.create_shader(gl::FRAGMENT_SHADER) {
            Some(handle) => handle,
            None => return Err(self.make_error("failed to allocate fragment shader")),
        };

        self.ctx.shader_source(&fragment, &format!("{}{}", header, fragment_src));

        self.ctx.compile_shader(&vertex);
        if self.ctx.get_shader_parameter(&vertex, gl::COMPILE_STATUS) != true {
            let s = self.ctx.get_shader_info_log(&vertex).unwrap();
            console::log_1(&s.into());
            // TODO: Log `self.ctx.get_shader_info_log`.
            return Err(self.make_error("failed to compile vertex shader"));
        }

        self.ctx.compile_shader(&fragment);
        if self.ctx.get_shader_parameter(&fragment, gl::COMPILE_STATUS) != true {
            let s = self.ctx.get_shader_info_log(&fragment).unwrap();
            console::log_1(&s.into());
            // TODO: Log `self.ctx.get_shader_info_log`.
            return Err(self.make_error("failed to compile fragment shader"));
        }

        self.ctx.attach_shader(&program, &vertex);
        self.ctx.attach_shader(&program, &fragment);
        self.ctx.bind_attrib_location(&program, 0, "vertex");
        self.ctx.bind_attrib_location(&program, 1, "tcoord");

        self.ctx.link_program(&program);
        if self.ctx.get_program_parameter(&program, gl::LINK_STATUS) != true {
            return Err(self.make_error("failed to link shader program"));
        }

        Ok(Shader { program, vertex, fragment })
    }
}

pub struct Shader {
    pub program: WebGlProgram,
    pub vertex: WebGlShader,
    pub fragment: WebGlShader,
}

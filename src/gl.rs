use crate::error::{Error, Result};
use web_sys::WebGlRenderingContext;

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
        self.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn check_error(&self, context: &'static str) -> Result<()> {
        match self.ctx.get_error() {
            WebGlRenderingContext::NO_ERROR => Ok(()),
            code => Err(Error::Gl { code, context: context }),
        }
    }
}

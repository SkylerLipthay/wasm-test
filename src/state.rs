use crate::error::Result;
use crate::gl::Gl;
use crate::tarp::Tarp;
use web_sys::WebGlRenderingContext;

/// The overall state of the application.
pub struct State {
    /// The time (in seconds) since the last frame.
    pub delta: f64,
    /// The active WebGL context to use for rendering.
    pub tarp: Tarp,
    /// The current width (in pixels) of the canvas.
    pub width: u32,
    /// The current height (in pixels) of the canvas.
    pub height: u32,
}

impl State {
    /// Initializes and returns a new application state.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The WebGL context (owned by an HTML canvas).
    /// * `width` - The width (in pixels) of the canvas.
    /// * `height` - The height (in pixels) of the canvas.
    pub fn new(ctx: WebGlRenderingContext, width: u32, height: u32) -> Result<State> {
        Ok(State {
            delta: 0.0,
            tarp: Tarp::new(Gl::new(ctx))?,
            width,
            height,
        })
    }

    pub fn draw(&mut self) -> Result<()> {
        self.tarp.start_frame();
        self.tarp.end_frame();
        Ok(())
    }
}

use crate::state::State;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

/// Steps and renders a frame of the application.
///
/// # Arguments
///
/// * `state` - The state from the previous frame.
/// * `delta` - The time (in seconds) that has passed since the previous frame.
/// * `width` - The width (in pixels) of the canvas.
/// * `height` - The height (in pixels) of the canvas.
#[wasm_bindgen]
pub fn step(state: &mut StatePtr, delta: f64, width: u32, height: u32) {
    state.delta = delta;
    state.width = width;
    state.height = height;
    state.ctx.clear_color(1.0, 1.0, 1.0, 1.0);
    state.ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
}

/// Initializes and returns a new application state.
///
/// # Arguments
///
/// * `ctx` - The WebGL context (owned by an HTML canvas).
/// * `width` - The width (in pixels) of the canvas.
/// * `height` - The height (in pixels) of the canvas.
#[wasm_bindgen(js_name = newState)]
pub fn new_state(ctx: WebGlRenderingContext, width: u32, height: u32) -> StatePtr {
    StatePtr::make(State::new(ctx, width, height))
}

/// Expose `State` as an opaque pointer to JavaScript.
#[wasm_bindgen]
pub struct StatePtr(*mut State);

impl StatePtr {
    fn make(state: State) -> StatePtr {
        StatePtr(Box::into_raw(Box::new(state)))
    }
}

impl Deref for StatePtr {
    type Target = State;

    fn deref(&self) -> &State {
        unsafe { &*self.0 }
    }
}

impl DerefMut for StatePtr {
    fn deref_mut(&mut self) -> &mut State {
        unsafe { &mut *self.0 }
    }
}

impl Drop for StatePtr {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.0); }
    }
}

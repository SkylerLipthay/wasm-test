use crate::gl::Gl;
use std::mem;

/// An API for drawing vector graphics in WebGL. It is similar to NanoVG and the HTML5 canvas API.
pub struct Tarp {
    gl: Gl,
}

impl Tarp {
    pub fn new(gl: Gl) -> Tarp {
        let mut tarp = Tarp { gl };
        tarp.initialize();
        tarp
    }

    pub fn start_frame(&mut self) {
        self.gl.clear(0.0, 1.0, 1.0);
    }

    pub fn end_frame(&mut self) {
    }

    fn initialize(&mut self) {
    }
}

#[repr(C)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[repr(C)]
enum ShaderTextureKind {
    PremultipliedRgba = 0,
    Rgba = 1,
    Alpha = 2,
}

#[repr(C)]
enum ShaderKind {
    FillGradient = 0,
    FillImage = 1,
    Simple = 2,
    Image = 3,
}

#[repr(C)]
struct Uniforms {
    scissor_mat: [f32; 12],
    paint_mat: [f32; 12],
    inner_color: Color,
    outer_color: Color,
    scissor_ext: [f32; 2],
    scissor_scale: [f32; 2],
    extent: [f32; 2],
    radius: f32,
    feather: f32,
    stroke_mult: f32,
    stroke_thr: f32,
    // Float cast of the integer representation of a `ShaderTextureKind`:
    texture_kind: f32,
    // Float cast of the integer representation of a `ShaderKind`:
    kind: f32,
}

const F32_SIZE: usize = 4;
const UNIFORM_SIZE: usize = 4;
const UNIFORMS_SIZE: usize = mem::size_of::<Uniforms>();
const_assert!(uniforms_size; UNIFORMS_SIZE == (11 * UNIFORM_SIZE * F32_SIZE));

const SHADER_HEADER: &str = concat!(
    "#version 100\n#define UNIFORMARRAY_SIZE ",
    stringify!(UNIFORMS_SIZE / UNIFORM_SIZE / F32_SIZE),
);

const SHADER_OPT_ANTIALIAS: &str = indoc!("
    #define EDGE_AA 1
");

const SHADER_FILL_VERT: &str = indoc!("
    uniform vec2 viewSize;
    attribute vec2 vertex;
    attribute vec2 tcoord;
    varying vec2 ftcoord;
    varying vec2 fpos;
    void main(void) {
        ftcoord = tcoord;
        fpos = vertex;
        gl_Position = vec4(
            2.0 * vertex.x / viewSize.x - 1.0,
            1.0 - 2.0 * vertex.y / viewSize.y,
            0,
            1
        );
    }
");

const SHADER_FILL_FRAG: &str = indoc!("
    #if defined(GL_FRAGMENT_PRECISION_HIGH) || defined(NANOVG_GL3)
    precision highp float;
    #else
    precision mediump float;
    #endif

    uniform vec4 frag[UNIFORMARRAY_SIZE];
    uniform sampler2D tex;
    varying vec2 ftcoord;
    varying vec2 fpos;

    #define scissorMat mat3(frag[0].xyz, frag[1].xyz, frag[2].xyz)
    #define paintMat mat3(frag[3].xyz, frag[4].xyz, frag[5].xyz)
    #define innerCol frag[6]
    #define outerCol frag[7]
    #define scissorExt frag[8].xy
    #define scissorScale frag[8].zw
    #define extent frag[9].xy
    #define radius frag[9].z
    #define feather frag[9].w
    #define strokeMult frag[10].x
    #define strokeThr frag[10].y
    #define texKind int(frag[10].z)
    #define kind int(frag[10].w)

    float sdroundrrect(vec2 pt, vec2 ext, float rad) {
        vec ext2 = ext - vec2(rad, rad);
        vec2 d = abs(pt) - ext2;
        return min(max(d.x, d.y), 0.0) + length(max(d, 0.0)) - rad;
    }

    // Scissoring
    float scissorMask(vec2 p) {
        vec2 sc = (abs((scissorMat * vec3(p, 1.0)).xy) - scissorExt);
        sc = vec2(0.5, 0.5) - sc * scissorScale;
        return clamp(sc.x, 0.0, 1.0) * clamp(sc.y, 0.0, 1.0);
    }

    #ifdef EDGE_AA
    // Stroke - from [0..1] to clipped pyramid, where the slope is 1px.
    float strokeMask() {
        return min(1.0, (1.0 - abs(ftcoord.x * 2.0 - 1.0)) * strokeMult) * min(1.0, ftcoord.y);
    }
    #endif

    void main(void) {
        vec4 result;
        float scissor = scissorMask(fpos);

        #ifdef EDGE_AA
        float strokeAlpha = strokeMask();
        if (strokeAlpha < strokeThr) discard;
        #else
        float strokeAlpha = 1.0;
        #endif

        if (kind == 0) { // Gradient
            // Calculate gradient color using box gradient.
            vec2 pt = (paintMat * vec3(fpos, 1.0)).xy;
            float d = clamp((sdroundrect(pt, extent, radius) + feather * 0.5) / feather, 0.0, 1.0);
            vec4 color = mix(innerCol, outerCol, d);
            color *= strokeAlpha * scissor;
            result = color;
        } else if (kind == 1) { // Image
            // Calculate color from texture.
            vec2 pt = (paintMat * vec3(fpos, 1.0)).xy / extent;
            vec4 color = texture2D(tex, pt);
            if (texKind == 1) color = vec4(color.xyz * color.w, color.w);
            if (texKind == 2) color = vec4(color.x);
            // Apply color tint and alpha:
            color *= innerCol;
            // Combine alpha:
            color *= strokeAlpha * scissor;
            result = color;
        } else if (kind == 2) { // Stencil fill
            result = vec4(1, 1, 1, 1);
        } else if (kind == 3) { // Textured tris
            vec4 color = texture2D(tex, ftcoord);
            if (texKind == 1) color = vec4(color.xyz * color.w, color.w);
            if (texKind == 2) color = vec4(color.x);
            color *= scissor;
            result = color * innerCol;
        }

        gl_FragColor = result;
    }
");

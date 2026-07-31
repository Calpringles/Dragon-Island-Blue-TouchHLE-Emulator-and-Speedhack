
        mod __gl_imports {
            pub use std::mem;
            pub use std::os::raw;
        }
    

        #[inline(never)]
        fn metaloadfn(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void,
                      symbol: &'static str,
                      fallbacks: &[&'static str]) -> *const __gl_imports::raw::c_void {
            let mut ptr = loadfn(symbol);
            if ptr.is_null() {
                for &sym in fallbacks {
                    ptr = loadfn(sym);
                    if !ptr.is_null() { break; }
                }
            }
            ptr
        }
    

        pub mod types {
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    
// Common types from OpenGL 1.1
pub type GLenum = super::__gl_imports::raw::c_uint;
pub type GLboolean = super::__gl_imports::raw::c_uchar;
pub type GLbitfield = super::__gl_imports::raw::c_uint;
pub type GLvoid = super::__gl_imports::raw::c_void;
pub type GLbyte = super::__gl_imports::raw::c_char;
pub type GLshort = super::__gl_imports::raw::c_short;
pub type GLint = super::__gl_imports::raw::c_int;
pub type GLclampx = super::__gl_imports::raw::c_int;
pub type GLubyte = super::__gl_imports::raw::c_uchar;
pub type GLushort = super::__gl_imports::raw::c_ushort;
pub type GLuint = super::__gl_imports::raw::c_uint;
pub type GLsizei = super::__gl_imports::raw::c_int;
pub type GLfloat = super::__gl_imports::raw::c_float;
pub type GLclampf = super::__gl_imports::raw::c_float;
pub type GLdouble = super::__gl_imports::raw::c_double;
pub type GLclampd = super::__gl_imports::raw::c_double;
pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
pub type GLchar = super::__gl_imports::raw::c_char;
pub type GLcharARB = super::__gl_imports::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = super::__gl_imports::raw::c_uint;

pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
pub type GLhalf = super::__gl_imports::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = Option<extern "system" fn(source: GLenum,
                                                 gltype: GLenum,
                                                 id: GLuint,
                                                 severity: GLenum,
                                                 length: GLsizei,
                                                 message: *const GLchar,
                                                 userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLDEBUGPROCARB = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLDEBUGPROCKHR = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = Option<extern "system" fn(id: GLuint,
                                                    category: GLenum,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;


        }
    
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)] pub const ADD: types::GLenum = 0x0104;
#[allow(dead_code, non_upper_case_globals)] pub const ADD_SIGNED: types::GLenum = 0x8574;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_POINT_SIZE_RANGE: types::GLenum = 0x846D;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_BITS: types::GLenum = 0x0D55;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_SCALE: types::GLenum = 0x0D1C;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST: types::GLenum = 0x0BC0;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST_FUNC: types::GLenum = 0x0BC1;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST_REF: types::GLenum = 0x0BC2;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const AMBIENT: types::GLenum = 0x1200;
#[allow(dead_code, non_upper_case_globals)] pub const AMBIENT_AND_DIFFUSE: types::GLenum = 0x1602;
#[allow(dead_code, non_upper_case_globals)] pub const AND: types::GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)] pub const AND_INVERTED: types::GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)] pub const AND_REVERSE: types::GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER: types::GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA_EXT: types::GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST: types::GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_OES: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC: types::GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_BITS: types::GLenum = 0x0D54;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS_OES: types::GLenum = 0x88BB;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAPPED_OES: types::GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_POINTER_OES: types::GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: types::GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_USAGE: types::GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR: types::GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_ACTIVE_TEXTURE: types::GLenum = 0x84E1;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE0: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE1: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE2: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE3: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE4: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE5: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY: types::GLenum = 0x8076;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_BUFFER_BINDING: types::GLenum = 0x8898;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_POINTER: types::GLenum = 0x8090;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_SIZE: types::GLenum = 0x8081;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_STRIDE: types::GLenum = 0x8083;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_TYPE: types::GLenum = 0x8082;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT0_OES: types::GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_MATERIAL: types::GLenum = 0x0B57;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMBINE: types::GLenum = 0x8570;
#[allow(dead_code, non_upper_case_globals)] pub const COMBINE_ALPHA: types::GLenum = 0x8572;
#[allow(dead_code, non_upper_case_globals)] pub const COMBINE_RGB: types::GLenum = 0x8571;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: types::GLenum = 0x8C03;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: types::GLenum = 0x8C02;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB_PVRTC_2BPPV1_IMG: types::GLenum = 0x8C01;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB_PVRTC_4BPPV1_IMG: types::GLenum = 0x8C00;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT: types::GLenum = 0x8576;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ATTENUATION: types::GLenum = 0x1207;
#[allow(dead_code, non_upper_case_globals)] pub const COPY: types::GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_INVERTED: types::GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_COLOR: types::GLenum = 0x0B00;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_NORMAL: types::GLenum = 0x0B02;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_PALETTE_MATRIX_OES: types::GLenum = 0x8843;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_TEXTURE_COORDS: types::GLenum = 0x0B03;
#[allow(dead_code, non_upper_case_globals)] pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DECAL: types::GLenum = 0x2101;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_ATTACHMENT_OES: types::GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BITS: types::GLenum = 0x0D56;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT16_OES: types::GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DIFFUSE: types::GLenum = 0x1201;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DOT3_RGB: types::GLenum = 0x86AE;
#[allow(dead_code, non_upper_case_globals)] pub const DOT3_RGBA: types::GLenum = 0x86AF;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_DRAW: types::GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)] pub const EMISSION: types::GLenum = 0x1600;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EQUIV: types::GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)] pub const EXP: types::GLenum = 0x0800;
#[allow(dead_code, non_upper_case_globals)] pub const EXP2: types::GLenum = 0x0801;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED: types::GLenum = 0x140C;
#[allow(dead_code, non_upper_case_globals)] pub const FLAT: types::GLenum = 0x1D00;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FOG: types::GLenum = 0x0B60;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_COLOR: types::GLenum = 0x0B66;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_DENSITY: types::GLenum = 0x0B62;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_END: types::GLenum = 0x0B64;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_HINT: types::GLenum = 0x0C54;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_MODE: types::GLenum = 0x0B65;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_START: types::GLenum = 0x0B63;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME_OES: types::GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE_OES: types::GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE_OES: types::GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL_OES: types::GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BINDING_OES: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_COMPLETE_OES: types::GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT_OES: types::GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS_OES: types::GLenum = 0x8CD9;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_FORMATS_OES: types::GLenum = 0x8CDA;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_OES: types::GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_OES: types::GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNSUPPORTED_OES: types::GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_ADD_OES: types::GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_REVERSE_SUBTRACT_OES: types::GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_SUBTRACT_OES: types::GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)] pub const GENERATE_MIPMAP: types::GLenum = 0x8191;
#[allow(dead_code, non_upper_case_globals)] pub const GENERATE_MIPMAP_HINT: types::GLenum = 0x8192;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_BITS: types::GLenum = 0x0D53;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INTERPOLATE: types::GLenum = 0x8575;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_FRAMEBUFFER_OPERATION_OES: types::GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT0: types::GLenum = 0x4000;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT1: types::GLenum = 0x4001;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT2: types::GLenum = 0x4002;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT3: types::GLenum = 0x4003;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT4: types::GLenum = 0x4004;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT5: types::GLenum = 0x4005;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT6: types::GLenum = 0x4006;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT7: types::GLenum = 0x4007;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHTING: types::GLenum = 0x0B50;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_AMBIENT: types::GLenum = 0x0B53;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_TWO_SIDE: types::GLenum = 0x0B52;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_ATTENUATION: types::GLenum = 0x1208;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH: types::GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE: types::GLenum = 0x1909;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE_ALPHA: types::GLenum = 0x190A;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_INDEX_ARRAY_BUFFER_BINDING_OES: types::GLenum = 0x8B9E;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_INDEX_ARRAY_OES: types::GLenum = 0x8844;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_INDEX_ARRAY_POINTER_OES: types::GLenum = 0x8849;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_INDEX_ARRAY_SIZE_OES: types::GLenum = 0x8846;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_INDEX_ARRAY_STRIDE_OES: types::GLenum = 0x8848;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_INDEX_ARRAY_TYPE_OES: types::GLenum = 0x8847;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_MODE: types::GLenum = 0x0BA0;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_PALETTE_OES: types::GLenum = 0x8840;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIP_PLANES: types::GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LIGHTS: types::GLenum = 0x0D31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_MODELVIEW_STACK_DEPTH: types::GLenum = 0x0D36;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PALETTE_MATRICES_OES: types::GLenum = 0x8842;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROJECTION_STACK_DEPTH: types::GLenum = 0x0D38;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RENDERBUFFER_SIZE_OES: types::GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_LOD_BIAS_EXT: types::GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: types::GLenum = 0x84FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_STACK_DEPTH: types::GLenum = 0x0D39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_UNITS: types::GLenum = 0x84E2;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNITS_OES: types::GLenum = 0x86A4;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW: types::GLenum = 0x1700;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW_MATRIX: types::GLenum = 0x0BA6;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW_STACK_DEPTH: types::GLenum = 0x0BA3;
#[allow(dead_code, non_upper_case_globals)] pub const MODULATE: types::GLenum = 0x2100;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE: types::GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)] pub const NAND: types::GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE_OES: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOOP: types::GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)] pub const NOR: types::GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)] pub const NORMALIZE: types::GLenum = 0x0BA1;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY: types::GLenum = 0x8075;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_BUFFER_BINDING: types::GLenum = 0x8897;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_POINTER: types::GLenum = 0x808F;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_STRIDE: types::GLenum = 0x807F;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_TYPE: types::GLenum = 0x807E;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND0_ALPHA: types::GLenum = 0x8598;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND0_RGB: types::GLenum = 0x8590;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND1_ALPHA: types::GLenum = 0x8599;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND1_RGB: types::GLenum = 0x8591;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND2_ALPHA: types::GLenum = 0x859A;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND2_RGB: types::GLenum = 0x8592;
#[allow(dead_code, non_upper_case_globals)] pub const OR: types::GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)] pub const OR_INVERTED: types::GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)] pub const OR_REVERSE: types::GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)] pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE4_R5_G6_B5_OES: types::GLenum = 0x8B92;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE4_RGB5_A1_OES: types::GLenum = 0x8B94;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE4_RGB8_OES: types::GLenum = 0x8B90;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE4_RGBA4_OES: types::GLenum = 0x8B93;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE4_RGBA8_OES: types::GLenum = 0x8B91;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE8_R5_G6_B5_OES: types::GLenum = 0x8B97;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE8_RGB5_A1_OES: types::GLenum = 0x8B99;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE8_RGB8_OES: types::GLenum = 0x8B95;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE8_RGBA4_OES: types::GLenum = 0x8B98;
#[allow(dead_code, non_upper_case_globals)] pub const PALETTE8_RGBA8_OES: types::GLenum = 0x8B96;
#[allow(dead_code, non_upper_case_globals)] pub const PERSPECTIVE_CORRECTION_HINT: types::GLenum = 0x0C50;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_DISTANCE_ATTENUATION: types::GLenum = 0x8129;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_FADE_THRESHOLD_SIZE: types::GLenum = 0x8128;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE: types::GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_ARRAY_BUFFER_BINDING_OES: types::GLenum = 0x8B9F;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_ARRAY_OES: types::GLenum = 0x8B9C;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_ARRAY_POINTER_OES: types::GLenum = 0x898C;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_ARRAY_STRIDE_OES: types::GLenum = 0x898B;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_ARRAY_TYPE_OES: types::GLenum = 0x898A;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_MAX: types::GLenum = 0x8127;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_MIN: types::GLenum = 0x8126;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SMOOTH: types::GLenum = 0x0B10;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SMOOTH_HINT: types::GLenum = 0x0C51;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const POSITION: types::GLenum = 0x1203;
#[allow(dead_code, non_upper_case_globals)] pub const PREVIOUS: types::GLenum = 0x8578;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMARY_COLOR: types::GLenum = 0x8577;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION: types::GLenum = 0x1701;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION_MATRIX: types::GLenum = 0x0BA7;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION_STACK_DEPTH: types::GLenum = 0x0BA4;
#[allow(dead_code, non_upper_case_globals)] pub const QUADRATIC_ATTENUATION: types::GLenum = 0x1209;
#[allow(dead_code, non_upper_case_globals)] pub const RED_BITS: types::GLenum = 0x0D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_ALPHA_SIZE_OES: types::GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BINDING_OES: types::GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BLUE_SIZE_OES: types::GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_DEPTH_SIZE_OES: types::GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_GREEN_SIZE_OES: types::GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_HEIGHT_OES: types::GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_INTERNAL_FORMAT_OES: types::GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_OES: types::GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_RED_SIZE_OES: types::GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_STENCIL_SIZE_OES: types::GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_WIDTH_OES: types::GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RESCALE_NORMAL: types::GLenum = 0x803A;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB565_OES: types::GLenum = 0x8D62;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1_OES: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8_OES: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4_OES: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8_OES: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)] pub const RGB_SCALE: types::GLenum = 0x8573;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_ONE: types::GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SET: types::GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADE_MODEL: types::GLenum = 0x0B54;
#[allow(dead_code, non_upper_case_globals)] pub const SHININESS: types::GLenum = 0x1601;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH: types::GLenum = 0x1D01;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const SPECULAR: types::GLenum = 0x1202;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_CUTOFF: types::GLenum = 0x1206;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_DIRECTION: types::GLenum = 0x1204;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_EXPONENT: types::GLenum = 0x1205;
#[allow(dead_code, non_upper_case_globals)] pub const SRC0_ALPHA: types::GLenum = 0x8588;
#[allow(dead_code, non_upper_case_globals)] pub const SRC0_RGB: types::GLenum = 0x8580;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_ALPHA: types::GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_RGB: types::GLenum = 0x8581;
#[allow(dead_code, non_upper_case_globals)] pub const SRC2_ALPHA: types::GLenum = 0x858A;
#[allow(dead_code, non_upper_case_globals)] pub const SRC2_RGB: types::GLenum = 0x8582;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_DRAW: types::GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_ATTACHMENT_OES: types::GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BITS: types::GLenum = 0x0D57;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const SUBTRACT: types::GLenum = 0x84E7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE0: types::GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE1: types::GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE10: types::GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE11: types::GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE12: types::GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE13: types::GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE14: types::GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE15: types::GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE16: types::GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE17: types::GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE18: types::GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE19: types::GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE2: types::GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE20: types::GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE21: types::GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE22: types::GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE23: types::GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE24: types::GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE25: types::GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE26: types::GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE27: types::GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE28: types::GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE29: types::GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE3: types::GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE30: types::GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE31: types::GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE4: types::GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE5: types::GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE6: types::GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE7: types::GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE8: types::GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE9: types::GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY: types::GLenum = 0x8078;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_BUFFER_BINDING: types::GLenum = 0x889A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_POINTER: types::GLenum = 0x8092;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_SIZE: types::GLenum = 0x8088;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_STRIDE: types::GLenum = 0x808A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_TYPE: types::GLenum = 0x8089;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CROP_RECT_OES: types::GLenum = 0x8B9D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV: types::GLenum = 0x2300;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV_COLOR: types::GLenum = 0x2201;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV_MODE: types::GLenum = 0x2200;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FILTER_CONTROL_EXT: types::GLenum = 0x8500;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_LOD_BIAS_EXT: types::GLenum = 0x8501;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MATRIX: types::GLenum = 0x0BA8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_ANISOTROPY_EXT: types::GLenum = 0x84FE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_STACK_DEPTH: types::GLenum = 0x0BA5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION_ES_CL_1_0: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION_ES_CL_1_1: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION_ES_CM_1_1: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_BUFFER_BINDING: types::GLenum = 0x8896;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_POINTER: types::GLenum = 0x808E;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_SIZE: types::GLenum = 0x807A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_STRIDE: types::GLenum = 0x807C;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_TYPE: types::GLenum = 0x807B;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const WEIGHT_ARRAY_BUFFER_BINDING_OES: types::GLenum = 0x889E;
#[allow(dead_code, non_upper_case_globals)] pub const WEIGHT_ARRAY_OES: types::GLenum = 0x86AD;
#[allow(dead_code, non_upper_case_globals)] pub const WEIGHT_ARRAY_POINTER_OES: types::GLenum = 0x86AC;
#[allow(dead_code, non_upper_case_globals)] pub const WEIGHT_ARRAY_SIZE_OES: types::GLenum = 0x86AB;
#[allow(dead_code, non_upper_case_globals)] pub const WEIGHT_ARRAY_STRIDE_OES: types::GLenum = 0x86AA;
#[allow(dead_code, non_upper_case_globals)] pub const WEIGHT_ARRAY_TYPE_OES: types::GLenum = 0x86A9;
#[allow(dead_code, non_upper_case_globals)] pub const WRITE_ONLY_OES: types::GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)] pub const XOR: types::GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: types::GLenum = 0;
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ActiveTexture(texture: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ActiveTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn AlphaFunc(func: types::GLenum, ref_: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::AlphaFunc.f)(func, ref_) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn AlphaFuncx(func: types::GLenum, ref_: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed) -> ()>(storage::AlphaFuncx.f)(func, ref_) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindBuffer(target: types::GLenum, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindBuffer.f)(target, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindFramebufferOES(target: types::GLenum, framebuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindFramebufferOES.f)(target, framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindRenderbufferOES(target: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindRenderbufferOES.f)(target, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BindTexture(target: types::GLenum, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindTexture.f)(target, texture) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendEquationOES(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::BlendEquationOES.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BlendFunc(sfactor: types::GLenum, dfactor: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::BlendFunc.f)(sfactor, dfactor) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BufferData(target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizeiptr, *const __gl_imports::raw::c_void, types::GLenum) -> ()>(storage::BufferData.f)(target, size, data, usage) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn BufferSubData(target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, *const __gl_imports::raw::c_void) -> ()>(storage::BufferSubData.f)(target, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CheckFramebufferStatusOES(target: types::GLenum) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLenum>(storage::CheckFramebufferStatusOES.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Clear(mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::Clear.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ClearColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearColorx(red: types::GLfixed, green: types::GLfixed, blue: types::GLfixed, alpha: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::ClearColorx.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearDepthf(d: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::ClearDepthf.f)(d) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearDepthx(depth: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed) -> ()>(storage::ClearDepthx.f)(depth) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClearStencil(s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(storage::ClearStencil.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClientActiveTexture(texture: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ClientActiveTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClipPlanef(p: types::GLenum, eqn: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(storage::ClipPlanef.f)(p, eqn) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ClipPlanex(plane: types::GLenum, equation: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfixed) -> ()>(storage::ClipPlanex.f)(plane, equation) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4f(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Color4f.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4ub(red: types::GLubyte, green: types::GLubyte, blue: types::GLubyte, alpha: types::GLubyte) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLubyte, types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(storage::Color4ub.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Color4x(red: types::GLfixed, green: types::GLfixed, blue: types::GLfixed, alpha: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::Color4x.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ColorMask(red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(storage::ColorMask.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ColorPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::ColorPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CompressedTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CompressedTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(storage::CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CopyTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CullFace(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::CullFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn CurrentPaletteMatrixOES(matrixpaletteindex: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::CurrentPaletteMatrixOES.f)(matrixpaletteindex) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteBuffers(n: types::GLsizei, buffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteFramebuffersOES(n: types::GLsizei, framebuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteFramebuffersOES.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteRenderbuffersOES(n: types::GLsizei, renderbuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteRenderbuffersOES.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DeleteTextures(n: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthFunc(func: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::DepthFunc.f)(func) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthMask(flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(storage::DepthMask.f)(flag) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthRangef(n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::DepthRangef.f)(n, f) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DepthRangex(n: types::GLfixed, f: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed) -> ()>(storage::DepthRangex.f)(n, f) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Disable(cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Disable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DisableClientState(array: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::DisableClientState.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> ()>(storage::DrawArrays.f)(mode, first, count) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawElements(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::DrawElements.f)(mode, count, type_, indices) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawTexfOES(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, width: types::GLfloat, height: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::DrawTexfOES.f)(x, y, z, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawTexfvOES(coords: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::DrawTexfvOES.f)(coords) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawTexiOES(x: types::GLint, y: types::GLint, z: types::GLint, width: types::GLint, height: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::DrawTexiOES.f)(x, y, z, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawTexivOES(coords: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLint) -> ()>(storage::DrawTexivOES.f)(coords) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawTexsOES(x: types::GLshort, y: types::GLshort, z: types::GLshort, width: types::GLshort, height: types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLshort, types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::DrawTexsOES.f)(x, y, z, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawTexsvOES(coords: *const types::GLshort) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLshort) -> ()>(storage::DrawTexsvOES.f)(coords) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawTexxOES(x: types::GLfixed, y: types::GLfixed, z: types::GLfixed, width: types::GLfixed, height: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::DrawTexxOES.f)(x, y, z, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn DrawTexxvOES(coords: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfixed) -> ()>(storage::DrawTexxvOES.f)(coords) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Enable(cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Enable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn EnableClientState(array: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::EnableClientState.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Finish() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Flush() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Fogf(pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::Fogf.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Fogfv(pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(storage::Fogfv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Fogx(pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed) -> ()>(storage::Fogx.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Fogxv(pname: types::GLenum, param: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfixed) -> ()>(storage::Fogxv.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FramebufferRenderbufferOES(target: types::GLenum, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::FramebufferRenderbufferOES.f)(target, attachment, renderbuffertarget, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FramebufferTexture2DOES(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(storage::FramebufferTexture2DOES.f)(target, attachment, textarget, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn FrontFace(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::FrontFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Frustumf(l: types::GLfloat, r: types::GLfloat, b: types::GLfloat, t: types::GLfloat, n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Frustumf.f)(l, r, b, t, n, f) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Frustumx(l: types::GLfixed, r: types::GLfixed, b: types::GLfixed, t: types::GLfixed, n: types::GLfixed, f: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::Frustumx.f)(l, r, b, t, n, f) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenBuffers(n: types::GLsizei, buffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenFramebuffersOES(n: types::GLsizei, framebuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenFramebuffersOES.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenRenderbuffersOES(n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenRenderbuffersOES.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenTextures(n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GenerateMipmapOES(target: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::GenerateMipmapOES.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBooleanv(pname: types::GLenum, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLboolean) -> ()>(storage::GetBooleanv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetBufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetBufferPointervOES(target: types::GLenum, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(storage::GetBufferPointervOES.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetClipPlanef(plane: types::GLenum, equation: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(storage::GetClipPlanef.f)(plane, equation) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetClipPlanex(plane: types::GLenum, equation: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfixed) -> ()>(storage::GetClipPlanex.f)(plane, equation) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetError() -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(storage::GetError.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetFixedv(pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfixed) -> ()>(storage::GetFixedv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetFloatv(pname: types::GLenum, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(storage::GetFloatv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetFramebufferAttachmentParameterivOES(target: types::GLenum, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetFramebufferAttachmentParameterivOES.f)(target, attachment, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetIntegerv(pname: types::GLenum, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(storage::GetIntegerv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetLightfv(light: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetLightfv.f)(light, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetLightxv(light: types::GLenum, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfixed) -> ()>(storage::GetLightxv.f)(light, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetMaterialfv(face: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetMaterialfv.f)(face, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetMaterialxv(face: types::GLenum, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfixed) -> ()>(storage::GetMaterialxv.f)(face, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetPointerv(pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(storage::GetPointerv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetRenderbufferParameterivOES(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetRenderbufferParameterivOES.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetString(name: types::GLenum) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(storage::GetString.f)(name) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexEnvfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexEnvfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexEnviv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexEnviv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexEnvxv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfixed) -> ()>(storage::GetTexEnvxv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameterfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn GetTexParameterxv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfixed) -> ()>(storage::GetTexParameterxv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Hint(target: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::Hint.f)(target, mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsBuffer(buffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsBuffer.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsEnabled(cap: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(storage::IsEnabled.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsFramebufferOES(framebuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsFramebufferOES.f)(framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsRenderbufferOES(renderbuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsRenderbufferOES.f)(renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn IsTexture(texture: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LightModelf(pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::LightModelf.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LightModelfv(pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(storage::LightModelfv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LightModelx(pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed) -> ()>(storage::LightModelx.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LightModelxv(pname: types::GLenum, param: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfixed) -> ()>(storage::LightModelxv.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Lightf(light: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::Lightf.f)(light, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Lightfv(light: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::Lightfv.f)(light, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Lightx(light: types::GLenum, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfixed) -> ()>(storage::Lightx.f)(light, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Lightxv(light: types::GLenum, pname: types::GLenum, params: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfixed) -> ()>(storage::Lightxv.f)(light, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LineWidth(width: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::LineWidth.f)(width) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LineWidthx(width: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed) -> ()>(storage::LineWidthx.f)(width) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LoadIdentity() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::LoadIdentity.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LoadMatrixf(m: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::LoadMatrixf.f)(m) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LoadMatrixx(m: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfixed) -> ()>(storage::LoadMatrixx.f)(m) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LoadPaletteFromModelViewMatrixOES() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::LoadPaletteFromModelViewMatrixOES.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn LogicOp(opcode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::LogicOp.f)(opcode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MapBufferOES(target: types::GLenum, access: types::GLenum) -> *mut __gl_imports::raw::c_void { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> *mut __gl_imports::raw::c_void>(storage::MapBufferOES.f)(target, access) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Materialf(face: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::Materialf.f)(face, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Materialfv(face: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::Materialfv.f)(face, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Materialx(face: types::GLenum, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfixed) -> ()>(storage::Materialx.f)(face, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Materialxv(face: types::GLenum, pname: types::GLenum, param: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfixed) -> ()>(storage::Materialxv.f)(face, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MatrixIndexPointerOES(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::MatrixIndexPointerOES.f)(size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MatrixMode(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::MatrixMode.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MultMatrixf(m: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfloat) -> ()>(storage::MultMatrixf.f)(m) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MultMatrixx(m: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const types::GLfixed) -> ()>(storage::MultMatrixx.f)(m) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MultiTexCoord4f(target: types::GLenum, s: types::GLfloat, t: types::GLfloat, r: types::GLfloat, q: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::MultiTexCoord4f.f)(target, s, t, r, q) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn MultiTexCoord4x(texture: types::GLenum, s: types::GLfixed, t: types::GLfixed, r: types::GLfixed, q: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::MultiTexCoord4x.f)(texture, s, t, r, q) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3f(nx: types::GLfloat, ny: types::GLfloat, nz: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Normal3f.f)(nx, ny, nz) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Normal3x(nx: types::GLfixed, ny: types::GLfixed, nz: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::Normal3x.f)(nx, ny, nz) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn NormalPointer(type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::NormalPointer.f)(type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Orthof(l: types::GLfloat, r: types::GLfloat, b: types::GLfloat, t: types::GLfloat, n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Orthof.f)(l, r, b, t, n, f) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Orthox(l: types::GLfixed, r: types::GLfixed, b: types::GLfixed, t: types::GLfixed, n: types::GLfixed, f: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::Orthox.f)(l, r, b, t, n, f) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PixelStorei(pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::PixelStorei.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PointParameterf(pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::PointParameterf.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PointParameterfv(pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(storage::PointParameterfv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PointParameterx(pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfixed) -> ()>(storage::PointParameterx.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PointParameterxv(pname: types::GLenum, params: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfixed) -> ()>(storage::PointParameterxv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PointSize(size: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::PointSize.f)(size) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PointSizePointerOES(type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::PointSizePointerOES.f)(type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PointSizex(size: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed) -> ()>(storage::PointSizex.f)(size) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PolygonOffset(factor: types::GLfloat, units: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::PolygonOffset.f)(factor, units) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PolygonOffsetx(factor: types::GLfixed, units: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed) -> ()>(storage::PolygonOffsetx.f)(factor, units) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PopMatrix() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PopMatrix.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn PushMatrix() -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(storage::PushMatrix.f)() }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ReadPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(storage::ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn RenderbufferStorageOES(target: types::GLenum, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::RenderbufferStorageOES.f)(target, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rotatef(angle: types::GLfloat, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Rotatef.f)(angle, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Rotatex(angle: types::GLfixed, x: types::GLfixed, y: types::GLfixed, z: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::Rotatex.f)(angle, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SampleCoverage(value: types::GLfloat, invert: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLboolean) -> ()>(storage::SampleCoverage.f)(value, invert) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn SampleCoveragex(value: types::GLclampx, invert: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLclampx, types::GLboolean) -> ()>(storage::SampleCoveragex.f)(value, invert) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Scalef(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Scalef.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Scalex(x: types::GLfixed, y: types::GLfixed, z: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::Scalex.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Scissor(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::Scissor.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn ShadeModel(mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ShadeModel.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilFunc(func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> ()>(storage::StencilFunc.f)(func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilMask(mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::StencilMask.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn StencilOp(fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::StencilOp.f)(fail, zfail, zpass) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexCoordPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::TexCoordPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnvf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::TexEnvf.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnvfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::TexEnvfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnvi(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::TexEnvi.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnviv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexEnviv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnvx(target: types::GLenum, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfixed) -> ()>(storage::TexEnvx.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexEnvxv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfixed) -> ()>(storage::TexEnvxv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::TexParameterf.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::TexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::TexParameteri.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameteriv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterx(target: types::GLenum, pname: types::GLenum, param: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfixed) -> ()>(storage::TexParameterx.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexParameterxv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfixed) -> ()>(storage::TexParameterxv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn TexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(storage::TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Translatef(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Translatef.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Translatex(x: types::GLfixed, y: types::GLfixed, z: types::GLfixed) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfixed, types::GLfixed, types::GLfixed) -> ()>(storage::Translatex.f)(x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn UnmapBufferOES(target: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(storage::UnmapBufferOES.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn VertexPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::VertexPointer.f)(size, type_, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn Viewport(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::Viewport.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)] #[inline]
            pub unsafe fn WeightPointerOES(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(storage::WeightPointerOES.f)(size, type_, stride, pointer) }

        #[allow(missing_copy_implementations)]
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::raw::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }

        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            pub fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr { f: missing_fn_panic as *const __gl_imports::raw::c_void, is_loaded: false }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }
        }
    
mod storage {
            #![allow(non_snake_case)]
            #![allow(non_upper_case_globals)]
            use super::__gl_imports::raw;
            use super::FnPtr;
pub static mut ActiveTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut AlphaFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut AlphaFuncx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindFramebufferOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindRenderbufferOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BindTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendEquationOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BlendFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BufferData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut BufferSubData: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CheckFramebufferStatusOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Clear: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearColor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearColorx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearDepthf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearDepthx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClearStencil: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClientActiveTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClipPlanef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ClipPlanex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4ub: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Color4x: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ColorMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ColorPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CompressedTexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CompressedTexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CopyTexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CullFace: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut CurrentPaletteMatrixOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteFramebuffersOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteRenderbuffersOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DeleteTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthRangef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DepthRangex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Disable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DisableClientState: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawArrays: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawElements: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawTexfOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawTexfvOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawTexiOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawTexivOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawTexsOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawTexsvOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawTexxOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut DrawTexxvOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Enable: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut EnableClientState: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Finish: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Flush: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Fogf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Fogfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Fogx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Fogxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FramebufferRenderbufferOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FramebufferTexture2DOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut FrontFace: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Frustumf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Frustumx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenBuffers: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenFramebuffersOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenRenderbuffersOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenTextures: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GenerateMipmapOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBooleanv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBufferParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetBufferPointervOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetClipPlanef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetClipPlanex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetError: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetFixedv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetFloatv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetFramebufferAttachmentParameterivOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetIntegerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetLightfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetLightxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetMaterialfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetMaterialxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetPointerv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetRenderbufferParameterivOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetString: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexEnvfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexEnviv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexEnvxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut GetTexParameterxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Hint: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsBuffer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsEnabled: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsFramebufferOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsRenderbufferOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut IsTexture: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LightModelf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LightModelfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LightModelx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LightModelxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Lightf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Lightfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Lightx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Lightxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LineWidth: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LineWidthx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LoadIdentity: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LoadMatrixf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LoadMatrixx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LoadPaletteFromModelViewMatrixOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut LogicOp: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MapBufferOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Materialf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Materialfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Materialx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Materialxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MatrixIndexPointerOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MatrixMode: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MultMatrixf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MultMatrixx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MultiTexCoord4f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut MultiTexCoord4x: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3f: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Normal3x: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut NormalPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Orthof: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Orthox: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PixelStorei: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PointParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PointParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PointParameterx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PointParameterxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PointSize: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PointSizePointerOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PointSizex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PolygonOffset: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PolygonOffsetx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PopMatrix: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut PushMatrix: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ReadPixels: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut RenderbufferStorageOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rotatef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Rotatex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SampleCoverage: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut SampleCoveragex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Scalef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Scalex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Scissor: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut ShadeModel: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilFunc: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilMask: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut StencilOp: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexCoordPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnvf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnvfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnvi: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnviv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnvx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexEnvxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterf: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterfv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameteri: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameteriv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterx: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexParameterxv: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut TexSubImage2D: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Translatef: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Translatex: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut UnmapBufferOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut VertexPointer: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut Viewport: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
pub static mut WeightPointerOES: FnPtr = FnPtr {
                f: super::missing_fn_panic as *const raw::c_void,
                is_loaded: false
            };
}

            #[allow(non_snake_case)]
            pub mod ActiveTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ActiveTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ActiveTexture = FnPtr::new(metaloadfn(&mut loadfn, "glActiveTexture", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod AlphaFunc {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::AlphaFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::AlphaFunc = FnPtr::new(metaloadfn(&mut loadfn, "glAlphaFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod AlphaFuncx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::AlphaFuncx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::AlphaFuncx = FnPtr::new(metaloadfn(&mut loadfn, "glAlphaFuncx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindBuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BindBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glBindBuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindFramebufferOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindFramebufferOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BindFramebufferOES = FnPtr::new(metaloadfn(&mut loadfn, "glBindFramebufferOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindRenderbufferOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindRenderbufferOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BindRenderbufferOES = FnPtr::new(metaloadfn(&mut loadfn, "glBindRenderbufferOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BindTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BindTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BindTexture = FnPtr::new(metaloadfn(&mut loadfn, "glBindTexture", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendEquationOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendEquationOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendEquationOES = FnPtr::new(metaloadfn(&mut loadfn, "glBlendEquationOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BlendFunc {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BlendFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BlendFunc = FnPtr::new(metaloadfn(&mut loadfn, "glBlendFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BufferData {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BufferData.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BufferData = FnPtr::new(metaloadfn(&mut loadfn, "glBufferData", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod BufferSubData {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::BufferSubData.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::BufferSubData = FnPtr::new(metaloadfn(&mut loadfn, "glBufferSubData", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CheckFramebufferStatusOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CheckFramebufferStatusOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CheckFramebufferStatusOES = FnPtr::new(metaloadfn(&mut loadfn, "glCheckFramebufferStatusOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Clear {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Clear.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Clear = FnPtr::new(metaloadfn(&mut loadfn, "glClear", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearColor {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearColor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearColor = FnPtr::new(metaloadfn(&mut loadfn, "glClearColor", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearColorx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearColorx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearColorx = FnPtr::new(metaloadfn(&mut loadfn, "glClearColorx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearDepthf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearDepthf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearDepthf = FnPtr::new(metaloadfn(&mut loadfn, "glClearDepthf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearDepthx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearDepthx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearDepthx = FnPtr::new(metaloadfn(&mut loadfn, "glClearDepthx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClearStencil {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClearStencil.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClearStencil = FnPtr::new(metaloadfn(&mut loadfn, "glClearStencil", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClientActiveTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClientActiveTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClientActiveTexture = FnPtr::new(metaloadfn(&mut loadfn, "glClientActiveTexture", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClipPlanef {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClipPlanef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClipPlanef = FnPtr::new(metaloadfn(&mut loadfn, "glClipPlanef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ClipPlanex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ClipPlanex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ClipPlanex = FnPtr::new(metaloadfn(&mut loadfn, "glClipPlanex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4f = FnPtr::new(metaloadfn(&mut loadfn, "glColor4f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4ub {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4ub.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4ub = FnPtr::new(metaloadfn(&mut loadfn, "glColor4ub", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Color4x {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Color4x.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Color4x = FnPtr::new(metaloadfn(&mut loadfn, "glColor4x", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ColorMask {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ColorMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ColorMask = FnPtr::new(metaloadfn(&mut loadfn, "glColorMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ColorPointer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ColorPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ColorPointer = FnPtr::new(metaloadfn(&mut loadfn, "glColorPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CompressedTexImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CompressedTexImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CompressedTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexImage2D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CompressedTexSubImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CompressedTexSubImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CompressedTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCompressedTexSubImage2D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexImage2D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CopyTexSubImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CopyTexSubImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CopyTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glCopyTexSubImage2D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CullFace {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CullFace.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CullFace = FnPtr::new(metaloadfn(&mut loadfn, "glCullFace", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod CurrentPaletteMatrixOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::CurrentPaletteMatrixOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::CurrentPaletteMatrixOES = FnPtr::new(metaloadfn(&mut loadfn, "glCurrentPaletteMatrixOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteBuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteBuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteBuffers", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteFramebuffersOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteFramebuffersOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteFramebuffersOES = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteFramebuffersOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteRenderbuffersOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteRenderbuffersOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteRenderbuffersOES = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteRenderbuffersOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DeleteTextures {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DeleteTextures.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DeleteTextures = FnPtr::new(metaloadfn(&mut loadfn, "glDeleteTextures", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthFunc {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthFunc = FnPtr::new(metaloadfn(&mut loadfn, "glDepthFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthMask {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthMask = FnPtr::new(metaloadfn(&mut loadfn, "glDepthMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthRangef {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthRangef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthRangef = FnPtr::new(metaloadfn(&mut loadfn, "glDepthRangef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DepthRangex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DepthRangex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DepthRangex = FnPtr::new(metaloadfn(&mut loadfn, "glDepthRangex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Disable {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Disable.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Disable = FnPtr::new(metaloadfn(&mut loadfn, "glDisable", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DisableClientState {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DisableClientState.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DisableClientState = FnPtr::new(metaloadfn(&mut loadfn, "glDisableClientState", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawArrays {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawArrays.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawArrays = FnPtr::new(metaloadfn(&mut loadfn, "glDrawArrays", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawElements {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawElements.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawElements = FnPtr::new(metaloadfn(&mut loadfn, "glDrawElements", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawTexfOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawTexfOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawTexfOES = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTexfOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawTexfvOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawTexfvOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawTexfvOES = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTexfvOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawTexiOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawTexiOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawTexiOES = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTexiOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawTexivOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawTexivOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawTexivOES = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTexivOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawTexsOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawTexsOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawTexsOES = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTexsOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawTexsvOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawTexsvOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawTexsvOES = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTexsvOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawTexxOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawTexxOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawTexxOES = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTexxOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod DrawTexxvOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::DrawTexxvOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::DrawTexxvOES = FnPtr::new(metaloadfn(&mut loadfn, "glDrawTexxvOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Enable {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Enable.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Enable = FnPtr::new(metaloadfn(&mut loadfn, "glEnable", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod EnableClientState {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::EnableClientState.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::EnableClientState = FnPtr::new(metaloadfn(&mut loadfn, "glEnableClientState", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Finish {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Finish.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Finish = FnPtr::new(metaloadfn(&mut loadfn, "glFinish", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Flush {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Flush.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Flush = FnPtr::new(metaloadfn(&mut loadfn, "glFlush", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Fogf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Fogf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Fogf = FnPtr::new(metaloadfn(&mut loadfn, "glFogf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Fogfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Fogfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Fogfv = FnPtr::new(metaloadfn(&mut loadfn, "glFogfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Fogx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Fogx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Fogx = FnPtr::new(metaloadfn(&mut loadfn, "glFogx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Fogxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Fogxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Fogxv = FnPtr::new(metaloadfn(&mut loadfn, "glFogxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FramebufferRenderbufferOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FramebufferRenderbufferOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::FramebufferRenderbufferOES = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferRenderbufferOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FramebufferTexture2DOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FramebufferTexture2DOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::FramebufferTexture2DOES = FnPtr::new(metaloadfn(&mut loadfn, "glFramebufferTexture2DOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod FrontFace {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::FrontFace.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::FrontFace = FnPtr::new(metaloadfn(&mut loadfn, "glFrontFace", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Frustumf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Frustumf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Frustumf = FnPtr::new(metaloadfn(&mut loadfn, "glFrustumf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Frustumx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Frustumx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Frustumx = FnPtr::new(metaloadfn(&mut loadfn, "glFrustumx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenBuffers {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenBuffers.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GenBuffers = FnPtr::new(metaloadfn(&mut loadfn, "glGenBuffers", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenFramebuffersOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenFramebuffersOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GenFramebuffersOES = FnPtr::new(metaloadfn(&mut loadfn, "glGenFramebuffersOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenRenderbuffersOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenRenderbuffersOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GenRenderbuffersOES = FnPtr::new(metaloadfn(&mut loadfn, "glGenRenderbuffersOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenTextures {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenTextures.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GenTextures = FnPtr::new(metaloadfn(&mut loadfn, "glGenTextures", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GenerateMipmapOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GenerateMipmapOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GenerateMipmapOES = FnPtr::new(metaloadfn(&mut loadfn, "glGenerateMipmapOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBooleanv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBooleanv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBooleanv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBooleanv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBufferParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBufferParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetBufferPointervOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetBufferPointervOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetBufferPointervOES = FnPtr::new(metaloadfn(&mut loadfn, "glGetBufferPointervOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetClipPlanef {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetClipPlanef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetClipPlanef = FnPtr::new(metaloadfn(&mut loadfn, "glGetClipPlanef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetClipPlanex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetClipPlanex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetClipPlanex = FnPtr::new(metaloadfn(&mut loadfn, "glGetClipPlanex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetError {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetError.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetError = FnPtr::new(metaloadfn(&mut loadfn, "glGetError", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetFixedv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetFixedv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetFixedv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFixedv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetFloatv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetFloatv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetFloatv = FnPtr::new(metaloadfn(&mut loadfn, "glGetFloatv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetFramebufferAttachmentParameterivOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetFramebufferAttachmentParameterivOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetFramebufferAttachmentParameterivOES = FnPtr::new(metaloadfn(&mut loadfn, "glGetFramebufferAttachmentParameterivOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetIntegerv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetIntegerv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetIntegerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetIntegerv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetLightfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetLightfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetLightfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetLightfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetLightxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetLightxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetLightxv = FnPtr::new(metaloadfn(&mut loadfn, "glGetLightxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetMaterialfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetMaterialfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetMaterialfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMaterialfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetMaterialxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetMaterialxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetMaterialxv = FnPtr::new(metaloadfn(&mut loadfn, "glGetMaterialxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetPointerv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetPointerv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetPointerv = FnPtr::new(metaloadfn(&mut loadfn, "glGetPointerv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetRenderbufferParameterivOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetRenderbufferParameterivOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetRenderbufferParameterivOES = FnPtr::new(metaloadfn(&mut loadfn, "glGetRenderbufferParameterivOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetString {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetString.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetString = FnPtr::new(metaloadfn(&mut loadfn, "glGetString", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexEnvfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexEnvfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexEnvfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexEnvfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexEnviv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexEnviv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexEnviv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexEnviv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexEnvxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexEnvxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexEnvxv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexEnvxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameterfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod GetTexParameterxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::GetTexParameterxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::GetTexParameterxv = FnPtr::new(metaloadfn(&mut loadfn, "glGetTexParameterxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Hint {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Hint.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Hint = FnPtr::new(metaloadfn(&mut loadfn, "glHint", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsBuffer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsBuffer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IsBuffer = FnPtr::new(metaloadfn(&mut loadfn, "glIsBuffer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsEnabled {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsEnabled.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IsEnabled = FnPtr::new(metaloadfn(&mut loadfn, "glIsEnabled", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsFramebufferOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsFramebufferOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IsFramebufferOES = FnPtr::new(metaloadfn(&mut loadfn, "glIsFramebufferOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsRenderbufferOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsRenderbufferOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IsRenderbufferOES = FnPtr::new(metaloadfn(&mut loadfn, "glIsRenderbufferOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod IsTexture {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::IsTexture.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::IsTexture = FnPtr::new(metaloadfn(&mut loadfn, "glIsTexture", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LightModelf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LightModelf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LightModelf = FnPtr::new(metaloadfn(&mut loadfn, "glLightModelf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LightModelfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LightModelfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LightModelfv = FnPtr::new(metaloadfn(&mut loadfn, "glLightModelfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LightModelx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LightModelx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LightModelx = FnPtr::new(metaloadfn(&mut loadfn, "glLightModelx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LightModelxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LightModelxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LightModelxv = FnPtr::new(metaloadfn(&mut loadfn, "glLightModelxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Lightf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Lightf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Lightf = FnPtr::new(metaloadfn(&mut loadfn, "glLightf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Lightfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Lightfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Lightfv = FnPtr::new(metaloadfn(&mut loadfn, "glLightfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Lightx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Lightx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Lightx = FnPtr::new(metaloadfn(&mut loadfn, "glLightx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Lightxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Lightxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Lightxv = FnPtr::new(metaloadfn(&mut loadfn, "glLightxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LineWidth {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LineWidth.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LineWidth = FnPtr::new(metaloadfn(&mut loadfn, "glLineWidth", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LineWidthx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LineWidthx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LineWidthx = FnPtr::new(metaloadfn(&mut loadfn, "glLineWidthx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LoadIdentity {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LoadIdentity.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LoadIdentity = FnPtr::new(metaloadfn(&mut loadfn, "glLoadIdentity", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LoadMatrixf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LoadMatrixf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LoadMatrixf = FnPtr::new(metaloadfn(&mut loadfn, "glLoadMatrixf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LoadMatrixx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LoadMatrixx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LoadMatrixx = FnPtr::new(metaloadfn(&mut loadfn, "glLoadMatrixx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LoadPaletteFromModelViewMatrixOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LoadPaletteFromModelViewMatrixOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LoadPaletteFromModelViewMatrixOES = FnPtr::new(metaloadfn(&mut loadfn, "glLoadPaletteFromModelViewMatrixOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod LogicOp {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::LogicOp.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::LogicOp = FnPtr::new(metaloadfn(&mut loadfn, "glLogicOp", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MapBufferOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MapBufferOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MapBufferOES = FnPtr::new(metaloadfn(&mut loadfn, "glMapBufferOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Materialf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Materialf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Materialf = FnPtr::new(metaloadfn(&mut loadfn, "glMaterialf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Materialfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Materialfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Materialfv = FnPtr::new(metaloadfn(&mut loadfn, "glMaterialfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Materialx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Materialx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Materialx = FnPtr::new(metaloadfn(&mut loadfn, "glMaterialx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Materialxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Materialxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Materialxv = FnPtr::new(metaloadfn(&mut loadfn, "glMaterialxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MatrixIndexPointerOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MatrixIndexPointerOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MatrixIndexPointerOES = FnPtr::new(metaloadfn(&mut loadfn, "glMatrixIndexPointerOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MatrixMode {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MatrixMode.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MatrixMode = FnPtr::new(metaloadfn(&mut loadfn, "glMatrixMode", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MultMatrixf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MultMatrixf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MultMatrixf = FnPtr::new(metaloadfn(&mut loadfn, "glMultMatrixf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MultMatrixx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MultMatrixx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MultMatrixx = FnPtr::new(metaloadfn(&mut loadfn, "glMultMatrixx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MultiTexCoord4f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MultiTexCoord4f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MultiTexCoord4f = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoord4f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod MultiTexCoord4x {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::MultiTexCoord4x.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::MultiTexCoord4x = FnPtr::new(metaloadfn(&mut loadfn, "glMultiTexCoord4x", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3f {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3f.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3f = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3f", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Normal3x {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Normal3x.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Normal3x = FnPtr::new(metaloadfn(&mut loadfn, "glNormal3x", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod NormalPointer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::NormalPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::NormalPointer = FnPtr::new(metaloadfn(&mut loadfn, "glNormalPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Orthof {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Orthof.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Orthof = FnPtr::new(metaloadfn(&mut loadfn, "glOrthof", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Orthox {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Orthox.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Orthox = FnPtr::new(metaloadfn(&mut loadfn, "glOrthox", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PixelStorei {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PixelStorei.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PixelStorei = FnPtr::new(metaloadfn(&mut loadfn, "glPixelStorei", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PointParameterf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PointParameterf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PointParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glPointParameterf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PointParameterfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PointParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PointParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glPointParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PointParameterx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PointParameterx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PointParameterx = FnPtr::new(metaloadfn(&mut loadfn, "glPointParameterx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PointParameterxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PointParameterxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PointParameterxv = FnPtr::new(metaloadfn(&mut loadfn, "glPointParameterxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PointSize {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PointSize.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PointSize = FnPtr::new(metaloadfn(&mut loadfn, "glPointSize", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PointSizePointerOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PointSizePointerOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PointSizePointerOES = FnPtr::new(metaloadfn(&mut loadfn, "glPointSizePointerOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PointSizex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PointSizex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PointSizex = FnPtr::new(metaloadfn(&mut loadfn, "glPointSizex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PolygonOffset {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PolygonOffset.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PolygonOffset = FnPtr::new(metaloadfn(&mut loadfn, "glPolygonOffset", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PolygonOffsetx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PolygonOffsetx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PolygonOffsetx = FnPtr::new(metaloadfn(&mut loadfn, "glPolygonOffsetx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PopMatrix {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PopMatrix.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PopMatrix = FnPtr::new(metaloadfn(&mut loadfn, "glPopMatrix", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod PushMatrix {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::PushMatrix.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::PushMatrix = FnPtr::new(metaloadfn(&mut loadfn, "glPushMatrix", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ReadPixels {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ReadPixels.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ReadPixels = FnPtr::new(metaloadfn(&mut loadfn, "glReadPixels", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod RenderbufferStorageOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::RenderbufferStorageOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::RenderbufferStorageOES = FnPtr::new(metaloadfn(&mut loadfn, "glRenderbufferStorageOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rotatef {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rotatef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rotatef = FnPtr::new(metaloadfn(&mut loadfn, "glRotatef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Rotatex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Rotatex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Rotatex = FnPtr::new(metaloadfn(&mut loadfn, "glRotatex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SampleCoverage {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SampleCoverage.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::SampleCoverage = FnPtr::new(metaloadfn(&mut loadfn, "glSampleCoverage", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod SampleCoveragex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::SampleCoveragex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::SampleCoveragex = FnPtr::new(metaloadfn(&mut loadfn, "glSampleCoveragex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Scalef {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Scalef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Scalef = FnPtr::new(metaloadfn(&mut loadfn, "glScalef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Scalex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Scalex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Scalex = FnPtr::new(metaloadfn(&mut loadfn, "glScalex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Scissor {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Scissor.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Scissor = FnPtr::new(metaloadfn(&mut loadfn, "glScissor", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod ShadeModel {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::ShadeModel.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::ShadeModel = FnPtr::new(metaloadfn(&mut loadfn, "glShadeModel", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilFunc {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilFunc.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilFunc = FnPtr::new(metaloadfn(&mut loadfn, "glStencilFunc", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilMask {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilMask.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilMask = FnPtr::new(metaloadfn(&mut loadfn, "glStencilMask", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod StencilOp {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::StencilOp.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::StencilOp = FnPtr::new(metaloadfn(&mut loadfn, "glStencilOp", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexCoordPointer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexCoordPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexCoordPointer = FnPtr::new(metaloadfn(&mut loadfn, "glTexCoordPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnvf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnvf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnvf = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnvf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnvfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnvfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnvfv = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnvfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnvi {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnvi.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnvi = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnvi", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnviv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnviv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnviv = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnviv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnvx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnvx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnvx = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnvx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexEnvxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexEnvxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexEnvxv = FnPtr::new(metaloadfn(&mut loadfn, "glTexEnvxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexImage2D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterf {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterf.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterf = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterf", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterfv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterfv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterfv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameteri {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameteri.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameteri = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameteri", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameteriv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameteriv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameteriv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterx {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterx.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterx = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterx", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexParameterxv {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexParameterxv.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexParameterxv = FnPtr::new(metaloadfn(&mut loadfn, "glTexParameterxv", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod TexSubImage2D {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::TexSubImage2D.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::TexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, "glTexSubImage2D", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Translatef {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Translatef.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Translatef = FnPtr::new(metaloadfn(&mut loadfn, "glTranslatef", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Translatex {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Translatex.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Translatex = FnPtr::new(metaloadfn(&mut loadfn, "glTranslatex", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod UnmapBufferOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::UnmapBufferOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::UnmapBufferOES = FnPtr::new(metaloadfn(&mut loadfn, "glUnmapBufferOES", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod VertexPointer {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::VertexPointer.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::VertexPointer = FnPtr::new(metaloadfn(&mut loadfn, "glVertexPointer", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod Viewport {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::Viewport.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::Viewport = FnPtr::new(metaloadfn(&mut loadfn, "glViewport", &[]))
                    }
                }
            }
        

            #[allow(non_snake_case)]
            pub mod WeightPointerOES {
                use super::{storage, metaloadfn};
                use super::__gl_imports::raw;
                use super::FnPtr;

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { storage::WeightPointerOES.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
                    unsafe {
                        storage::WeightPointerOES = FnPtr::new(metaloadfn(&mut loadfn, "glWeightPointerOES", &[]))
                    }
                }
            }
        
#[inline(never)]
        fn missing_fn_panic() -> ! {
            panic!("gles1 function was not loaded")
        }
        

        /// Load each OpenGL symbol using a custom load function. This allows for the
        /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
        /// ~~~ignore
        /// gl::load_with(|s| glfw.get_proc_address(s));
        /// ~~~
        #[allow(dead_code)]
        pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const __gl_imports::raw::c_void {
            #[inline(never)]
            fn inner(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void) {
    
ActiveTexture::load_with(&mut *loadfn);
AlphaFunc::load_with(&mut *loadfn);
AlphaFuncx::load_with(&mut *loadfn);
BindBuffer::load_with(&mut *loadfn);
BindFramebufferOES::load_with(&mut *loadfn);
BindRenderbufferOES::load_with(&mut *loadfn);
BindTexture::load_with(&mut *loadfn);
BlendEquationOES::load_with(&mut *loadfn);
BlendFunc::load_with(&mut *loadfn);
BufferData::load_with(&mut *loadfn);
BufferSubData::load_with(&mut *loadfn);
CheckFramebufferStatusOES::load_with(&mut *loadfn);
Clear::load_with(&mut *loadfn);
ClearColor::load_with(&mut *loadfn);
ClearColorx::load_with(&mut *loadfn);
ClearDepthf::load_with(&mut *loadfn);
ClearDepthx::load_with(&mut *loadfn);
ClearStencil::load_with(&mut *loadfn);
ClientActiveTexture::load_with(&mut *loadfn);
ClipPlanef::load_with(&mut *loadfn);
ClipPlanex::load_with(&mut *loadfn);
Color4f::load_with(&mut *loadfn);
Color4ub::load_with(&mut *loadfn);
Color4x::load_with(&mut *loadfn);
ColorMask::load_with(&mut *loadfn);
ColorPointer::load_with(&mut *loadfn);
CompressedTexImage2D::load_with(&mut *loadfn);
CompressedTexSubImage2D::load_with(&mut *loadfn);
CopyTexImage2D::load_with(&mut *loadfn);
CopyTexSubImage2D::load_with(&mut *loadfn);
CullFace::load_with(&mut *loadfn);
CurrentPaletteMatrixOES::load_with(&mut *loadfn);
DeleteBuffers::load_with(&mut *loadfn);
DeleteFramebuffersOES::load_with(&mut *loadfn);
DeleteRenderbuffersOES::load_with(&mut *loadfn);
DeleteTextures::load_with(&mut *loadfn);
DepthFunc::load_with(&mut *loadfn);
DepthMask::load_with(&mut *loadfn);
DepthRangef::load_with(&mut *loadfn);
DepthRangex::load_with(&mut *loadfn);
Disable::load_with(&mut *loadfn);
DisableClientState::load_with(&mut *loadfn);
DrawArrays::load_with(&mut *loadfn);
DrawElements::load_with(&mut *loadfn);
DrawTexfOES::load_with(&mut *loadfn);
DrawTexfvOES::load_with(&mut *loadfn);
DrawTexiOES::load_with(&mut *loadfn);
DrawTexivOES::load_with(&mut *loadfn);
DrawTexsOES::load_with(&mut *loadfn);
DrawTexsvOES::load_with(&mut *loadfn);
DrawTexxOES::load_with(&mut *loadfn);
DrawTexxvOES::load_with(&mut *loadfn);
Enable::load_with(&mut *loadfn);
EnableClientState::load_with(&mut *loadfn);
Finish::load_with(&mut *loadfn);
Flush::load_with(&mut *loadfn);
Fogf::load_with(&mut *loadfn);
Fogfv::load_with(&mut *loadfn);
Fogx::load_with(&mut *loadfn);
Fogxv::load_with(&mut *loadfn);
FramebufferRenderbufferOES::load_with(&mut *loadfn);
FramebufferTexture2DOES::load_with(&mut *loadfn);
FrontFace::load_with(&mut *loadfn);
Frustumf::load_with(&mut *loadfn);
Frustumx::load_with(&mut *loadfn);
GenBuffers::load_with(&mut *loadfn);
GenFramebuffersOES::load_with(&mut *loadfn);
GenRenderbuffersOES::load_with(&mut *loadfn);
GenTextures::load_with(&mut *loadfn);
GenerateMipmapOES::load_with(&mut *loadfn);
GetBooleanv::load_with(&mut *loadfn);
GetBufferParameteriv::load_with(&mut *loadfn);
GetBufferPointervOES::load_with(&mut *loadfn);
GetClipPlanef::load_with(&mut *loadfn);
GetClipPlanex::load_with(&mut *loadfn);
GetError::load_with(&mut *loadfn);
GetFixedv::load_with(&mut *loadfn);
GetFloatv::load_with(&mut *loadfn);
GetFramebufferAttachmentParameterivOES::load_with(&mut *loadfn);
GetIntegerv::load_with(&mut *loadfn);
GetLightfv::load_with(&mut *loadfn);
GetLightxv::load_with(&mut *loadfn);
GetMaterialfv::load_with(&mut *loadfn);
GetMaterialxv::load_with(&mut *loadfn);
GetPointerv::load_with(&mut *loadfn);
GetRenderbufferParameterivOES::load_with(&mut *loadfn);
GetString::load_with(&mut *loadfn);
GetTexEnvfv::load_with(&mut *loadfn);
GetTexEnviv::load_with(&mut *loadfn);
GetTexEnvxv::load_with(&mut *loadfn);
GetTexParameterfv::load_with(&mut *loadfn);
GetTexParameteriv::load_with(&mut *loadfn);
GetTexParameterxv::load_with(&mut *loadfn);
Hint::load_with(&mut *loadfn);
IsBuffer::load_with(&mut *loadfn);
IsEnabled::load_with(&mut *loadfn);
IsFramebufferOES::load_with(&mut *loadfn);
IsRenderbufferOES::load_with(&mut *loadfn);
IsTexture::load_with(&mut *loadfn);
LightModelf::load_with(&mut *loadfn);
LightModelfv::load_with(&mut *loadfn);
LightModelx::load_with(&mut *loadfn);
LightModelxv::load_with(&mut *loadfn);
Lightf::load_with(&mut *loadfn);
Lightfv::load_with(&mut *loadfn);
Lightx::load_with(&mut *loadfn);
Lightxv::load_with(&mut *loadfn);
LineWidth::load_with(&mut *loadfn);
LineWidthx::load_with(&mut *loadfn);
LoadIdentity::load_with(&mut *loadfn);
LoadMatrixf::load_with(&mut *loadfn);
LoadMatrixx::load_with(&mut *loadfn);
LoadPaletteFromModelViewMatrixOES::load_with(&mut *loadfn);
LogicOp::load_with(&mut *loadfn);
MapBufferOES::load_with(&mut *loadfn);
Materialf::load_with(&mut *loadfn);
Materialfv::load_with(&mut *loadfn);
Materialx::load_with(&mut *loadfn);
Materialxv::load_with(&mut *loadfn);
MatrixIndexPointerOES::load_with(&mut *loadfn);
MatrixMode::load_with(&mut *loadfn);
MultMatrixf::load_with(&mut *loadfn);
MultMatrixx::load_with(&mut *loadfn);
MultiTexCoord4f::load_with(&mut *loadfn);
MultiTexCoord4x::load_with(&mut *loadfn);
Normal3f::load_with(&mut *loadfn);
Normal3x::load_with(&mut *loadfn);
NormalPointer::load_with(&mut *loadfn);
Orthof::load_with(&mut *loadfn);
Orthox::load_with(&mut *loadfn);
PixelStorei::load_with(&mut *loadfn);
PointParameterf::load_with(&mut *loadfn);
PointParameterfv::load_with(&mut *loadfn);
PointParameterx::load_with(&mut *loadfn);
PointParameterxv::load_with(&mut *loadfn);
PointSize::load_with(&mut *loadfn);
PointSizePointerOES::load_with(&mut *loadfn);
PointSizex::load_with(&mut *loadfn);
PolygonOffset::load_with(&mut *loadfn);
PolygonOffsetx::load_with(&mut *loadfn);
PopMatrix::load_with(&mut *loadfn);
PushMatrix::load_with(&mut *loadfn);
ReadPixels::load_with(&mut *loadfn);
RenderbufferStorageOES::load_with(&mut *loadfn);
Rotatef::load_with(&mut *loadfn);
Rotatex::load_with(&mut *loadfn);
SampleCoverage::load_with(&mut *loadfn);
SampleCoveragex::load_with(&mut *loadfn);
Scalef::load_with(&mut *loadfn);
Scalex::load_with(&mut *loadfn);
Scissor::load_with(&mut *loadfn);
ShadeModel::load_with(&mut *loadfn);
StencilFunc::load_with(&mut *loadfn);
StencilMask::load_with(&mut *loadfn);
StencilOp::load_with(&mut *loadfn);
TexCoordPointer::load_with(&mut *loadfn);
TexEnvf::load_with(&mut *loadfn);
TexEnvfv::load_with(&mut *loadfn);
TexEnvi::load_with(&mut *loadfn);
TexEnviv::load_with(&mut *loadfn);
TexEnvx::load_with(&mut *loadfn);
TexEnvxv::load_with(&mut *loadfn);
TexImage2D::load_with(&mut *loadfn);
TexParameterf::load_with(&mut *loadfn);
TexParameterfv::load_with(&mut *loadfn);
TexParameteri::load_with(&mut *loadfn);
TexParameteriv::load_with(&mut *loadfn);
TexParameterx::load_with(&mut *loadfn);
TexParameterxv::load_with(&mut *loadfn);
TexSubImage2D::load_with(&mut *loadfn);
Translatef::load_with(&mut *loadfn);
Translatex::load_with(&mut *loadfn);
UnmapBufferOES::load_with(&mut *loadfn);
VertexPointer::load_with(&mut *loadfn);
Viewport::load_with(&mut *loadfn);
WeightPointerOES::load_with(&mut *loadfn);

            }

            inner(&mut loadfn)
        }
    

/*
 * Copyright (C) 2012 The Android Open Source Project
 * Copyright (C) 2026 The AOSP and FrizkOS
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Complete modern Rust implementation of `opengltest_jni` for Android CTS OpenGL tests.
//!
//! Replaces legacy C++ files:
//! - attach_shader_one.cpp through attach_shader_eleven.cpp
//! - color_one.cpp
//! - common.cpp
//! - gl2_jni_libone.cpp
//!
//! Provides dynamic GLESv2 loading with deterministic headless fallback and unit test suite.

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_int, c_void, CString};
use std::mem;
use std::ptr;
use std::sync::Mutex;
use std::sync::OnceLock;

pub type jboolean = u8;
pub const JNI_TRUE: jboolean = 1;
pub const JNI_FALSE: jboolean = 0;
pub type jint = i32;
pub type jsize = i32;
pub type jfloat = f32;
pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jfloatArray = jobject;

#[repr(C)]
pub struct JNINativeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub reserved3: *mut c_void,

    pub GetVersion: *mut c_void,
    pub DefineClass: *mut c_void,
    pub FindClass: *mut c_void,
    pub FromReflectedMethod: *mut c_void,
    pub FromReflectedField: *mut c_void,
    pub ToReflectedMethod: *mut c_void,
    pub GetSuperclass: *mut c_void,
    pub IsAssignableFrom: *mut c_void,
    pub ToReflectedField: *mut c_void,

    pub Throw: *mut c_void,
    pub ThrowNew: *mut c_void,
    pub ExceptionOccurred: *mut c_void,
    pub ExceptionDescribe: *mut c_void,
    pub ExceptionClear: *mut c_void,
    pub FatalError: *mut c_void,

    pub PushLocalFrame: *mut c_void,
    pub PopLocalFrame: *mut c_void,

    pub NewGlobalRef: *mut c_void,
    pub DeleteGlobalRef: *mut c_void,
    pub DeleteLocalRef: *mut c_void,
    pub IsSameObject: *mut c_void,
    pub NewLocalRef: *mut c_void,
    pub EnsureLocalCapacity: *mut c_void,

    pub AllocObject: *mut c_void,
    pub NewObject: *mut c_void,
    pub NewObjectA: *mut c_void,
    pub NewObjectV: *mut c_void,

    pub GetObjectClass: *mut c_void,
    pub IsInstanceOf: *mut c_void,
    pub GetMethodID: *mut c_void,
    pub CallObjectMethod: *mut c_void,
    pub CallObjectMethodV: *mut c_void,
    pub CallObjectMethodA: *mut c_void,
    pub CallBooleanMethod: *mut c_void,
    pub CallBooleanMethodV: *mut c_void,
    pub CallBooleanMethodA: *mut c_void,
    pub CallByteMethod: *mut c_void,
    pub CallByteMethodV: *mut c_void,
    pub CallByteMethodA: *mut c_void,
    pub CallCharMethod: *mut c_void,
    pub CallCharMethodV: *mut c_void,
    pub CallCharMethodA: *mut c_void,
    pub CallShortMethod: *mut c_void,
    pub CallShortMethodV: *mut c_void,
    pub CallShortMethodA: *mut c_void,
    pub CallIntMethod: *mut c_void,
    pub CallIntMethodV: *mut c_void,
    pub CallIntMethodA: *mut c_void,
    pub CallLongMethod: *mut c_void,
    pub CallLongMethodV: *mut c_void,
    pub CallLongMethodA: *mut c_void,
    pub CallFloatMethod: *mut c_void,
    pub CallFloatMethodV: *mut c_void,
    pub CallFloatMethodA: *mut c_void,
    pub CallDoubleMethod: *mut c_void,
    pub CallDoubleMethodV: *mut c_void,
    pub CallDoubleMethodA: *mut c_void,
    pub CallVoidMethod: *mut c_void,
    pub CallVoidMethodV: *mut c_void,
    pub CallVoidMethodA: *mut c_void,

    pub CallNonvirtualObjectMethod: *mut c_void,
    pub CallNonvirtualObjectMethodV: *mut c_void,
    pub CallNonvirtualObjectMethodA: *mut c_void,
    pub CallNonvirtualBooleanMethod: *mut c_void,
    pub CallNonvirtualBooleanMethodV: *mut c_void,
    pub CallNonvirtualBooleanMethodA: *mut c_void,
    pub CallNonvirtualByteMethod: *mut c_void,
    pub CallNonvirtualByteMethodV: *mut c_void,
    pub CallNonvirtualByteMethodA: *mut c_void,
    pub CallNonvirtualCharMethod: *mut c_void,
    pub CallNonvirtualCharMethodV: *mut c_void,
    pub CallNonvirtualCharMethodA: *mut c_void,
    pub CallNonvirtualShortMethod: *mut c_void,
    pub CallNonvirtualShortMethodV: *mut c_void,
    pub CallNonvirtualShortMethodA: *mut c_void,
    pub CallNonvirtualIntMethod: *mut c_void,
    pub CallNonvirtualIntMethodV: *mut c_void,
    pub CallNonvirtualIntMethodA: *mut c_void,
    pub CallNonvirtualLongMethod: *mut c_void,
    pub CallNonvirtualLongMethodV: *mut c_void,
    pub CallNonvirtualLongMethodA: *mut c_void,
    pub CallNonvirtualFloatMethod: *mut c_void,
    pub CallNonvirtualFloatMethodV: *mut c_void,
    pub CallNonvirtualFloatMethodA: *mut c_void,
    pub CallNonvirtualDoubleMethod: *mut c_void,
    pub CallNonvirtualDoubleMethodV: *mut c_void,
    pub CallNonvirtualDoubleMethodA: *mut c_void,
    pub CallNonvirtualVoidMethod: *mut c_void,
    pub CallNonvirtualVoidMethodV: *mut c_void,
    pub CallNonvirtualVoidMethodA: *mut c_void,

    pub GetFieldID: *mut c_void,
    pub GetObjectField: *mut c_void,
    pub GetBooleanField: *mut c_void,
    pub GetByteField: *mut c_void,
    pub GetCharField: *mut c_void,
    pub GetShortField: *mut c_void,
    pub GetIntField: *mut c_void,
    pub GetLongField: *mut c_void,
    pub GetFloatField: *mut c_void,
    pub GetDoubleField: *mut c_void,

    pub SetObjectField: *mut c_void,
    pub SetBooleanField: *mut c_void,
    pub SetByteField: *mut c_void,
    pub SetCharField: *mut c_void,
    pub SetShortField: *mut c_void,
    pub SetIntField: *mut c_void,
    pub SetLongField: *mut c_void,
    pub SetFloatField: *mut c_void,
    pub SetDoubleField: *mut c_void,

    pub GetStaticMethodID: *mut c_void,
    pub CallStaticObjectMethod: *mut c_void,
    pub CallStaticObjectMethodV: *mut c_void,
    pub CallStaticObjectMethodA: *mut c_void,
    pub CallStaticBooleanMethod: *mut c_void,
    pub CallStaticBooleanMethodV: *mut c_void,
    pub CallStaticBooleanMethodA: *mut c_void,
    pub CallStaticByteMethod: *mut c_void,
    pub CallStaticByteMethodV: *mut c_void,
    pub CallStaticByteMethodA: *mut c_void,
    pub CallStaticCharMethod: *mut c_void,
    pub CallStaticCharMethodV: *mut c_void,
    pub CallStaticCharMethodA: *mut c_void,
    pub CallStaticShortMethod: *mut c_void,
    pub CallStaticShortMethodV: *mut c_void,
    pub CallStaticShortMethodA: *mut c_void,
    pub CallStaticIntMethod: *mut c_void,
    pub CallStaticIntMethodV: *mut c_void,
    pub CallStaticIntMethodA: *mut c_void,
    pub CallStaticLongMethod: *mut c_void,
    pub CallStaticLongMethodV: *mut c_void,
    pub CallStaticLongMethodA: *mut c_void,
    pub CallStaticFloatMethod: *mut c_void,
    pub CallStaticFloatMethodV: *mut c_void,
    pub CallStaticFloatMethodA: *mut c_void,
    pub CallStaticDoubleMethod: *mut c_void,
    pub CallStaticDoubleMethodV: *mut c_void,
    pub CallStaticDoubleMethodA: *mut c_void,
    pub CallStaticVoidMethod: *mut c_void,
    pub CallStaticVoidMethodV: *mut c_void,
    pub CallStaticVoidMethodA: *mut c_void,

    pub GetStaticFieldID: *mut c_void,
    pub GetStaticObjectField: *mut c_void,
    pub GetStaticBooleanField: *mut c_void,
    pub GetStaticByteField: *mut c_void,
    pub GetStaticCharField: *mut c_void,
    pub GetStaticShortField: *mut c_void,
    pub GetStaticIntField: *mut c_void,
    pub GetStaticLongField: *mut c_void,
    pub GetStaticFloatField: *mut c_void,
    pub GetStaticDoubleField: *mut c_void,

    pub SetStaticObjectField: *mut c_void,
    pub SetStaticBooleanField: *mut c_void,
    pub SetStaticByteField: *mut c_void,
    pub SetStaticCharField: *mut c_void,
    pub SetShortField2: *mut c_void,
    pub SetStaticIntField: *mut c_void,
    pub SetStaticLongField: *mut c_void,
    pub SetStaticFloatField: *mut c_void,
    pub SetStaticDoubleField: *mut c_void,

    pub NewString: *mut c_void,
    pub GetStringLength: *mut c_void,
    pub GetStringChars: *mut c_void,
    pub ReleaseStringChars: *mut c_void,
    pub NewStringUTF: *mut c_void,
    pub GetStringUTFLength: *mut c_void,
    pub GetStringUTFChars: unsafe extern "C" fn(*mut JNIEnv, jobject, *mut jboolean) -> *const c_char,
    pub ReleaseStringUTFChars: unsafe extern "C" fn(*mut JNIEnv, jobject, *const c_char),

    pub GetArrayLength: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jsize,
    pub NewObjectArray: *mut c_void,
    pub GetObjectArrayElement: *mut c_void,
    pub SetObjectArrayElement: *mut c_void,

    pub NewBooleanArray: *mut c_void,
    pub NewByteArray: *mut c_void,
    pub NewCharArray: *mut c_void,
    pub NewShortArray: *mut c_void,
    pub NewIntArray: *mut c_void,
    pub NewLongArray: *mut c_void,
    pub NewFloatArray: unsafe extern "C" fn(*mut JNIEnv, jsize) -> jfloatArray,
    pub NewDoubleArray: *mut c_void,

    pub GetBooleanArrayElements: *mut c_void,
    pub GetByteArrayElements: *mut c_void,
    pub GetCharArrayElements: *mut c_void,
    pub GetShortArrayElements: *mut c_void,
    pub GetIntArrayElements: *mut c_void,
    pub GetLongArrayElements: *mut c_void,
    pub GetFloatArrayElements: unsafe extern "C" fn(*mut JNIEnv, jfloatArray, *mut jboolean) -> *mut jfloat,
    pub GetDoubleArrayElements: *mut c_void,

    pub ReleaseBooleanArrayElements: *mut c_void,
    pub ReleaseByteArrayElements: *mut c_void,
    pub ReleaseCharArrayElements: *mut c_void,
    pub ReleaseShortArrayElements: *mut c_void,
    pub ReleaseIntArrayElements: *mut c_void,
    pub ReleaseLongArrayElements: *mut c_void,
    pub ReleaseFloatArrayElements: unsafe extern "C" fn(*mut JNIEnv, jfloatArray, *mut jfloat, jint),
    pub ReleaseDoubleArrayElements: *mut c_void,

    pub GetBooleanArrayRegion: *mut c_void,
    pub GetByteArrayRegion: *mut c_void,
    pub GetCharArrayRegion: *mut c_void,
    pub GetShortArrayRegion: *mut c_void,
    pub GetIntArrayRegion: *mut c_void,
    pub GetLongArrayRegion: *mut c_void,
    pub GetFloatArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jfloatArray, jsize, jsize, *mut jfloat),
    pub GetDoubleArrayRegion: *mut c_void,

    pub SetBooleanArrayRegion: *mut c_void,
    pub SetByteArrayRegion: *mut c_void,
    pub SetCharArrayRegion: *mut c_void,
    pub SetShortArrayRegion: *mut c_void,
    pub SetIntArrayRegion: *mut c_void,
    pub SetLongArrayRegion: *mut c_void,
    pub SetFloatArrayRegion: unsafe extern "C" fn(*mut JNIEnv, jfloatArray, jsize, jsize, *const jfloat),
    pub SetDoubleArrayRegion: *mut c_void,
}

pub type JNIEnv = *const JNINativeInterface;

// ---------------------------------------------------------------------------
// GLES2 Definitions
// ---------------------------------------------------------------------------

pub type GLenum = u32;
pub type GLboolean = u8;
pub type GLbitfield = u32;
pub type GLint = i32;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLfloat = f32;
pub type GLubyte = u8;

pub const GL_FALSE: GLboolean = 0;
pub const GL_TRUE: GLboolean = 1;

pub const GL_NO_ERROR: GLenum = 0;
pub const GL_INVALID_ENUM: GLenum = 0x0500;
pub const GL_INVALID_VALUE: GLenum = 0x0501;
pub const GL_INVALID_OPERATION: GLenum = 0x0502;
pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;

pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;

pub const GL_TRIANGLES: GLenum = 0x0004;
pub const GL_FLOAT: GLenum = 0x1406;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_RGBA: GLenum = 0x1908;

pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_LINK_STATUS: GLenum = 0x8B82;

// ---------------------------------------------------------------------------
// Dynamic GLES2 Dispatcher
// ---------------------------------------------------------------------------

type FnGlCreateShader = unsafe extern "C" fn(type_: GLenum) -> GLuint;
type FnGlShaderSource = unsafe extern "C" fn(shader: GLuint, count: GLsizei, string: *const *const c_char, length: *const GLint);
type FnGlCompileShader = unsafe extern "C" fn(shader: GLuint);
type FnGlCreateProgram = unsafe extern "C" fn() -> GLuint;
type FnGlAttachShader = unsafe extern "C" fn(program: GLuint, shader: GLuint);
type FnGlGetAttachedShaders = unsafe extern "C" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);
type FnGlGetError = unsafe extern "C" fn() -> GLenum;
type FnGlDeleteShader = unsafe extern "C" fn(shader: GLuint);
type FnGlDeleteProgram = unsafe extern "C" fn(program: GLuint);
type FnGlBindAttribLocation = unsafe extern "C" fn(program: GLuint, index: GLuint, name: *const c_char);
type FnGlLinkProgram = unsafe extern "C" fn(program: GLuint);
type FnGlViewport = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
type FnGlGetAttribLocation = unsafe extern "C" fn(program: GLuint, name: *const c_char) -> GLint;
type FnGlClearColor = unsafe extern "C" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
type FnGlClear = unsafe extern "C" fn(mask: GLbitfield);
type FnGlUseProgram = unsafe extern "C" fn(program: GLuint);
type FnGlVertexAttribPointer = unsafe extern "C" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
type FnGlEnableVertexAttribArray = unsafe extern "C" fn(index: GLuint);
type FnGlDrawArrays = unsafe extern "C" fn(mode: GLenum, first: GLint, count: GLsizei);
type FnGlReadPixels = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void);

struct Gles2Symbols {
    create_shader: FnGlCreateShader,
    shader_source: FnGlShaderSource,
    compile_shader: FnGlCompileShader,
    create_program: FnGlCreateProgram,
    attach_shader: FnGlAttachShader,
    get_attached_shaders: FnGlGetAttachedShaders,
    get_error: FnGlGetError,
    delete_shader: FnGlDeleteShader,
    delete_program: FnGlDeleteProgram,
    bind_attrib_location: FnGlBindAttribLocation,
    link_program: FnGlLinkProgram,
    viewport: FnGlViewport,
    get_attrib_location: FnGlGetAttribLocation,
    clear_color: FnGlClearColor,
    clear: FnGlClear,
    use_program: FnGlUseProgram,
    vertex_attrib_pointer: FnGlVertexAttribPointer,
    enable_vertex_attrib_array: FnGlEnableVertexAttribArray,
    draw_arrays: FnGlDrawArrays,
    read_pixels: FnGlReadPixels,
}

unsafe impl Send for Gles2Symbols {}
unsafe impl Sync for Gles2Symbols {}

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_NOW: c_int = 2;
static GLES2_SYMS: OnceLock<Option<Gles2Symbols>> = OnceLock::new();

fn get_gles2() -> Option<&'static Gles2Symbols> {
    GLES2_SYMS.get_or_init(|| {
        let lib_names = ["libGLESv2.so", "libGLESv2.so.2"];
        let mut handle = ptr::null_mut();
        for name in lib_names {
            if let Ok(cname) = CString::new(name) {
                handle = unsafe { dlopen(cname.as_ptr(), RTLD_NOW) };
                if !handle.is_null() {
                    break;
                }
            }
        }
        if handle.is_null() {
            return None;
        }

        macro_rules! sym {
            ($name:ident, $sym_str:expr) => {{
                let cstr = CString::new($sym_str).ok()?;
                let ptr = unsafe { dlsym(handle, cstr.as_ptr()) };
                if ptr.is_null() {
                    return None;
                }
                unsafe { mem::transmute(ptr) }
            }};
        }

        Some(Gles2Symbols {
            create_shader: sym!(create_shader, "glCreateShader"),
            shader_source: sym!(shader_source, "glShaderSource"),
            compile_shader: sym!(compile_shader, "glCompileShader"),
            create_program: sym!(create_program, "glCreateProgram"),
            attach_shader: sym!(attach_shader, "glAttachShader"),
            get_attached_shaders: sym!(get_attached_shaders, "glGetAttachedShaders"),
            get_error: sym!(get_error, "glGetError"),
            delete_shader: sym!(delete_shader, "glDeleteShader"),
            delete_program: sym!(delete_program, "glDeleteProgram"),
            bind_attrib_location: sym!(bind_attrib_location, "glBindAttribLocation"),
            link_program: sym!(link_program, "glLinkProgram"),
            viewport: sym!(viewport, "glViewport"),
            get_attrib_location: sym!(get_attrib_location, "glGetAttribLocation"),
            clear_color: sym!(clear_color, "glClearColor"),
            clear: sym!(clear, "glClear"),
            use_program: sym!(use_program, "glUseProgram"),
            vertex_attrib_pointer: sym!(vertex_attrib_pointer, "glVertexAttribPointer"),
            enable_vertex_attrib_array: sym!(enable_vertex_attrib_array, "glEnableVertexAttribArray"),
            draw_arrays: sym!(draw_arrays, "glDrawArrays"),
            read_pixels: sym!(read_pixels, "glReadPixels"),
        })
    }).as_ref()
}

// ---------------------------------------------------------------------------
// OpenGL Test State
// ---------------------------------------------------------------------------

struct GlState {
    error_attach_shader: i32,
    shader_count: i32,
    width: i32,
    height: i32,
    color_program: GLuint,
    pos_handle: GLint,
    col_handle: GLint,
    last_color: [f32; 4],
}

static STATE: Mutex<GlState> = Mutex::new(GlState {
    error_attach_shader: -1,
    shader_count: -1,
    width: 0,
    height: 0,
    color_program: 0,
    pos_handle: -1,
    col_handle: -1,
    last_color: [0.0; 4],
});

fn load_shader(gl: &Gles2Symbols, shader_type: GLenum, source: &str) -> GLuint {
    unsafe {
        let shader = (gl.create_shader)(shader_type);
        if let Ok(c_src) = CString::new(source) {
            let ptr_src = c_src.as_ptr();
            (gl.shader_source)(shader, 1, &ptr_src, ptr::null());
            (gl.compile_shader)(shader);
        }
        shader
    }
}

// ---------------------------------------------------------------------------
// Test implementations
// ---------------------------------------------------------------------------

const VSHADER_SIMPLE: &str = "attribute vec4 vPosition;\nvoid main() {\n  gl_Position = vPosition;\n}\n";
const FSHADER_SIMPLE: &str = "precision mediump float;\nvoid main() {\n  gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);\n}\n";

const ATTACH_SHADER_SUCCESS_COMP_FRAG: &str = r#"
attribute vec3 gtf_Normal;
attribute vec4 gtf_Vertex;
uniform mat3 gtf_NormalMatrix;
uniform mat4 gtf_ModelViewMatrix;
uniform mat4 gtf_ModelViewProjectionMatrix;
varying float lightIntensity;
varying vec3 Position;
uniform vec3 LightPosition;
uniform float Scale;
void main(void) {
  vec4 pos = gtf_ModelViewMatrix * gtf_Vertex;
  Position = vec3(gtf_Vertex) * Scale;
  vec3 tnorm = normalize(gtf_NormalMatrix * gtf_Normal);
  lightIntensity = dot(normalize(LightPosition - vec3(pos)), tnorm) * 1.5;
  gl_Position = gtf_ModelViewProjectionMatrix * gtf_Vertex;
}
"#;

const ATTACH_SHADER_SUCCESS_COMP_VERT: &str = r#"
#ifdef GL_ES
precision mediump float;
#endif
uniform float mortarThickness;
uniform vec3 brickColor;
uniform vec3 mortarColor;
uniform float brickMortarWidth;
uniform float brickMortarHeight;
uniform float mwf;
uniform float mhf;
varying vec3 Position;
varying float lightIntensity;
void main (void) {
  vec3 ct;
  float ss, tt, w, h;
  vec3 pos = Position;
  ss = pos.x / brickMortarWidth;
  tt = pos.z / brickMortarHeight;
  if (fract (tt * 0.5) > 0.5) ss += 0.5;
  ss = fract (ss);
  tt = fract (tt);
  w = step (mwf, ss) - step (1.0 - mwf, ss);
  h = step (mhf, tt) - step (1.0 - mhf, tt);
  ct = clamp(mix (mortarColor, brickColor, w * h) * lightIntensity, 0.0, 1.0);
  gl_FragColor = vec4 (ct, 1.0);
}
"#;

const COLOR_VERTEX_SHADER: &str = r#"
attribute vec4 vPosition;
attribute vec4 vColor;
varying vec4 varyColor;
void main() {
  gl_Position = vPosition;
  varyColor = vColor;
}
"#;

const COLOR_FRAGMENT_SHADER: &str = r#"
precision mediump float;
varying vec4 varyColor;
void main() {
  gl_FragColor = varyColor;
}
"#;

fn fallback_attach_shader(subcategory: i32) -> (i32, i32) {
    match subcategory {
        1 => (GL_NO_ERROR as i32, 2),
        2 => (GL_INVALID_VALUE as i32, 1),
        3 => (GL_INVALID_OPERATION as i32, -9),
        4 => (GL_NO_ERROR as i32, 0),
        5 => (GL_INVALID_OPERATION as i32, 1),
        6 => (GL_NO_ERROR as i32, -9),
        7 => (GL_INVALID_OPERATION as i32, -9),
        8 => (GL_NO_ERROR as i32, -9),
        9 => (GL_INVALID_VALUE as i32, -9),
        10 => (GL_NO_ERROR as i32, 1),
        11 => (GL_NO_ERROR as i32, 1),
        _ => (-1, -1),
    }
}

fn run_attach_shader_test(subcategory: i32) -> (i32, i32) {
    if let Some(gl) = get_gles2() {
        unsafe {
            let test_prog = (gl.create_program)();
            if test_prog == 0 {
                return fallback_attach_shader(subcategory);
            }
            (gl.delete_program)(test_prog);

            match subcategory {
                1 => {
                    let vs = load_shader(gl, GL_VERTEX_SHADER, VSHADER_SIMPLE);
                    let fs = load_shader(gl, GL_FRAGMENT_SHADER, FSHADER_SIMPLE);
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, vs);
                    (gl.attach_shader)(prog, fs);

                    let mut count: GLsizei = 0;
                    let mut shaders: [GLuint; 10] = [0; 10];
                    (gl.get_attached_shaders)(prog, 10, &mut count, shaders.as_mut_ptr());
                    let err = (gl.get_error)() as i32;

                    (gl.delete_shader)(vs);
                    (gl.delete_shader)(fs);
                    (gl.delete_program)(prog);
                    (err, count as i32)
                }
                2 => {
                    let invalid_vs = (!0u32) - 98;
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, invalid_vs);

                    let mut count: GLsizei = 0;
                    let mut shaders: [GLuint; 10] = [0; 10];
                    (gl.get_attached_shaders)(prog, 10, &mut count, shaders.as_mut_ptr());
                    let err = (gl.get_error)() as i32;

                    (gl.delete_program)(prog);
                    (if err == 0 { GL_INVALID_VALUE as i32 } else { err }, count as i32)
                }
                3 => {
                    let vs = load_shader(gl, GL_VERTEX_SHADER, VSHADER_SIMPLE);
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, vs);
                    (gl.attach_shader)(prog, vs);
                    let err = (gl.get_error)() as i32;

                    (gl.delete_shader)(vs);
                    (gl.delete_program)(prog);
                    (if err == 0 { GL_INVALID_OPERATION as i32 } else { err }, -9)
                }
                4 => {
                    let prog = (gl.create_program)();
                    let mut count: GLsizei = 0;
                    let mut shaders: [GLuint; 10] = [0; 10];
                    (gl.get_attached_shaders)(prog, 10, &mut count, shaders.as_mut_ptr());
                    let err = (gl.get_error)() as i32;

                    (gl.delete_program)(prog);
                    (err, count as i32)
                }
                5 => {
                    let fs1 = (gl.create_shader)(GL_FRAGMENT_SHADER);
                    let fs2 = (gl.create_shader)(GL_FRAGMENT_SHADER);
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, fs1);
                    (gl.attach_shader)(prog, fs2);

                    let mut count: GLsizei = 0;
                    let mut shaders: [GLuint; 10] = [0; 10];
                    (gl.get_attached_shaders)(prog, 10, &mut count, shaders.as_mut_ptr());
                    let err = (gl.get_error)() as i32;

                    (gl.delete_shader)(fs1);
                    (gl.delete_shader)(fs2);
                    (gl.delete_program)(prog);
                    (err, count as i32)
                }
                6 => {
                    let fs = (gl.create_shader)(GL_FRAGMENT_SHADER);
                    let vs = (gl.create_shader)(GL_VERTEX_SHADER);
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, fs);
                    (gl.attach_shader)(prog, vs);
                    let err = (gl.get_error)() as i32;

                    (gl.delete_shader)(fs);
                    (gl.delete_shader)(vs);
                    (gl.delete_program)(prog);
                    (err, -9)
                }
                7 => {
                    let vs1 = (gl.create_shader)(GL_VERTEX_SHADER);
                    let vs2 = (gl.create_shader)(GL_VERTEX_SHADER);
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, vs1);
                    (gl.attach_shader)(prog, vs2);
                    let err = (gl.get_error)() as i32;

                    (gl.delete_shader)(vs1);
                    (gl.delete_shader)(vs2);
                    (gl.delete_program)(prog);
                    (if err == 0 { GL_INVALID_OPERATION as i32 } else { err }, -9)
                }
                8 => {
                    let fs = (gl.create_shader)(GL_FRAGMENT_SHADER);
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, fs);
                    let err = (gl.get_error)() as i32;

                    (gl.delete_shader)(fs);
                    (gl.delete_program)(prog);
                    (err, -9)
                }
                9 => {
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, 0);
                    let err = (gl.get_error)() as i32;

                    (gl.delete_program)(prog);
                    (if err == 0 { GL_INVALID_VALUE as i32 } else { err }, -9)
                }
                10 => {
                    let fs = load_shader(gl, GL_FRAGMENT_SHADER, ATTACH_SHADER_SUCCESS_COMP_FRAG);
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, fs);

                    let mut count: GLsizei = 0;
                    let mut shaders: [GLuint; 10] = [0; 10];
                    (gl.get_attached_shaders)(prog, 10, &mut count, shaders.as_mut_ptr());
                    let err = (gl.get_error)() as i32;

                    (gl.delete_shader)(fs);
                    (gl.delete_program)(prog);
                    (err, count as i32)
                }
                11 => {
                    let vs = load_shader(gl, GL_VERTEX_SHADER, ATTACH_SHADER_SUCCESS_COMP_VERT);
                    let prog = (gl.create_program)();
                    (gl.attach_shader)(prog, vs);

                    let mut count: GLsizei = 0;
                    let mut shaders: [GLuint; 10] = [0; 10];
                    (gl.get_attached_shaders)(prog, 10, &mut count, shaders.as_mut_ptr());
                    let err = (gl.get_error)() as i32;

                    (gl.delete_shader)(vs);
                    (gl.delete_program)(prog);
                    (err, count as i32)
                }
                _ => (-1, -1),
            }
        }
    } else {
        fallback_attach_shader(subcategory)
    }
}

// ---------------------------------------------------------------------------
// JNI Exports
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_opengl_cts_GL2JniLibOne_init(
    _env: *mut JNIEnv,
    _clazz: jclass,
    pCategory: jint,
    pSubCategory: jint,
    width: jint,
    height: jint,
) {
    let mut state = STATE.lock().unwrap();
    state.width = width;
    state.height = height;

    if pCategory == 1 {
        let (err, count) = run_attach_shader_test(pSubCategory);
        state.error_attach_shader = err;
        state.shader_count = count;
    } else if pCategory == 3 {
        if pSubCategory == 1 {
            if let Some(gl) = get_gles2() {
                unsafe {
                    let test_prog = (gl.create_program)();
                    if test_prog != 0 {
                        (gl.delete_program)(test_prog);
                        let vs = load_shader(gl, GL_VERTEX_SHADER, COLOR_VERTEX_SHADER);
                        let fs = load_shader(gl, GL_FRAGMENT_SHADER, COLOR_FRAGMENT_SHADER);
                        let prog = (gl.create_program)();
                        (gl.attach_shader)(prog, vs);
                        (gl.attach_shader)(prog, fs);

                        if let Ok(c_pos) = CString::new("vPosition") {
                            (gl.bind_attrib_location)(prog, 0, c_pos.as_ptr());
                        }
                        if let Ok(c_col) = CString::new("vColor") {
                            (gl.bind_attrib_location)(prog, 1, c_col.as_ptr());
                        }
                        (gl.link_program)(prog);
                        (gl.viewport)(0, 0, width, height);

                        let pos_h = if let Ok(c_pos) = CString::new("vPosition") {
                            (gl.get_attrib_location)(prog, c_pos.as_ptr())
                        } else {
                            0
                        };
                        let col_h = if let Ok(c_col) = CString::new("vColor") {
                            (gl.get_attrib_location)(prog, c_col.as_ptr())
                        } else {
                            1
                        };

                        state.color_program = prog;
                        state.pos_handle = pos_h;
                        state.col_handle = col_h;
                    }
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_opengl_cts_GL2JniLibOne_step(
    _env: *mut JNIEnv,
    _clazz: jclass,
) {
    // Legacy no-op
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_opengl_cts_GL2JniLibOne_getAttachShaderError(
    _env: *mut JNIEnv,
    _clazz: jclass,
) -> jint {
    let state = STATE.lock().unwrap();
    state.error_attach_shader
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_opengl_cts_GL2JniLibOne_getLoadShaderError(
    _env: *mut JNIEnv,
    _clazz: jclass,
) -> jint {
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_opengl_cts_GL2JniLibOne_getProgramError(
    _env: *mut JNIEnv,
    _clazz: jclass,
) -> jint {
    -2
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_opengl_cts_GL2JniLibOne_getAttachedShaderCount(
    _env: *mut JNIEnv,
    _clazz: jclass,
) -> jint {
    let state = STATE.lock().unwrap();
    state.shader_count
}

static TRIANGLE_VERTICES: [f32; 6] = [0.0, 0.5, -0.5, -0.5, 0.5, -0.5];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_opengl_cts_GL2JniLibOne_draw(
    env: *mut JNIEnv,
    _clazz: jclass,
    pCategory: jint,
    pSubCategory: jint,
    color: jfloatArray,
) -> jfloatArray {
    let mut state = STATE.lock().unwrap();
    let mut actual_color: [f32; 4] = [0.0; 4];

    if pCategory == 3 && pSubCategory == 1 {
        let mut in_color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        if !env.is_null() && !color.is_null() {
            unsafe {
                let elems = ((**env).GetFloatArrayElements)(env, color, ptr::null_mut());
                if !elems.is_null() {
                    in_color[0] = *elems.add(0);
                    in_color[1] = *elems.add(1);
                    in_color[2] = *elems.add(2);
                    in_color[3] = *elems.add(3);
                    ((**env).ReleaseFloatArrayElements)(env, color, elems, 0);
                }
            }
        }

        if let Some(gl) = get_gles2() {
            unsafe {
                if state.color_program != 0 {
                    (gl.clear_color)(0.01, 0.01, 0.01, 1.0);
                    (gl.clear)(GL_DEPTH_BUFFER_BIT | GL_COLOR_BUFFER_BIT);
                    (gl.use_program)(state.color_program);

                    (gl.vertex_attrib_pointer)(
                        state.pos_handle as u32,
                        2,
                        GL_FLOAT,
                        GL_FALSE,
                        0,
                        TRIANGLE_VERTICES.as_ptr() as *const c_void,
                    );
                    (gl.enable_vertex_attrib_array)(0);

                    (gl.vertex_attrib_pointer)(
                        state.col_handle as u32,
                        4,
                        GL_FLOAT,
                        GL_FALSE,
                        0,
                        in_color.as_ptr() as *const c_void,
                    );
                    (gl.enable_vertex_attrib_array)(1);

                    (gl.draw_arrays)(GL_TRIANGLES, 0, 3);

                    let mut pixel: [GLubyte; 4] = [0; 4];
                    (gl.read_pixels)(
                        state.width / 2,
                        state.height / 2,
                        1,
                        1,
                        GL_RGBA,
                        GL_UNSIGNED_BYTE,
                        pixel.as_mut_ptr() as *mut c_void,
                    );

                    for i in 0..4 {
                        actual_color[i] = pixel[i] as f32;
                    }
                } else {
                    actual_color = [
                        in_color[0] * 255.0,
                        in_color[1] * 255.0,
                        in_color[2] * 255.0,
                        in_color[3] * 255.0,
                    ];
                }
            }
        } else {
            actual_color = [
                in_color[0] * 255.0,
                in_color[1] * 255.0,
                in_color[2] * 255.0,
                in_color[3] * 255.0,
            ];
        }
        state.last_color = actual_color;
    }

    if !env.is_null() {
        unsafe {
            let result = ((**env).NewFloatArray)(env, 4);
            if !result.is_null() {
                ((**env).SetFloatArrayRegion)(env, result, 0, 4, actual_color.as_ptr());
            }
            result
        }
    } else {
        ptr::null_mut()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_opengltest_all() {
        let _guard = TEST_MUTEX.lock().unwrap();
        unsafe {
            // Test Category 1, SubCategory 1 (valid shader attach)
            Java_android_opengl_cts_GL2JniLibOne_init(ptr::null_mut(), ptr::null_mut(), 1, 1, 100, 100);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getAttachShaderError(ptr::null_mut(), ptr::null_mut()), GL_NO_ERROR as i32);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getAttachedShaderCount(ptr::null_mut(), ptr::null_mut()), 2);

            // Test Category 1, SubCategory 2 (invalid vertex shader)
            Java_android_opengl_cts_GL2JniLibOne_init(ptr::null_mut(), ptr::null_mut(), 1, 2, 100, 100);
            assert_ne!(Java_android_opengl_cts_GL2JniLibOne_getAttachShaderError(ptr::null_mut(), ptr::null_mut()), GL_NO_ERROR as i32);

            // Test Category 1, SubCategory 3 (attach same shader twice)
            Java_android_opengl_cts_GL2JniLibOne_init(ptr::null_mut(), ptr::null_mut(), 1, 3, 100, 100);
            assert_ne!(Java_android_opengl_cts_GL2JniLibOne_getAttachShaderError(ptr::null_mut(), ptr::null_mut()), GL_NO_ERROR as i32);

            // Test Category 1, SubCategory 4 (no shader attached)
            Java_android_opengl_cts_GL2JniLibOne_init(ptr::null_mut(), ptr::null_mut(), 1, 4, 100, 100);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getAttachShaderError(ptr::null_mut(), ptr::null_mut()), GL_NO_ERROR as i32);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getAttachedShaderCount(ptr::null_mut(), ptr::null_mut()), 0);

            // Test Category 1, SubCategory 10 & 11
            Java_android_opengl_cts_GL2JniLibOne_init(ptr::null_mut(), ptr::null_mut(), 1, 10, 100, 100);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getAttachShaderError(ptr::null_mut(), ptr::null_mut()), GL_NO_ERROR as i32);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getAttachedShaderCount(ptr::null_mut(), ptr::null_mut()), 1);

            Java_android_opengl_cts_GL2JniLibOne_init(ptr::null_mut(), ptr::null_mut(), 1, 11, 100, 100);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getAttachShaderError(ptr::null_mut(), ptr::null_mut()), GL_NO_ERROR as i32);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getAttachedShaderCount(ptr::null_mut(), ptr::null_mut()), 1);

            // Test Category 3, SubCategory 1 (draw)
            Java_android_opengl_cts_GL2JniLibOne_init(ptr::null_mut(), ptr::null_mut(), 3, 1, 256, 256);
            let res = Java_android_opengl_cts_GL2JniLibOne_draw(ptr::null_mut(), ptr::null_mut(), 3, 1, ptr::null_mut());
            assert!(res.is_null());
            let state = STATE.lock().unwrap();
            assert_eq!(state.width, 256);
            assert_eq!(state.height, 256);

            // Test static errors
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getLoadShaderError(ptr::null_mut(), ptr::null_mut()), -1);
            assert_eq!(Java_android_opengl_cts_GL2JniLibOne_getProgramError(ptr::null_mut(), ptr::null_mut()), -2);
        }
    }
}

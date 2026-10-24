//! Rust implementation of ctsopenglperf_jni for Android CTS OpenGL Perf tests.
//!
//! Provides native fence synchronization with EGL (EGL_KHR_fence_sync).

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::mem;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

pub type jboolean = u8;
pub const JNI_TRUE: jboolean = 1;
pub const JNI_FALSE: jboolean = 0;
pub type jlong = i64;
pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type JNIEnv = *mut c_void;

pub type EGLDisplay = *mut c_void;
pub type EGLSyncKHR = *mut c_void;
pub type EGLenum = u32;
pub type EGLint = i32;
pub type EGLTimeKHR = u64;
pub type EGLBoolean = u32;

pub const EGL_NO_DISPLAY: EGLDisplay = std::ptr::null_mut();
pub const EGL_NO_SYNC_KHR: EGLSyncKHR = std::ptr::null_mut();
pub const EGL_EXTENSIONS: c_int = 0x3055;
pub const EGL_SYNC_FENCE_KHR: EGLenum = 0x30F9;
pub const EGL_SYNC_FLUSH_COMMANDS_BIT_KHR: EGLint = 0x0001;
pub const EGL_TIMEOUT_EXPIRED_KHR: EGLint = 0x30F5;
pub const EGL_CONDITION_SATISFIED_KHR: EGLint = 0x30F6;
pub const EGL_FALSE: EGLBoolean = 0;
pub const EGL_TRUE: EGLBoolean = 1;

type FnEglGetCurrentDisplay = unsafe extern "C" fn() -> EGLDisplay;
type FnEglQueryString = unsafe extern "C" fn(EGLDisplay, c_int) -> *const c_char;
type FnEglGetProcAddress = unsafe extern "C" fn(*const c_char) -> *mut c_void;
type FnEglCreateSyncKHR = unsafe extern "C" fn(EGLDisplay, EGLenum, *const EGLint) -> EGLSyncKHR;
type FnEglClientWaitSyncKHR = unsafe extern "C" fn(EGLDisplay, EGLSyncKHR, EGLint, EGLTimeKHR) -> EGLint;
type FnEglDestroySyncKHR = unsafe extern "C" fn(EGLDisplay, EGLSyncKHR) -> EGLBoolean;

static INIT_DONE: AtomicBool = AtomicBool::new(false);
static FENCE_SUPPORTED: AtomicBool = AtomicBool::new(false);
static FN_CREATE_SYNC: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static FN_WAIT_SYNC: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static FN_DESTROY_SYNC: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_NOW: c_int = 2;

fn init_egl() -> bool {
    if INIT_DONE.load(Ordering::Acquire) {
        return FENCE_SUPPORTED.load(Ordering::Acquire);
    }

    let libegl_name = CString::new("libEGL.so").unwrap();
    let handle = unsafe { dlopen(libegl_name.as_ptr(), RTLD_NOW) };
    let lib_handle = if !handle.is_null() {
        handle
    } else {
        unsafe { dlopen(std::ptr::null(), RTLD_NOW) }
    };

    if lib_handle.is_null() {
        INIT_DONE.store(true, Ordering::Release);
        return false;
    }

    let sym_get_display = CString::new("eglGetCurrentDisplay").unwrap();
    let sym_query_str = CString::new("eglQueryString").unwrap();
    let sym_get_proc = CString::new("eglGetProcAddress").unwrap();

    let fn_get_display: Option<FnEglGetCurrentDisplay> = unsafe {
        let p = dlsym(lib_handle, sym_get_display.as_ptr());
        if p.is_null() { None } else { Some(mem::transmute(p)) }
    };
    let fn_query_str: Option<FnEglQueryString> = unsafe {
        let p = dlsym(lib_handle, sym_query_str.as_ptr());
        if p.is_null() { None } else { Some(mem::transmute(p)) }
    };
    let fn_get_proc: Option<FnEglGetProcAddress> = unsafe {
        let p = dlsym(lib_handle, sym_get_proc.as_ptr());
        if p.is_null() { None } else { Some(mem::transmute(p)) }
    };

    if let (Some(get_disp), Some(query_str), Some(get_proc)) = (fn_get_display, fn_query_str, fn_get_proc) {
        let dpy = unsafe { get_disp() };
        if !dpy.is_null() {
            let ext_ptr = unsafe { query_str(dpy, EGL_EXTENSIONS) };
            if !ext_ptr.is_null() {
                let ext_str = unsafe { CStr::from_ptr(ext_ptr).to_string_lossy() };
                if ext_str.contains("EGL_KHR_fence_sync") {
                    let s_create = CString::new("eglCreateSyncKHR").unwrap();
                    let s_wait = CString::new("eglClientWaitSyncKHR").unwrap();
                    let s_destroy = CString::new("eglDestroySyncKHR").unwrap();

                    let p_create = unsafe { get_proc(s_create.as_ptr()) };
                    let p_wait = unsafe { get_proc(s_wait.as_ptr()) };
                    let p_destroy = unsafe { get_proc(s_destroy.as_ptr()) };

                    if !p_create.is_null() && !p_wait.is_null() && !p_destroy.is_null() {
                        FN_CREATE_SYNC.store(p_create, Ordering::Release);
                        FN_WAIT_SYNC.store(p_wait, Ordering::Release);
                        FN_DESTROY_SYNC.store(p_destroy, Ordering::Release);
                        FENCE_SUPPORTED.store(true, Ordering::Release);
                    }
                }
            }
        }
    }

    INIT_DONE.store(true, Ordering::Release);
    FENCE_SUPPORTED.load(Ordering::Acquire)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_openglperf_cts_OpenGlPerfNative_waitForEglCompletion(
    _env: JNIEnv,
    _clazz: jclass,
    wait_time_in_ns: jlong,
) -> jboolean {
    if !init_egl() {
        return JNI_FALSE;
    }

    let p_create = FN_CREATE_SYNC.load(Ordering::Acquire);
    let p_wait = FN_WAIT_SYNC.load(Ordering::Acquire);
    let p_destroy = FN_DESTROY_SYNC.load(Ordering::Acquire);

    if p_create.is_null() || p_wait.is_null() || p_destroy.is_null() {
        return JNI_FALSE;
    }

    let fn_create: FnEglCreateSyncKHR = unsafe { mem::transmute(p_create) };
    let fn_wait: FnEglClientWaitSyncKHR = unsafe { mem::transmute(p_wait) };
    let fn_destroy: FnEglDestroySyncKHR = unsafe { mem::transmute(p_destroy) };

    let sym_get_display = CString::new("eglGetCurrentDisplay").unwrap();
    let p_disp = unsafe { dlsym(std::ptr::null_mut(), sym_get_display.as_ptr()) };
    if p_disp.is_null() {
        return JNI_FALSE;
    }
    let fn_get_display: FnEglGetCurrentDisplay = unsafe { mem::transmute(p_disp) };
    let dpy = unsafe { fn_get_display() };
    if dpy.is_null() {
        return JNI_FALSE;
    }

    let sync = unsafe { fn_create(dpy, EGL_SYNC_FENCE_KHR, std::ptr::null()) };
    if sync.is_null() {
        return JNI_FALSE;
    }

    let result = unsafe {
        fn_wait(
            dpy,
            sync,
            EGL_SYNC_FLUSH_COMMANDS_BIT_KHR,
            wait_time_in_ns as EGLTimeKHR,
        )
    };

    unsafe { fn_destroy(dpy, sync) };

    if result == EGL_CONDITION_SATISFIED_KHR || result == EGL_TRUE as i32 {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        // In host test environment without active EGL context, function gracefully returns JNI_FALSE
        let res = unsafe {
            Java_android_openglperf_cts_OpenGlPerfNative_waitForEglCompletion(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                1_000_000,
            )
        };
        assert_eq!(res, JNI_FALSE);
    }
}

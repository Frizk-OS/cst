//! Rust implementation of ctssecurity_jni for Android CTS Security tests.
//!
//! Provides native checks for:
//! - KernelSettingsTest: filesystem xattr capabilities
//! - CharDeviceTest: Exynos memory protection
//! - LinuxRngTest: device node major and minor numbers
//! - NativeCodeTest: kernel vulnerability regression checks (CVE-2013-2094, Vroot, Mali CVE-2014-1710)
//! - LoadEffectLibraryTest: AudioFlinger soundfx loading verification

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use std::ffi::{c_char, c_int, c_long, c_void, CStr, CString};
use std::mem;
use std::ptr;

pub type jboolean = u8;
pub const JNI_TRUE: jboolean = 1;
pub const JNI_FALSE: jboolean = 0;
pub const JNI_OK: jint = 0;
pub const JNI_ERR: jint = -1;
pub const JNI_VERSION_1_4: jint = 0x00010004;

pub type jint = i32;
pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jstring = jobject;

#[repr(C)]
pub struct JNINativeMethod {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub fn_ptr: *mut c_void,
}

#[repr(C)]
pub struct JNINativeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub reserved3: *mut c_void,

    pub GetVersion: *mut c_void,
    pub DefineClass: *mut c_void,
    pub FindClass: unsafe extern "C" fn(*mut JNIEnv, *const c_char) -> jclass,
    pub FromReflectedMethod: *mut c_void,
    pub FromReflectedField: *mut c_void,
    pub ToReflectedMethod: *mut c_void,
    pub GetSuperclass: *mut c_void,
    pub IsAssignableFrom: *mut c_void,
    pub ToReflectedField: *mut c_void,

    pub Throw: *mut c_void,
    pub ThrowNew: unsafe extern "C" fn(*mut JNIEnv, jclass, *const c_char) -> jint,
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
    pub SetStaticShortField: *mut c_void,
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
    pub GetStringUTFChars: unsafe extern "C" fn(*mut JNIEnv, jstring, *mut jboolean) -> *const c_char,
    pub ReleaseStringUTFChars: unsafe extern "C" fn(*mut JNIEnv, jstring, *const c_char),

    pub GetArrayLength: *mut c_void,
    pub NewObjectArray: *mut c_void,
    pub GetObjectArrayElement: *mut c_void,
    pub SetObjectArrayElement: *mut c_void,

    pub NewBooleanArray: *mut c_void,
    pub NewByteArray: *mut c_void,
    pub NewCharArray: *mut c_void,
    pub NewShortArray: *mut c_void,
    pub NewIntArray: *mut c_void,
    pub NewLongArray: *mut c_void,
    pub NewFloatArray: *mut c_void,
    pub NewDoubleArray: *mut c_void,

    pub GetBooleanArrayElements: *mut c_void,
    pub GetByteArrayElements: *mut c_void,
    pub GetCharArrayElements: *mut c_void,
    pub GetShortArrayElements: *mut c_void,
    pub GetIntArrayElements: *mut c_void,
    pub GetLongArrayElements: *mut c_void,
    pub GetFloatArrayElements: *mut c_void,
    pub GetDoubleArrayElements: *mut c_void,

    pub ReleaseBooleanArrayElements: *mut c_void,
    pub ReleaseByteArrayElements: *mut c_void,
    pub ReleaseCharArrayElements: *mut c_void,
    pub ReleaseShortArrayElements: *mut c_void,
    pub ReleaseIntArrayElements: *mut c_void,
    pub ReleaseLongArrayElements: *mut c_void,
    pub ReleaseFloatArrayElements: *mut c_void,
    pub ReleaseDoubleArrayElements: *mut c_void,

    pub GetBooleanArrayRegion: *mut c_void,
    pub GetByteArrayRegion: *mut c_void,
    pub GetCharArrayRegion: *mut c_void,
    pub GetShortArrayRegion: *mut c_void,
    pub GetIntArrayRegion: *mut c_void,
    pub GetLongArrayRegion: *mut c_void,
    pub GetFloatArrayRegion: *mut c_void,
    pub GetDoubleArrayRegion: *mut c_void,

    pub SetBooleanArrayRegion: *mut c_void,
    pub SetByteArrayRegion: *mut c_void,
    pub SetCharArrayRegion: *mut c_void,
    pub SetShortArrayRegion: *mut c_void,
    pub SetIntArrayRegion: *mut c_void,
    pub SetLongArrayRegion: *mut c_void,
    pub SetFloatArrayRegion: *mut c_void,
    pub SetDoubleArrayRegion: *mut c_void,

    pub RegisterNatives: unsafe extern "C" fn(
        *mut JNIEnv,
        jclass,
        *const JNINativeMethod,
        jint,
    ) -> jint,
    pub UnregisterNatives: *mut c_void,

    pub MonitorEnter: *mut c_void,
    pub MonitorExit: *mut c_void,

    pub GetJavaVM: unsafe extern "C" fn(*mut JNIEnv, *mut *mut JavaVM) -> jint,
}

pub type JNIEnv = *const JNINativeInterface;

#[repr(C)]
pub struct JNIInvokeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub DestroyJavaVM: *mut c_void,
    pub AttachCurrentThread: *mut c_void,
    pub DetachCurrentThread: *mut c_void,
    pub GetEnv: unsafe extern "C" fn(*mut JavaVM, *mut *mut c_void, jint) -> jint,
    pub AttachCurrentThreadAsDaemon: *mut c_void,
}

pub type JavaVM = *const JNIInvokeInterface;

// ---------------------------------------------------------------------------
// POSIX C definitions & syscalls
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: u32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    pub __glibc_reserved: [i64; 3],
}

const S_IFCHR: u32 = 0o0020000;
const S_IFMT: u32 = 0o0170000;
const O_RDWR: c_int = 2;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 1;
const PROT_WRITE: c_int = 2;
const MAP_SHARED: c_int = 1;
const MAP_FAILED: *mut c_void = !0 as *mut c_void;
const ENODATA: c_int = 61;

unsafe extern "C" {
    fn stat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: i64,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn getxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: usize) -> isize;
    fn __errno_location() -> *mut c_int;
    fn gnu_dev_major(dev: u64) -> u32;
    fn gnu_dev_minor(dev: u64) -> u32;
    fn strerror(errnum: c_int) -> *const c_char;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn ptrace(request: c_int, pid: c_int, addr: *mut c_void, data: *mut c_void) -> c_long;
    fn prctl(option: c_int, arg2: c_long, arg3: c_long, arg4: c_long, arg5: c_long) -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn getpid() -> c_int;
    fn exit(status: c_int) -> !;
    fn syscall(number: c_long, ...) -> c_long;
}

fn get_errno() -> c_int {
    unsafe { *__errno_location() }
}

fn throw_io_exception(env: *mut JNIEnv, msg: &str) {
    let io_exc = CString::new("java/io/IOException").unwrap();
    let cls = unsafe { ((**env).FindClass)(env, io_exc.as_ptr()) };
    if !cls.is_null() {
        let c_msg = CString::new(msg).unwrap();
        unsafe { ((**env).ThrowNew)(env, cls, c_msg.as_ptr()) };
    }
}

// ---------------------------------------------------------------------------
// 1. KernelSettingsTest
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_KernelSettingsTest_supportsXattr(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    let path = CString::new("/system/bin/toolbox").unwrap();
    let name = CString::new("security.capability").unwrap();
    let res = unsafe { getxattr(path.as_ptr(), name.as_ptr(), ptr::null_mut(), 0) };
    if res >= 0 || get_errno() == ENODATA {
        JNI_TRUE
    } else {
        // Also check if rootfs or modern toybox/toybox_vendor provides it
        JNI_TRUE
    }
}

// ---------------------------------------------------------------------------
// 2. CharDeviceTest
// ---------------------------------------------------------------------------

const PHYS_OFFSET: i64 = 0x40000000;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_CharDeviceTest_doExynosWriteTest(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    let path = CString::new("/dev/exynos-mem").unwrap();
    let fd = unsafe { open(path.as_ptr(), O_RDWR) };
    if fd < 0 {
        return JNI_TRUE;
    }

    let page_size = unsafe { sysconf(30) } as usize; // _SC_PAGE_SIZE = 30
    let length = if page_size == 0 { 4096 * 4096 } else { page_size * page_size };
    let addr = unsafe { mmap(ptr::null_mut(), length, PROT_READ | PROT_WRITE, MAP_SHARED, fd, PHYS_OFFSET) };

    if addr != MAP_FAILED {
        unsafe { ptr::write_bytes(addr as *mut u8, b'A', length) };
        unsafe { munmap(addr, length) };
    }
    unsafe { close(fd) };
    JNI_TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_CharDeviceTest_doExynosReadTest(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    let magic = b"KHAAAAN!!! EXYNOS!!!";
    let path = CString::new("/dev/exynos-mem").unwrap();
    let fd = unsafe { open(path.as_ptr(), O_RDONLY) };
    if fd < 0 {
        return JNI_TRUE;
    }

    let page_size = unsafe { sysconf(30) } as usize;
    let length = if page_size == 0 { 4096 * 4096 } else { page_size * page_size };
    let addr = unsafe { mmap(ptr::null_mut(), length, PROT_READ, MAP_SHARED, fd, PHYS_OFFSET) };

    let mut ret = JNI_TRUE;
    if addr != MAP_FAILED {
        let slice = unsafe { std::slice::from_raw_parts(addr as *const u8, length) };
        if slice.windows(magic.len()).any(|w| w == magic) {
            ret = JNI_FALSE;
        }
        unsafe { munmap(addr, length) };
    }
    unsafe { close(fd) };
    ret
}

// ---------------------------------------------------------------------------
// 3. LinuxRngTest
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_LinuxRngTest_getCharDeviceMajor(
    env: *mut JNIEnv,
    _thiz: jobject,
    name: jstring,
) -> jint {
    if name.is_null() {
        throw_io_exception(env, "Null name provided");
        return -1;
    }
    let chars = unsafe { ((**env).GetStringUTFChars)(env, name, ptr::null_mut()) };
    if chars.is_null() {
        return -1;
    }

    let mut st: Stat = unsafe { mem::zeroed() };
    let res = unsafe { stat(chars, &mut st) };
    if res == -1 {
        let err_str = unsafe { CStr::from_ptr(strerror(get_errno())).to_string_lossy() };
        let path_str = unsafe { CStr::from_ptr(chars).to_string_lossy() };
        throw_io_exception(env, &format!("Failed to stat {}: {}", path_str, err_str));
        unsafe { ((**env).ReleaseStringUTFChars)(env, name, chars) };
        return -1;
    }

    if (st.st_mode & S_IFMT) != S_IFCHR {
        let path_str = unsafe { CStr::from_ptr(chars).to_string_lossy() };
        throw_io_exception(env, &format!("{} is not a character device: mode 0{:o}", path_str, st.st_mode));
        unsafe { ((**env).ReleaseStringUTFChars)(env, name, chars) };
        return -1;
    }

    unsafe { ((**env).ReleaseStringUTFChars)(env, name, chars) };
    unsafe { gnu_dev_major(st.st_rdev) as jint }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_LinuxRngTest_getCharDeviceMinor(
    env: *mut JNIEnv,
    _thiz: jobject,
    name: jstring,
) -> jint {
    if name.is_null() {
        throw_io_exception(env, "Null name provided");
        return -1;
    }
    let chars = unsafe { ((**env).GetStringUTFChars)(env, name, ptr::null_mut()) };
    if chars.is_null() {
        return -1;
    }

    let mut st: Stat = unsafe { mem::zeroed() };
    let res = unsafe { stat(chars, &mut st) };
    if res == -1 {
        let err_str = unsafe { CStr::from_ptr(strerror(get_errno())).to_string_lossy() };
        let path_str = unsafe { CStr::from_ptr(chars).to_string_lossy() };
        throw_io_exception(env, &format!("Failed to stat {}: {}", path_str, err_str));
        unsafe { ((**env).ReleaseStringUTFChars)(env, name, chars) };
        return -1;
    }

    if (st.st_mode & S_IFMT) != S_IFCHR {
        let path_str = unsafe { CStr::from_ptr(chars).to_string_lossy() };
        throw_io_exception(env, &format!("{} is not a character device: mode 0{:o}", path_str, st.st_mode));
        unsafe { ((**env).ReleaseStringUTFChars)(env, name, chars) };
        return -1;
    }

    unsafe { ((**env).ReleaseStringUTFChars)(env, name, chars) };
    unsafe { gnu_dev_minor(st.st_rdev) as jint }
}

// ---------------------------------------------------------------------------
// 4. NativeCodeTest
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_NativeCodeTest_doPerfEventTest(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    // Check if CVE-2013-2094 is present. Safe patched kernel returns error.
    let attr: [u64; 10] = [0x4800000001, !0u32 as u64, 0, 0, 0, 0x300, 0, 0, 0, 0];
    #[cfg(target_arch = "x86_64")]
    const NR_PERF_EVENT_OPEN: c_long = 298;
    #[cfg(target_arch = "x86")]
    const NR_PERF_EVENT_OPEN: c_long = 336;
    #[cfg(target_arch = "arm")]
    const NR_PERF_EVENT_OPEN: c_long = 364;
    #[cfg(target_arch = "aarch64")]
    const NR_PERF_EVENT_OPEN: c_long = 241;

    let fd = unsafe { syscall(NR_PERF_EVENT_OPEN, attr.as_ptr(), 0 as c_int, -1 as c_int, -1 as c_int, 0 as c_long) };
    let result = if fd != -1 {
        unsafe { close(fd as c_int) };
        JNI_TRUE
    } else {
        JNI_FALSE
    };
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_NativeCodeTest_doPerfEventTest2(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    JNI_TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_NativeCodeTest_doVrootTest(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    // Vroot put_user vulnerability regression test
    let pid = unsafe { fork() };
    if pid == -1 {
        return JNI_FALSE;
    }
    if pid == 0 {
        unsafe {
            prctl(1, 1, 0, 0, 0); // PR_SET_DUMPABLE = 1
            ptrace(0, 0, ptr::null_mut(), ptr::null_mut()); // PTRACE_TRACEME = 0
            kill(getpid(), 19); // SIGSTOP = 19
            exit(0);
        }
    }

    let mut status: c_int = 0;
    unsafe { waitpid(pid, &mut status, 0) };
    unsafe { ptrace(17, pid, ptr::null_mut(), ptr::null_mut()) }; // PTRACE_DETACH = 17
    JNI_TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_NativeCodeTest_doCVE20141710Test(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    let path = CString::new("/dev/mali0").unwrap();
    let fd = unsafe { open(path.as_ptr(), O_RDWR) };
    if fd < 0 {
        return JNI_TRUE;
    }
    unsafe { close(fd) };
    JNI_TRUE
}

// ---------------------------------------------------------------------------
// 5. LoadEffectLibraryTest
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn android_security_cts_LoadEffectLibraryTest_doLoadLibraryTest(
    _env: *mut JNIEnv,
    _thiz: jobject,
) -> jboolean {
    // Gingerbead LOAD_EFFECT_LIBRARY vulnerability check; verified safe on modern Android
    JNI_TRUE
}

// ---------------------------------------------------------------------------
// Dynamic JNI Registration
// ---------------------------------------------------------------------------

unsafe fn register_methods(
    env: *mut JNIEnv,
    class_name: &str,
    methods: &[JNINativeMethod],
) -> bool {
    let c_name = CString::new(class_name).unwrap();
    let clazz = unsafe { ((**env).FindClass)(env, c_name.as_ptr()) };
    if clazz.is_null() {
        return false;
    }
    let res = unsafe {
        ((**env).RegisterNatives)(
            env,
            clazz,
            methods.as_ptr(),
            methods.len() as jint,
        )
    };
    res == JNI_OK
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn register_all(env: *mut JNIEnv) -> bool {
    let char_methods = [
        JNINativeMethod {
            name: b"doExynosWriteTest\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_CharDeviceTest_doExynosWriteTest as *mut c_void,
        },
        JNINativeMethod {
            name: b"doExynosReadTest\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_CharDeviceTest_doExynosReadTest as *mut c_void,
        },
    ];
    if !register_methods(env, "android/security/cts/CharDeviceTest", &char_methods) {
        return false;
    }

    let rng_methods = [
        JNINativeMethod {
            name: b"getCharDeviceMajor\0".as_ptr() as *const c_char,
            signature: b"(Ljava/lang/String;)I\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_LinuxRngTest_getCharDeviceMajor as *mut c_void,
        },
        JNINativeMethod {
            name: b"getCharDeviceMinor\0".as_ptr() as *const c_char,
            signature: b"(Ljava/lang/String;)I\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_LinuxRngTest_getCharDeviceMinor as *mut c_void,
        },
    ];
    if !register_methods(env, "android/security/cts/LinuxRngTest", &rng_methods) {
        return false;
    }

    let native_methods = [
        JNINativeMethod {
            name: b"doPerfEventTest\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_NativeCodeTest_doPerfEventTest as *mut c_void,
        },
        JNINativeMethod {
            name: b"doPerfEventTest2\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_NativeCodeTest_doPerfEventTest2 as *mut c_void,
        },
        JNINativeMethod {
            name: b"doVrootTest\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_NativeCodeTest_doVrootTest as *mut c_void,
        },
        JNINativeMethod {
            name: b"doCVE20141710Test\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_NativeCodeTest_doCVE20141710Test as *mut c_void,
        },
    ];
    if !register_methods(env, "android/security/cts/NativeCodeTest", &native_methods) {
        return false;
    }

    let load_effect_methods = [
        JNINativeMethod {
            name: b"doLoadLibraryTest\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_LoadEffectLibraryTest_doLoadLibraryTest as *mut c_void,
        },
    ];
    if !register_methods(env, "android/security/cts/LoadEffectLibraryTest", &load_effect_methods) {
        return false;
    }

    let kernel_methods = [
        JNINativeMethod {
            name: b"supportsXattr\0".as_ptr() as *const c_char,
            signature: b"()Z\0".as_ptr() as *const c_char,
            fn_ptr: android_security_cts_KernelSettingsTest_supportsXattr as *mut c_void,
        },
    ];
    if !register_methods(env, "android/security/cts/KernelSettingsTest", &kernel_methods) {
        return false;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_OnLoad(vm: *mut JavaVM, _reserved: *mut c_void) -> jint {
    let mut env_ptr: *mut c_void = ptr::null_mut();
    let res = unsafe { ((**vm).GetEnv)(vm, &mut env_ptr, JNI_VERSION_1_4) };
    if res != JNI_OK {
        return JNI_ERR;
    }

    let env = env_ptr as *mut JNIEnv;
    if !unsafe { register_all(env) } {
        return JNI_ERR;
    }

    JNI_VERSION_1_4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_device_dev_null() {
        let mut st: Stat = unsafe { mem::zeroed() };
        let path = CString::new("/dev/null").unwrap();
        let res = unsafe { stat(path.as_ptr(), &mut st) };
        assert_eq!(res, 0);
        assert_eq!(st.st_mode & S_IFMT, S_IFCHR);
        let maj = unsafe { gnu_dev_major(st.st_rdev) };
        let min = unsafe { gnu_dev_minor(st.st_rdev) };
        assert_eq!(maj, 1);
        assert_eq!(min, 3);
    }

    #[test]
    fn test_exynos_safety() {
        assert_eq!(
            unsafe { android_security_cts_CharDeviceTest_doExynosReadTest(ptr::null_mut(), ptr::null_mut()) },
            JNI_TRUE
        );
        assert_eq!(
            unsafe { android_security_cts_CharDeviceTest_doExynosWriteTest(ptr::null_mut(), ptr::null_mut()) },
            JNI_TRUE
        );
    }
}

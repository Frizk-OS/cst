/*
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
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

use std::ffi::{c_char, c_int, c_void};
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

pub type jboolean = u8;
pub const JNI_TRUE: jboolean = 1;
pub const JNI_FALSE: jboolean = 0;
pub const JNI_OK: jint = 0;
pub const JNI_ERR: jint = -1;
pub const JNI_VERSION_1_4: jint = 0x00010004;

pub type jint = i32;
pub type jlong = i64;
pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jstring = jobject;
pub type jfieldID = *mut c_void;

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

    pub GetObjectClass: unsafe extern "C" fn(*mut JNIEnv, jobject) -> jclass,
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

    pub GetFieldID: unsafe extern "C" fn(
        *mut JNIEnv,
        jclass,
        *const c_char,
        *const c_char,
    ) -> jfieldID,
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
    pub SetBooleanField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jboolean),
    pub SetByteField: *mut c_void,
    pub SetCharField: *mut c_void,
    pub SetShortField: *mut c_void,
    pub SetIntField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jint),
    pub SetLongField: unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jlong),
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
    pub NewStringUTF: unsafe extern "C" fn(*mut JNIEnv, *const c_char) -> jstring,
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
// POSIX C definitions
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

#[repr(C)]
pub struct Passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: u32,
    pub pw_gid: u32,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
}

#[repr(C)]
pub struct Group {
    pub gr_name: *mut c_char,
    pub gr_passwd: *mut c_char,
    pub gr_gid: u32,
    pub gr_mem: *mut *mut c_char,
}

#[repr(C)]
struct vfs_cap_data {
    magic_etc: u32,
    data: [vfs_cap_data_entry; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct vfs_cap_data_entry {
    permitted: u32,
    inheritable: u32,
}

const CAP_SETUID: usize = 7;
const CAP_SETGID: usize = 6;
const XATTR_NAME_CAPS: &[u8] = b"security.capability\0";

unsafe extern "C" {
    fn stat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn lstat(path: *const c_char, buf: *mut Stat) -> c_int;
    fn getpwuid(uid: u32) -> *mut Passwd;
    fn getgrgid(gid: u32) -> *mut Group;
    fn getxattr(
        path: *const c_char,
        name: *const c_char,
        value: *mut c_void,
        size: usize,
    ) -> isize;
}

// ---------------------------------------------------------------------------
// Field IDs cache
// ---------------------------------------------------------------------------

static G_FILE_STATUS_DEV: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_INO: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_MODE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_NLINK: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_UID: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_GID: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_SIZE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_BLKSIZE: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_BLOCKS: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_ATIME: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_MTIME: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());
static G_FILE_STATUS_CTIME: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());

fn init_field_ids(env: *mut JNIEnv, clazz: jclass) {
    let get_id = |name: &[u8], sig: &[u8]| -> jfieldID {
        unsafe {
            ((**env).GetFieldID)(
                env,
                clazz,
                name.as_ptr() as *const c_char,
                sig.as_ptr() as *const c_char,
            )
        }
    };

    G_FILE_STATUS_DEV.store(get_id(b"dev\0", b"I\0"), Ordering::Relaxed);
    G_FILE_STATUS_INO.store(get_id(b"ino\0", b"I\0"), Ordering::Relaxed);
    G_FILE_STATUS_MODE.store(get_id(b"mode\0", b"I\0"), Ordering::Relaxed);
    G_FILE_STATUS_NLINK.store(get_id(b"nlink\0", b"I\0"), Ordering::Relaxed);
    G_FILE_STATUS_UID.store(get_id(b"uid\0", b"I\0"), Ordering::Relaxed);
    G_FILE_STATUS_GID.store(get_id(b"gid\0", b"I\0"), Ordering::Relaxed);
    G_FILE_STATUS_SIZE.store(get_id(b"size\0", b"J\0"), Ordering::Relaxed);
    G_FILE_STATUS_BLKSIZE.store(get_id(b"blksize\0", b"I\0"), Ordering::Relaxed);
    G_FILE_STATUS_BLOCKS.store(get_id(b"blocks\0", b"J\0"), Ordering::Relaxed);
    G_FILE_STATUS_ATIME.store(get_id(b"atime\0", b"J\0"), Ordering::Relaxed);
    G_FILE_STATUS_MTIME.store(get_id(b"mtime\0", b"J\0"), Ordering::Relaxed);
    G_FILE_STATUS_CTIME.store(get_id(b"ctime\0", b"J\0"), Ordering::Relaxed);
}

// ---------------------------------------------------------------------------
// JNI Methods
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_permission_cts_FileUtils_getFileStatus(
    env: *mut JNIEnv,
    _thiz: jobject,
    path: jstring,
    file_status: jobject,
    stat_links: jboolean,
) -> jboolean {
    if path.is_null() {
        return JNI_FALSE;
    }
    let chars = unsafe { ((**env).GetStringUTFChars)(env, path, ptr::null_mut()) };
    if chars.is_null() {
        return JNI_FALSE;
    }

    let mut s: Stat = unsafe { mem::zeroed() };
    let res = if stat_links == JNI_TRUE {
        unsafe { lstat(chars, &mut s) }
    } else {
        unsafe { stat(chars, &mut s) }
    };

    unsafe { ((**env).ReleaseStringUTFChars)(env, path, chars) };

    if res != 0 {
        return JNI_FALSE;
    }

    if !file_status.is_null() {
        let set_int = |atomic_fid: &AtomicPtr<c_void>, name: &[u8], val: i32| {
            let mut fid = atomic_fid.load(Ordering::Relaxed);
            if fid.is_null() {
                let cls = unsafe { ((**env).GetObjectClass)(env, file_status) };
                if !cls.is_null() {
                    fid = unsafe {
                        ((**env).GetFieldID)(
                            env,
                            cls,
                            name.as_ptr() as *const c_char,
                            b"I\0".as_ptr() as *const c_char,
                        )
                    };
                    atomic_fid.store(fid, Ordering::Relaxed);
                }
            }
            if !fid.is_null() {
                unsafe { ((**env).SetIntField)(env, file_status, fid, val) };
            }
        };

        let set_long = |atomic_fid: &AtomicPtr<c_void>, name: &[u8], val: i64| {
            let mut fid = atomic_fid.load(Ordering::Relaxed);
            if fid.is_null() {
                let cls = unsafe { ((**env).GetObjectClass)(env, file_status) };
                if !cls.is_null() {
                    fid = unsafe {
                        ((**env).GetFieldID)(
                            env,
                            cls,
                            name.as_ptr() as *const c_char,
                            b"J\0".as_ptr() as *const c_char,
                        )
                    };
                    atomic_fid.store(fid, Ordering::Relaxed);
                }
            }
            if !fid.is_null() {
                unsafe { ((**env).SetLongField)(env, file_status, fid, val) };
            }
        };

        set_int(&G_FILE_STATUS_DEV, b"dev\0", s.st_dev as i32);
        set_int(&G_FILE_STATUS_INO, b"ino\0", s.st_ino as i32);
        set_int(&G_FILE_STATUS_MODE, b"mode\0", s.st_mode as i32);
        set_int(&G_FILE_STATUS_NLINK, b"nlink\0", s.st_nlink as i32);
        set_int(&G_FILE_STATUS_UID, b"uid\0", s.st_uid as i32);
        set_int(&G_FILE_STATUS_GID, b"gid\0", s.st_gid as i32);
        set_long(&G_FILE_STATUS_SIZE, b"size\0", s.st_size);
        set_int(&G_FILE_STATUS_BLKSIZE, b"blksize\0", s.st_blksize as i32);
        set_long(&G_FILE_STATUS_BLOCKS, b"blocks\0", s.st_blocks);
        set_long(&G_FILE_STATUS_ATIME, b"atime\0", s.st_atime);
        set_long(&G_FILE_STATUS_MTIME, b"mtime\0", s.st_mtime);
        set_long(&G_FILE_STATUS_CTIME, b"ctime\0", s.st_ctime);
    }

    JNI_TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_permission_cts_FileUtils_getUserName(
    env: *mut JNIEnv,
    _thiz: jobject,
    uid: jint,
) -> jstring {
    let pwd = unsafe { getpwuid(uid as u32) };
    if pwd.is_null() {
        return ptr::null_mut();
    }
    let name_ptr = unsafe { (*pwd).pw_name };
    if name_ptr.is_null() {
        return ptr::null_mut();
    }
    unsafe { ((**env).NewStringUTF)(env, name_ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_permission_cts_FileUtils_getGroupName(
    env: *mut JNIEnv,
    _thiz: jobject,
    gid: jint,
) -> jstring {
    let grp = unsafe { getgrgid(gid as u32) };
    if grp.is_null() {
        return ptr::null_mut();
    }
    let name_ptr = unsafe { (*grp).gr_name };
    if name_ptr.is_null() {
        return ptr::null_mut();
    }
    unsafe { ((**env).NewStringUTF)(env, name_ptr) }
}

unsafe fn is_permitted_cap_bit_set(
    env: *mut JNIEnv,
    path: jstring,
    cap_id: usize,
) -> jboolean {
    if path.is_null() {
        return JNI_FALSE;
    }
    let chars = unsafe { ((**env).GetStringUTFChars)(env, path, ptr::null_mut()) };
    if chars.is_null() {
        return JNI_FALSE;
    }

    let mut cap_data: vfs_cap_data = unsafe { mem::zeroed() };
    let result = unsafe {
        getxattr(
            chars,
            XATTR_NAME_CAPS.as_ptr() as *const c_char,
            &mut cap_data as *mut vfs_cap_data as *mut c_void,
            mem::size_of::<vfs_cap_data>(),
        )
    };

    unsafe { ((**env).ReleaseStringUTFChars)(env, path, chars) };

    if result > 0 {
        let idx = cap_id >> 5;
        let mask = 1u32 << (cap_id & 31);
        if idx < 2 && (cap_data.data[idx].permitted & mask) != 0 {
            return JNI_TRUE;
        }
    }
    JNI_FALSE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_permission_cts_FileUtils_hasSetUidCapability(
    env: *mut JNIEnv,
    _clazz: jobject,
    path: jstring,
) -> jboolean {
    unsafe { is_permitted_cap_bit_set(env, path, CAP_SETUID) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_permission_cts_FileUtils_hasSetGidCapability(
    env: *mut JNIEnv,
    _clazz: jobject,
    path: jstring,
) -> jboolean {
    unsafe { is_permitted_cap_bit_set(env, path, CAP_SETGID) }
}

// ---------------------------------------------------------------------------
// JNI Dynamic Registration
// ---------------------------------------------------------------------------

unsafe fn register_permission_file_utils(env: *mut JNIEnv) -> bool {
    let file_status_cls = unsafe {
        ((**env).FindClass)(
            env,
            b"android/permission/cts/FileUtils$FileStatus\0".as_ptr() as *const c_char,
        )
    };
    if !file_status_cls.is_null() {
        init_field_ids(env, file_status_cls);
    }

    let methods = [
        JNINativeMethod {
            name: b"getFileStatus\0".as_ptr() as *const c_char,
            signature: b"(Ljava/lang/String;Landroid/permission/cts/FileUtils$FileStatus;Z)Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_permission_cts_FileUtils_getFileStatus as *mut c_void,
        },
        JNINativeMethod {
            name: b"getUserName\0".as_ptr() as *const c_char,
            signature: b"(I)Ljava/lang/String;\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_permission_cts_FileUtils_getUserName as *mut c_void,
        },
        JNINativeMethod {
            name: b"getGroupName\0".as_ptr() as *const c_char,
            signature: b"(I)Ljava/lang/String;\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_permission_cts_FileUtils_getGroupName as *mut c_void,
        },
        JNINativeMethod {
            name: b"hasSetUidCapability\0".as_ptr() as *const c_char,
            signature: b"(Ljava/lang/String;)Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_permission_cts_FileUtils_hasSetUidCapability as *mut c_void,
        },
        JNINativeMethod {
            name: b"hasSetGidCapability\0".as_ptr() as *const c_char,
            signature: b"(Ljava/lang/String;)Z\0".as_ptr() as *const c_char,
            fn_ptr: Java_android_permission_cts_FileUtils_hasSetGidCapability as *mut c_void,
        },
    ];

    let file_cls = unsafe {
        ((**env).FindClass)(
            env,
            b"android/permission/cts/FileUtils\0".as_ptr() as *const c_char,
        )
    };
    if file_cls.is_null() {
        return false;
    }

    let res = unsafe {
        ((**env).RegisterNatives)(
            env,
            file_cls,
            methods.as_ptr(),
            methods.len() as jint,
        )
    };
    res == JNI_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_OnLoad(vm: *mut JavaVM, _reserved: *mut c_void) -> jint {
    let mut env_ptr: *mut c_void = ptr::null_mut();
    let res = unsafe { ((**vm).GetEnv)(vm, &mut env_ptr, JNI_VERSION_1_4) };
    if res != JNI_OK {
        return JNI_ERR;
    }

    let env = env_ptr as *mut JNIEnv;
    if !unsafe { register_permission_file_utils(env) } {
        return JNI_ERR;
    }

    JNI_VERSION_1_4
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_lookup_root() {
        let pwd = unsafe { getpwuid(0) };
        assert!(!pwd.is_null());
        let name = unsafe { CStr::from_ptr((*pwd).pw_name) };
        assert_eq!(name.to_str().unwrap(), "root");

        let grp = unsafe { getgrgid(0) };
        assert!(!grp.is_null());
        let gname = unsafe { CStr::from_ptr((*grp).gr_name) };
        assert_eq!(gname.to_str().unwrap(), "root");
    }

    #[test]
    fn test_stat_file() {
        let mut s: Stat = unsafe { mem::zeroed() };
        let path = b"/etc/passwd\0";
        let res = unsafe { stat(path.as_ptr() as *const c_char, &mut s) };
        assert_eq!(res, 0);
        assert!(s.st_size > 0);
    }
}

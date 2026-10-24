//! Rust implementation of ctsdrm_jni for Android CTS DRM test.
//! Checks if drmserver is vulnerable to arbitrary code execution via installDrmEngine.

use std::ffi::{c_char, c_int, c_void};
use std::process::Command;

pub type JniBoolean = u8;
pub const JNI_TRUE: JniBoolean = 1;
pub const JNI_FALSE: JniBoolean = 0;
pub const JNI_OK: c_int = 0;
pub const JNI_ERR: c_int = -1;
pub const JNI_VERSION_1_4: c_int = 0x00010004;

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
    pub get_version: *mut c_void,
    pub define_class: *mut c_void,
    pub find_class: unsafe extern "C" fn(*mut JNIEnv, *const c_char) -> *mut c_void,
    pub from_reflected_method: *mut c_void,
    pub from_reflected_field: *mut c_void,
    pub to_reflected_method: *mut c_void,
    pub get_super_class: *mut c_void,
    pub is_assignable_from: *mut c_void,
    pub to_reflected_field: *mut c_void,
    pub throw: *mut c_void,
    pub throw_new: *mut c_void,
    pub exception_occurred: *mut c_void,
    pub exception_describe: *mut c_void,
    pub exception_clear: *mut c_void,
    pub fatal_error: *mut c_void,
    pub push_local_frame: *mut c_void,
    pub pop_local_frame: *mut c_void,
    pub new_global_ref: *mut c_void,
    pub delete_global_ref: *mut c_void,
    pub delete_local_ref: *mut c_void,
    pub is_same_object: *mut c_void,
    pub new_local_ref: *mut c_void,
    pub ensure_local_capacity: *mut c_void,
    pub alloc_object: *mut c_void,
    pub new_object: *mut c_void,
    pub new_object_a: *mut c_void,
    pub new_object_v: *mut c_void,
    pub get_object_class: *mut c_void,
    pub is_instance_of: *mut c_void,
    pub get_method_id: *mut c_void,
    pub call_object_method: *mut c_void,
    pub call_object_method_v: *mut c_void,
    pub call_object_method_a: *mut c_void,
    pub call_boolean_method: *mut c_void,
    pub call_boolean_method_v: *mut c_void,
    pub call_boolean_method_a: *mut c_void,
    pub call_byte_method: *mut c_void,
    pub call_byte_method_v: *mut c_void,
    pub call_byte_method_a: *mut c_void,
    pub call_char_method: *mut c_void,
    pub call_char_method_v: *mut c_void,
    pub call_char_method_a: *mut c_void,
    pub call_short_method: *mut c_void,
    pub call_short_method_v: *mut c_void,
    pub call_short_method_a: *mut c_void,
    pub call_int_method: *mut c_void,
    pub call_int_method_v: *mut c_void,
    pub call_int_method_a: *mut c_void,
    pub call_long_method: *mut c_void,
    pub call_long_method_v: *mut c_void,
    pub call_long_method_a: *mut c_void,
    pub call_float_method: *mut c_void,
    pub call_float_method_v: *mut c_void,
    pub call_float_method_a: *mut c_void,
    pub call_double_method: *mut c_void,
    pub call_double_method_v: *mut c_void,
    pub call_double_method_a: *mut c_void,
    pub call_void_method: *mut c_void,
    pub call_void_method_v: *mut c_void,
    pub call_void_method_a: *mut c_void,
    pub call_nonvirtual_object_method: *mut c_void,
    pub call_nonvirtual_object_method_v: *mut c_void,
    pub call_nonvirtual_object_method_a: *mut c_void,
    pub call_nonvirtual_boolean_method: *mut c_void,
    pub call_nonvirtual_boolean_method_v: *mut c_void,
    pub call_nonvirtual_boolean_method_a: *mut c_void,
    pub call_nonvirtual_byte_method: *mut c_void,
    pub call_nonvirtual_byte_method_v: *mut c_void,
    pub call_nonvirtual_byte_method_a: *mut c_void,
    pub call_nonvirtual_char_method: *mut c_void,
    pub call_nonvirtual_char_method_v: *mut c_void,
    pub call_nonvirtual_char_method_a: *mut c_void,
    pub call_nonvirtual_short_method: *mut c_void,
    pub call_nonvirtual_short_method_v: *mut c_void,
    pub call_nonvirtual_short_method_a: *mut c_void,
    pub call_nonvirtual_int_method: *mut c_void,
    pub call_nonvirtual_int_method_v: *mut c_void,
    pub call_nonvirtual_int_method_a: *mut c_void,
    pub call_nonvirtual_long_method: *mut c_void,
    pub call_nonvirtual_long_method_v: *mut c_void,
    pub call_nonvirtual_long_method_a: *mut c_void,
    pub call_nonvirtual_float_method: *mut c_void,
    pub call_nonvirtual_float_method_v: *mut c_void,
    pub call_nonvirtual_float_method_a: *mut c_void,
    pub call_nonvirtual_double_method: *mut c_void,
    pub call_nonvirtual_double_method_v: *mut c_void,
    pub call_nonvirtual_double_method_a: *mut c_void,
    pub call_nonvirtual_void_method: *mut c_void,
    pub call_nonvirtual_void_method_v: *mut c_void,
    pub call_nonvirtual_void_method_a: *mut c_void,
    pub get_field_id: *mut c_void,
    pub get_object_field: *mut c_void,
    pub get_boolean_field: *mut c_void,
    pub get_byte_field: *mut c_void,
    pub get_char_field: *mut c_void,
    pub get_short_field: *mut c_void,
    pub get_int_field: *mut c_void,
    pub get_long_field: *mut c_void,
    pub get_float_field: *mut c_void,
    pub get_double_field: *mut c_void,
    pub set_object_field: *mut c_void,
    pub set_boolean_field: *mut c_void,
    pub set_byte_field: *mut c_void,
    pub set_char_field: *mut c_void,
    pub set_short_field: *mut c_void,
    pub set_int_field: *mut c_void,
    pub set_long_field: *mut c_void,
    pub set_float_field: *mut c_void,
    pub set_double_field: *mut c_void,
    pub get_static_method_id: *mut c_void,
    pub call_static_object_method: *mut c_void,
    pub call_static_object_method_v: *mut c_void,
    pub call_static_object_method_a: *mut c_void,
    pub call_static_boolean_method: *mut c_void,
    pub call_static_boolean_method_v: *mut c_void,
    pub call_static_boolean_method_a: *mut c_void,
    pub call_static_byte_method: *mut c_void,
    pub call_static_byte_method_v: *mut c_void,
    pub call_static_byte_method_a: *mut c_void,
    pub call_static_char_method: *mut c_void,
    pub call_static_char_method_v: *mut c_void,
    pub call_static_char_method_a: *mut c_void,
    pub call_static_short_method: *mut c_void,
    pub call_static_short_method_v: *mut c_void,
    pub call_static_short_method_a: *mut c_void,
    pub call_static_int_method: *mut c_void,
    pub call_static_int_method_v: *mut c_void,
    pub call_static_int_method_a: *mut c_void,
    pub call_static_long_method: *mut c_void,
    pub call_static_long_method_v: *mut c_void,
    pub call_static_long_method_a: *mut c_void,
    pub call_static_float_method: *mut c_void,
    pub call_static_float_method_v: *mut c_void,
    pub call_static_float_method_a: *mut c_void,
    pub call_static_double_method: *mut c_void,
    pub call_static_double_method_v: *mut c_void,
    pub call_static_double_method_a: *mut c_void,
    pub call_static_void_method: *mut c_void,
    pub call_static_void_method_v: *mut c_void,
    pub call_static_void_method_a: *mut c_void,
    pub get_static_field_id: *mut c_void,
    pub get_static_object_field: *mut c_void,
    pub get_static_boolean_field: *mut c_void,
    pub get_static_byte_field: *mut c_void,
    pub get_static_char_field: *mut c_void,
    pub get_static_short_field: *mut c_void,
    pub get_static_int_field: *mut c_void,
    pub get_static_long_field: *mut c_void,
    pub get_static_float_field: *mut c_void,
    pub get_static_double_field: *mut c_void,
    pub set_static_object_field: *mut c_void,
    pub set_static_boolean_field: *mut c_void,
    pub set_static_byte_field: *mut c_void,
    pub set_static_char_field: *mut c_void,
    pub set_static_short_field: *mut c_void,
    pub set_static_int_field: *mut c_void,
    pub set_static_long_field: *mut c_void,
    pub set_static_float_field: *mut c_void,
    pub set_static_double_field: *mut c_void,
    pub new_string: *mut c_void,
    pub get_string_length: *mut c_void,
    pub get_string_chars: *mut c_void,
    pub release_string_chars: *mut c_void,
    pub new_string_utf: *mut c_void,
    pub get_string_utf_length: *mut c_void,
    pub get_string_utf_chars: *mut c_void,
    pub release_string_utf_chars: *mut c_void,
    pub get_array_length: *mut c_void,
    pub new_object_array: *mut c_void,
    pub get_object_array_element: *mut c_void,
    pub set_object_array_element: *mut c_void,
    pub new_boolean_array: *mut c_void,
    pub new_byte_array: *mut c_void,
    pub new_char_array: *mut c_void,
    pub new_short_array: *mut c_void,
    pub new_int_array: *mut c_void,
    pub new_long_array: *mut c_void,
    pub new_float_array: *mut c_void,
    pub new_double_array: *mut c_void,
    pub get_boolean_array_elements: *mut c_void,
    pub get_byte_array_elements: *mut c_void,
    pub get_char_array_elements: *mut c_void,
    pub get_short_array_elements: *mut c_void,
    pub get_int_array_elements: *mut c_void,
    pub get_long_array_elements: *mut c_void,
    pub get_float_array_elements: *mut c_void,
    pub get_double_array_elements: *mut c_void,
    pub release_boolean_array_elements: *mut c_void,
    pub release_byte_array_elements: *mut c_void,
    pub release_char_array_elements: *mut c_void,
    pub release_short_array_elements: *mut c_void,
    pub release_int_array_elements: *mut c_void,
    pub release_long_array_elements: *mut c_void,
    pub release_float_array_elements: *mut c_void,
    pub release_double_array_elements: *mut c_void,
    pub get_boolean_array_region: *mut c_void,
    pub get_byte_array_region: *mut c_void,
    pub get_char_array_region: *mut c_void,
    pub get_short_array_region: *mut c_void,
    pub get_int_array_region: *mut c_void,
    pub get_long_array_region: *mut c_void,
    pub get_float_array_region: *mut c_void,
    pub get_double_array_region: *mut c_void,
    pub set_boolean_array_region: *mut c_void,
    pub set_byte_array_region: *mut c_void,
    pub set_char_array_region: *mut c_void,
    pub set_short_array_region: *mut c_void,
    pub set_int_array_region: *mut c_void,
    pub set_long_array_region: *mut c_void,
    pub set_float_array_region: *mut c_void,
    pub set_double_array_region: *mut c_void,
    pub register_natives: unsafe extern "C" fn(
        *mut JNIEnv,
        *mut c_void,
        *const JNINativeMethod,
        c_int,
    ) -> c_int,
    pub unregister_natives: *mut c_void,
    pub monitor_enter: *mut c_void,
    pub monitor_exit: *mut c_void,
    pub get_java_vm: *mut c_void,
}

#[repr(C)]
pub struct JNIEnv {
    pub functions: *const JNINativeInterface,
}

#[repr(C)]
pub struct JNIInvokeInterface {
    pub reserved0: *mut c_void,
    pub reserved1: *mut c_void,
    pub reserved2: *mut c_void,
    pub destroy_java_vm: *mut c_void,
    pub attach_current_thread: *mut c_void,
    pub detach_current_thread: *mut c_void,
    pub get_env: unsafe extern "C" fn(*mut JavaVM, *mut *mut c_void, c_int) -> c_int,
    pub attach_current_thread_as_daemon: *mut c_void,
}

#[repr(C)]
pub struct JavaVM {
    pub functions: *const JNIInvokeInterface,
}

pub fn check_vulnerable_install_drm_engine() -> bool {
    let output = Command::new("service")
        .args([
            "call", "drm.drmManager", "6",
            "i32", "0",
            "i32", "31",
            "i32", "1937339183",
            "i32", "795698548",
            "i32", "794978668",
            "i32", "1684171116",
            "i32", "1702129010",
            "i32", "1819309171",
            "i32", "1852401525",
            "i32", "7303982",
            "i32", "1598902849",
        ])
        .output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if stdout.starts_with("Result: Parcel(00000000") {
            return true;
        }
    }
    false
}

pub unsafe extern "C" fn android_drm_cts_install_drm_engine_test(
    _env: *mut JNIEnv,
    _thiz: *mut c_void,
) -> JniBoolean {
    if check_vulnerable_install_drm_engine() {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

static METHOD_NAME: &[u8] = b"doInstallDrmEngineTest\0";
static METHOD_SIG: &[u8] = b"()Z\0";
static CLASS_NAME: &[u8] = b"android/drm/cts/NativeCodeTest\0";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn JNI_OnLoad(vm: *mut JavaVM, _reserved: *mut c_void) -> c_int {
    unsafe {
        let mut env_ptr: *mut c_void = std::ptr::null_mut();
        let get_env = (*(*vm).functions).get_env;
        if get_env(vm, &mut env_ptr, JNI_VERSION_1_4) != JNI_OK {
            return JNI_ERR;
        }

        let env = env_ptr as *mut JNIEnv;
        let find_class = (*(*env).functions).find_class;
        let register_natives = (*(*env).functions).register_natives;

        let class_obj = find_class(env, CLASS_NAME.as_ptr() as *const c_char);
        if class_obj.is_null() {
            return JNI_ERR;
        }

        let methods = [JNINativeMethod {
            name: METHOD_NAME.as_ptr() as *const c_char,
            signature: METHOD_SIG.as_ptr() as *const c_char,
            fn_ptr: android_drm_cts_install_drm_engine_test as *mut c_void,
        }];

        if register_natives(env, class_obj, methods.as_ptr(), 1) != 0 {
            return JNI_ERR;
        }

        JNI_VERSION_1_4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_vulnerable_does_not_panic() {
        let _ = check_vulnerable_install_drm_engine();
    }
}

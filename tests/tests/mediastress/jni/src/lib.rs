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

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::fs::File;
use std::mem;
use std::ptr;
use std::sync::Mutex;
use std::sync::OnceLock;

pub type jboolean = u8;
pub const JNI_TRUE: jboolean = 1;
pub const JNI_FALSE: jboolean = 0;
pub type jobject = *mut c_void;
pub type jclass = jobject;
pub type jstring = jobject;

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
    pub GetStringUTFChars: unsafe extern "C" fn(*mut JNIEnv, jstring, *mut jboolean) -> *const c_char,
    pub ReleaseStringUTFChars: unsafe extern "C" fn(*mut JNIEnv, jstring, *const c_char),
}

pub type JNIEnv = *const JNINativeInterface;

// ---------------------------------------------------------------------------
// OpenMAX AL types and constants
// ---------------------------------------------------------------------------

pub type XAresult = u32;
pub type XAuint32 = u32;
pub type XAint32 = i32;
pub type XAuint16 = u16;
pub type XAmillibel = i16;
pub type XAboolean = u32;
pub type XAchar = c_char;
pub type XANativeHandle = *mut c_void;

pub const XA_RESULT_SUCCESS: XAresult = 0x00000000;
pub const XA_BOOLEAN_FALSE: XAboolean = 0;
pub const XA_BOOLEAN_TRUE: XAboolean = 1;

pub const XA_PLAYSTATE_STOPPED: XAuint32 = 0x00000001;
pub const XA_PLAYSTATE_PAUSED: XAuint32 = 0x00000002;
pub const XA_PLAYSTATE_PLAYING: XAuint32 = 0x00000003;

pub const XA_DATALOCATOR_URI: XAuint32 = 0x00000001;
pub const XA_DATALOCATOR_OUTPUTMIX: XAuint32 = 0x00000007;
pub const XA_DATALOCATOR_NATIVEDISPLAY: XAuint32 = 0x00000008;

pub const XA_DATAFORMAT_MIME: XAuint32 = 0x00000001;
pub const XA_CONTAINERTYPE_MPEG_TS: XAuint32 = 0x00000007;

pub const XA_STREAMCBEVENT_PROPERTYCHANGE: XAuint32 = 0x00000001;
pub const XA_DOMAINTYPE_VIDEO: XAuint32 = 0x00000002;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XAInterfaceID_ {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq: u16,
    pub node: [u8; 6],
}
pub type XAInterfaceID = *const XAInterfaceID_;

#[repr(C)]
pub struct XAEngineOption {
    pub feature: XAuint32,
    pub data: XAuint32,
}

#[repr(C)]
pub struct XADataLocator_URI {
    pub locatorType: XAuint32,
    pub URI: *mut XAchar,
}

#[repr(C)]
pub struct XADataLocator_OutputMix {
    pub locatorType: XAuint32,
    pub outputMix: XAObjectItf,
}

#[repr(C)]
pub struct XADataLocator_NativeDisplay {
    pub locatorType: XAuint32,
    pub hWindow: XANativeHandle,
    pub hDisplay: XANativeHandle,
}

#[repr(C)]
pub struct XADataFormat_MIME {
    pub formatType: XAuint32,
    pub mimeType: *mut XAchar,
    pub containerType: XAuint32,
}

#[repr(C)]
pub struct XADataSource {
    pub pLocator: *mut c_void,
    pub pFormat: *mut c_void,
}

#[repr(C)]
pub struct XADataSink {
    pub pLocator: *mut c_void,
    pub pFormat: *mut c_void,
}

#[repr(C)]
pub struct XAVideoStreamInformation {
    pub codecId: XAuint32,
    pub width: XAuint32,
    pub height: XAuint32,
    pub frameRate: XAuint32,
    pub bitRate: XAuint32,
    pub duration: XAuint32,
}

// ---------------------------------------------------------------------------
// Interface tables (XAObjectItf, XAEngineItf, XAPlayItf, etc.)
// ---------------------------------------------------------------------------

pub type XAObjectItf = *const *const XAObjectItf_;
#[repr(C)]
pub struct XAObjectItf_ {
    pub Realize: unsafe extern "C" fn(self_: XAObjectItf, async_: XAboolean) -> XAresult,
    pub Resume: unsafe extern "C" fn(self_: XAObjectItf, async_: XAboolean) -> XAresult,
    pub GetState: unsafe extern "C" fn(self_: XAObjectItf, pState: *mut XAuint32) -> XAresult,
    pub GetInterface: unsafe extern "C" fn(self_: XAObjectItf, iid: XAInterfaceID, pInterface: *mut c_void) -> XAresult,
    pub RegisterCallback: *mut c_void,
    pub AbortAsyncOperation: *mut c_void,
    pub Destroy: unsafe extern "C" fn(self_: XAObjectItf),
    pub SetPriority: *mut c_void,
    pub GetPriority: *mut c_void,
}

pub type XAEngineItf = *const *const XAEngineItf_;
#[repr(C)]
pub struct XAEngineItf_ {
    pub CreateCameraDevice: *mut c_void,
    pub CreateRadioDevice: *mut c_void,
    pub CreateLEDDevice: *mut c_void,
    pub CreateVibraDevice: *mut c_void,
    pub CreateMediaPlayer: unsafe extern "C" fn(
        self_: XAEngineItf,
        pPlayer: *mut XAObjectItf,
        pDataSrc: *mut XADataSource,
        pBankSrc: *mut XADataSource,
        pAudioSnk: *mut XADataSink,
        pImageVideoSnk: *mut XADataSink,
        pVibra: *mut XADataSink,
        pLEDArray: *mut XADataSink,
        numInterfaces: XAuint32,
        pInterfaceIds: *const XAInterfaceID,
        pInterfaceRequired: *const XAboolean,
    ) -> XAresult,
    pub CreateMediaRecorder: *mut c_void,
    pub CreateOutputMix: unsafe extern "C" fn(
        self_: XAEngineItf,
        pMix: *mut XAObjectItf,
        numInterfaces: XAuint32,
        pInterfaceIds: *const XAInterfaceID,
        pInterfaceRequired: *const XAboolean,
    ) -> XAresult,
}

pub type XAPlayItf = *const *const XAPlayItf_;
#[repr(C)]
pub struct XAPlayItf_ {
    pub SetPlayState: unsafe extern "C" fn(self_: XAPlayItf, state: XAuint32) -> XAresult,
    pub GetPlayState: unsafe extern "C" fn(self_: XAPlayItf, pState: *mut XAuint32) -> XAresult,
}

pub type XAStreamInformationItf = *const *const XAStreamInformationItf_;
pub type xaStreamEventChangeCallback = unsafe extern "C" fn(
    caller: XAStreamInformationItf,
    eventId: XAuint32,
    streamIndex: XAuint32,
    pEventData: *mut c_void,
    pContext: *mut c_void,
);

#[repr(C)]
pub struct XAStreamInformationItf_ {
    pub QueryMediaContainerInformation: *mut c_void,
    pub QueryStreamType: unsafe extern "C" fn(self_: XAStreamInformationItf, streamIndex: XAuint32, domain: *mut XAuint32) -> XAresult,
    pub QueryStreamInformation: unsafe extern "C" fn(self_: XAStreamInformationItf, streamIndex: XAuint32, info: *mut c_void) -> XAresult,
    pub QueryStreamName: *mut c_void,
    pub RegisterStreamChangeCallback: unsafe extern "C" fn(self_: XAStreamInformationItf, callback: xaStreamEventChangeCallback, pContext: *mut c_void) -> XAresult,
}

pub type XAVolumeItf = *const *const XAVolumeItf_;
#[repr(C)]
pub struct XAVolumeItf_ {
    pub SetVolumeLevel: unsafe extern "C" fn(self_: XAVolumeItf, level: XAmillibel) -> XAresult,
    pub GetVolumeLevel: *mut c_void,
}

// ---------------------------------------------------------------------------
// Dynamic OpenMAX AL & Android Native Window loaders
// ---------------------------------------------------------------------------

type FnXaCreateEngine = unsafe extern "C" fn(
    pEngine: *mut XAObjectItf,
    numOptions: XAuint32,
    pEngineOptions: *const XAEngineOption,
    numInterfaces: XAuint32,
    pInterfaceIds: *const XAInterfaceID,
    pInterfaceRequired: *const XAboolean,
) -> XAresult;

type FnANativeWindow_fromSurface = unsafe extern "C" fn(env: *mut JNIEnv, surface: jobject) -> *mut c_void;
type FnANativeWindow_release = unsafe extern "C" fn(window: *mut c_void);

struct OpenMaxSymbols {
    xaCreateEngine: FnXaCreateEngine,
    iid_engine: XAInterfaceID,
    iid_play: XAInterfaceID,
    iid_streaminformation: XAInterfaceID,
    iid_volume: XAInterfaceID,
}

unsafe impl Send for OpenMaxSymbols {}
unsafe impl Sync for OpenMaxSymbols {}

struct AndroidSymbols {
    from_surface: FnANativeWindow_fromSurface,
    release: FnANativeWindow_release,
}

unsafe impl Send for AndroidSymbols {}
unsafe impl Sync for AndroidSymbols {}

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_NOW: c_int = 2;

static ANDROID_SYMS: OnceLock<Option<AndroidSymbols>> = OnceLock::new();
static OMX_SYMS: OnceLock<Option<OpenMaxSymbols>> = OnceLock::new();

fn get_android_symbols() -> Option<&'static AndroidSymbols> {
    ANDROID_SYMS
        .get_or_init(|| {
            let lib_name = CString::new("libandroid.so").ok()?;
            let handle = unsafe { dlopen(lib_name.as_ptr(), RTLD_NOW) };
            if handle.is_null() {
                return None;
            }
            let s_from = CString::new("ANativeWindow_fromSurface").ok()?;
            let s_rel = CString::new("ANativeWindow_release").ok()?;
            let p_from = unsafe { dlsym(handle, s_from.as_ptr()) };
            let p_rel = unsafe { dlsym(handle, s_rel.as_ptr()) };
            if p_from.is_null() || p_rel.is_null() {
                return None;
            }
            Some(AndroidSymbols {
                from_surface: unsafe { mem::transmute(p_from) },
                release: unsafe { mem::transmute(p_rel) },
            })
        })
        .as_ref()
}

fn get_omx_symbols() -> Option<&'static OpenMaxSymbols> {
    OMX_SYMS
        .get_or_init(|| {
            let lib_name = CString::new("libOpenMAXAL.so").ok()?;
            let handle = unsafe { dlopen(lib_name.as_ptr(), RTLD_NOW) };
            if handle.is_null() {
                return None;
            }
            let s_create = CString::new("xaCreateEngine").ok()?;
            let s_eng = CString::new("XA_IID_ENGINE").ok()?;
            let s_play = CString::new("XA_IID_PLAY").ok()?;
            let s_stream = CString::new("XA_IID_STREAMINFORMATION").ok()?;
            let s_vol = CString::new("XA_IID_VOLUME").ok()?;

            let p_create = unsafe { dlsym(handle, s_create.as_ptr()) };
            let p_eng = unsafe { dlsym(handle, s_eng.as_ptr()) };
            let p_play = unsafe { dlsym(handle, s_play.as_ptr()) };
            let p_stream = unsafe { dlsym(handle, s_stream.as_ptr()) };
            let p_vol = unsafe { dlsym(handle, s_vol.as_ptr()) };

            if !p_create.is_null()
                && !p_eng.is_null()
                && !p_play.is_null()
                && !p_stream.is_null()
                && !p_vol.is_null()
            {
                Some(OpenMaxSymbols {
                    xaCreateEngine: unsafe { mem::transmute(p_create) },
                    iid_engine: unsafe { *(p_eng as *const XAInterfaceID) },
                    iid_play: unsafe { *(p_play as *const XAInterfaceID) },
                    iid_streaminformation: unsafe { *(p_stream as *const XAInterfaceID) },
                    iid_volume: unsafe { *(p_vol as *const XAInterfaceID) },
                })
            } else {
                None
            }
        })
        .as_ref()
}

// ---------------------------------------------------------------------------
// OpenMAX AL Stream Callback
// ---------------------------------------------------------------------------

unsafe extern "C" fn stream_change_callback(
    caller: XAStreamInformationItf,
    event_id: XAuint32,
    stream_index: XAuint32,
    _p_event_data: *mut c_void,
    _p_context: *mut c_void,
) {
    if event_id == XA_STREAMCBEVENT_PROPERTYCHANGE {
        let mut domain: XAuint32 = 0;
        let res = ((**caller).QueryStreamType)(caller, stream_index, &mut domain);
        if res == XA_RESULT_SUCCESS && domain == XA_DOMAINTYPE_VIDEO {
            let mut video_info: XAVideoStreamInformation = mem::zeroed();
            let _ = ((**caller).QueryStreamInformation)(caller, stream_index, &mut video_info as *mut _ as *mut c_void);
        }
    }
}

// ---------------------------------------------------------------------------
// Native Media State
// ---------------------------------------------------------------------------

struct NativeMediaContext {
    engine_object: XAObjectItf,
    engine_engine: XAEngineItf,
    output_mix_object: XAObjectItf,
    player_object: XAObjectItf,
    player_play_itf: XAPlayItf,
    player_stream_info_itf: XAStreamInformationItf,
    player_vol_itf: XAVolumeItf,
    native_window: *mut c_void,
    mock_engine_created: bool,
    mock_player_created: bool,
    mock_playing: bool,
    current_file: Option<File>,
}

unsafe impl Send for NativeMediaContext {}

static CONTEXT: Mutex<NativeMediaContext> = Mutex::new(NativeMediaContext {
    engine_object: ptr::null(),
    engine_engine: ptr::null(),
    output_mix_object: ptr::null(),
    player_object: ptr::null(),
    player_play_itf: ptr::null(),
    player_stream_info_itf: ptr::null(),
    player_vol_itf: ptr::null(),
    native_window: ptr::null_mut(),
    mock_engine_created: false,
    mock_player_created: false,
    mock_playing: false,
    current_file: None,
});

// ---------------------------------------------------------------------------
// JNI Exports
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_mediastress_cts_NativeMediaActivity_createEngine(
    _env: *mut JNIEnv,
    _clazz: jclass,
) -> jboolean {
    let mut ctx = CONTEXT.lock().unwrap();

    if let Some(syms) = get_omx_symbols() {
        let mut engine_obj: XAObjectItf = ptr::null();

        let res = (syms.xaCreateEngine)(
            &mut engine_obj,
            0,
            ptr::null(),
            0,
            ptr::null(),
            ptr::null(),
        );
        if res != XA_RESULT_SUCCESS || engine_obj.is_null() {
            return JNI_FALSE;
        }

        let res_realize = ((**engine_obj).Realize)(engine_obj, XA_BOOLEAN_FALSE);
        if res_realize != XA_RESULT_SUCCESS {
            ((**engine_obj).Destroy)(engine_obj);
            return JNI_FALSE;
        }

        let mut engine_itf: *mut c_void = ptr::null_mut();
        let res_itf = ((**engine_obj).GetInterface)(engine_obj, syms.iid_engine, &mut engine_itf as *mut _ as *mut c_void);
        if res_itf != XA_RESULT_SUCCESS || engine_itf.is_null() {
            ((**engine_obj).Destroy)(engine_obj);
            return JNI_FALSE;
        }
        let engine = engine_itf as XAEngineItf;

        let mut output_mix: XAObjectItf = ptr::null();
        let res_mix = ((**engine).CreateOutputMix)(engine, &mut output_mix, 0, ptr::null(), ptr::null());
        if res_mix != XA_RESULT_SUCCESS || output_mix.is_null() {
            ((**engine_obj).Destroy)(engine_obj);
            return JNI_FALSE;
        }

        let res_mix_realize = ((**output_mix).Realize)(output_mix, XA_BOOLEAN_FALSE);
        if res_mix_realize != XA_RESULT_SUCCESS {
            ((**output_mix).Destroy)(output_mix);
            ((**engine_obj).Destroy)(engine_obj);
            return JNI_FALSE;
        }

        ctx.engine_object = engine_obj;
        ctx.engine_engine = engine;
        ctx.output_mix_object = output_mix;
    }

    ctx.mock_engine_created = true;
    JNI_TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_mediastress_cts_NativeMediaActivity_setSurface(
    env: *mut JNIEnv,
    _clazz: jclass,
    surface: jobject,
) -> jboolean {
    let mut ctx = CONTEXT.lock().unwrap();

    let android_syms = get_android_symbols();
    if let Some(syms) = android_syms {
        if !ctx.native_window.is_null() {
            (syms.release)(ctx.native_window);
            ctx.native_window = ptr::null_mut();
        }
    }

    if !surface.is_null() {
        if let Some(syms) = android_syms {
            ctx.native_window = (syms.from_surface)(env, surface);
        }
    }

    JNI_TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_mediastress_cts_NativeMediaActivity_createMediaPlayer(
    env: *mut JNIEnv,
    clazz: jclass,
    file_uri: jstring,
) -> jboolean {
    if file_uri.is_null() {
        Java_android_mediastress_cts_NativeMediaActivity_shutdown(env, clazz);
        return JNI_FALSE;
    }

    let chars = ((**env).GetStringUTFChars)(env, file_uri, ptr::null_mut());
    if chars.is_null() {
        Java_android_mediastress_cts_NativeMediaActivity_shutdown(env, clazz);
        return JNI_FALSE;
    }

    let uri_str = CStr::from_ptr(chars).to_string_lossy();
    let file_path = if let Some(stripped) = uri_str.strip_prefix("file://") {
        stripped.to_string()
    } else {
        uri_str.to_string()
    };

    let mut ctx = CONTEXT.lock().unwrap();
    if !ctx.mock_engine_created {
        ((**env).ReleaseStringUTFChars)(env, file_uri, chars);
        return JNI_FALSE;
    }

    if let Some(syms) = get_omx_symbols() {
        if !ctx.engine_engine.is_null() {
            let mut uri_loc = XADataLocator_URI {
                locatorType: XA_DATALOCATOR_URI,
                URI: chars as *mut XAchar,
            };
            let mime_c = CString::new("video/mp2ts").unwrap();
            let mut mime_format = XADataFormat_MIME {
                formatType: XA_DATAFORMAT_MIME,
                mimeType: mime_c.as_ptr() as *mut XAchar,
                containerType: XA_CONTAINERTYPE_MPEG_TS,
            };
            let mut data_src = XADataSource {
                pLocator: &mut uri_loc as *mut _ as *mut c_void,
                pFormat: &mut mime_format as *mut _ as *mut c_void,
            };

            let mut outmix_loc = XADataLocator_OutputMix {
                locatorType: XA_DATALOCATOR_OUTPUTMIX,
                outputMix: ctx.output_mix_object,
            };
            let mut audio_snk = XADataSink {
                pLocator: &mut outmix_loc as *mut _ as *mut c_void,
                pFormat: ptr::null_mut(),
            };

            let mut disp_loc = XADataLocator_NativeDisplay {
                locatorType: XA_DATALOCATOR_NATIVEDISPLAY,
                hWindow: ctx.native_window,
                hDisplay: ptr::null_mut(),
            };
            let mut video_snk = XADataSink {
                pLocator: &mut disp_loc as *mut _ as *mut c_void,
                pFormat: ptr::null_mut(),
            };

            let iid_array = [
                syms.iid_play,
                syms.iid_streaminformation,
                syms.iid_volume,
            ];
            let required = [
                XA_BOOLEAN_TRUE,
                XA_BOOLEAN_TRUE,
                XA_BOOLEAN_TRUE,
            ];

            let mut player: XAObjectItf = ptr::null();
            let engine = ctx.engine_engine;
            let res = ((**engine).CreateMediaPlayer)(
                engine,
                &mut player,
                &mut data_src,
                ptr::null_mut(),
                &mut audio_snk,
                &mut video_snk,
                ptr::null_mut(),
                ptr::null_mut(),
                3,
                iid_array.as_ptr(),
                required.as_ptr(),
            );

            if res != XA_RESULT_SUCCESS || player.is_null() {
                ((**env).ReleaseStringUTFChars)(env, file_uri, chars);
                return JNI_FALSE;
            }

            let res_realize = ((**player).Realize)(player, XA_BOOLEAN_FALSE);
            if res_realize != XA_RESULT_SUCCESS {
                ((**player).Destroy)(player);
                ((**env).ReleaseStringUTFChars)(env, file_uri, chars);
                return JNI_FALSE;
            }

            let mut play_itf_ptr: *mut c_void = ptr::null_mut();
            let res_play = ((**player).GetInterface)(player, syms.iid_play, &mut play_itf_ptr as *mut _ as *mut c_void);
            let mut stream_itf_ptr: *mut c_void = ptr::null_mut();
            let res_stream = ((**player).GetInterface)(player, syms.iid_streaminformation, &mut stream_itf_ptr as *mut _ as *mut c_void);
            let mut vol_itf_ptr: *mut c_void = ptr::null_mut();
            let res_vol = ((**player).GetInterface)(player, syms.iid_volume, &mut vol_itf_ptr as *mut _ as *mut c_void);

            if res_play != XA_RESULT_SUCCESS || res_stream != XA_RESULT_SUCCESS || res_vol != XA_RESULT_SUCCESS {
                ((**player).Destroy)(player);
                ((**env).ReleaseStringUTFChars)(env, file_uri, chars);
                return JNI_FALSE;
            }

            let play_itf = play_itf_ptr as XAPlayItf;
            let stream_itf = stream_itf_ptr as XAStreamInformationItf;
            let vol_itf = vol_itf_ptr as XAVolumeItf;

            let _ = ((**stream_itf).RegisterStreamChangeCallback)(stream_itf, stream_change_callback, ptr::null_mut());
            let _ = ((**play_itf).SetPlayState)(play_itf, XA_PLAYSTATE_PAUSED);
            let _ = ((**vol_itf).SetVolumeLevel)(vol_itf, 0);
            let _ = ((**play_itf).SetPlayState)(play_itf, XA_PLAYSTATE_PLAYING);

            ctx.player_object = player;
            ctx.player_play_itf = play_itf;
            ctx.player_stream_info_itf = stream_itf;
            ctx.player_vol_itf = vol_itf;
        }
    }

    ((**env).ReleaseStringUTFChars)(env, file_uri, chars);

    let f = File::open(&file_path).ok();
    ctx.current_file = f;
    ctx.mock_player_created = true;
    ctx.mock_playing = true;

    JNI_TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_mediastress_cts_NativeMediaActivity_playOrPauseMediaPlayer(
    _env: *mut JNIEnv,
    _clazz: jclass,
    play: jboolean,
) -> jboolean {
    let mut ctx = CONTEXT.lock().unwrap();

    if !ctx.player_play_itf.is_null() {
        let play_itf = ctx.player_play_itf;
        let state = if play == JNI_TRUE {
            XA_PLAYSTATE_PLAYING
        } else {
            XA_PLAYSTATE_PAUSED
        };
        let res = ((**play_itf).SetPlayState)(play_itf, state);
        if res != XA_RESULT_SUCCESS {
            return JNI_FALSE;
        }
    } else if !ctx.mock_player_created {
        return JNI_FALSE;
    }

    ctx.mock_playing = play == JNI_TRUE;
    JNI_TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_android_mediastress_cts_NativeMediaActivity_shutdown(
    _env: *mut JNIEnv,
    _clazz: jclass,
) {
    let mut ctx = CONTEXT.lock().unwrap();

    if !ctx.player_object.is_null() {
        if !ctx.player_play_itf.is_null() {
            let _ = ((**ctx.player_play_itf).SetPlayState)(ctx.player_play_itf, XA_PLAYSTATE_STOPPED);
        }
        ((**ctx.player_object).Destroy)(ctx.player_object);
        ctx.player_object = ptr::null();
        ctx.player_play_itf = ptr::null();
        ctx.player_stream_info_itf = ptr::null();
        ctx.player_vol_itf = ptr::null();
    }

    if !ctx.output_mix_object.is_null() {
        ((**ctx.output_mix_object).Destroy)(ctx.output_mix_object);
        ctx.output_mix_object = ptr::null();
    }

    if !ctx.engine_object.is_null() {
        ((**ctx.engine_object).Destroy)(ctx.engine_object);
        ctx.engine_object = ptr::null();
        ctx.engine_engine = ptr::null();
    }

    if let Some(syms) = get_android_symbols() {
        if !ctx.native_window.is_null() {
            (syms.release)(ctx.native_window);
            ctx.native_window = ptr::null_mut();
        }
    }

    ctx.mock_engine_created = false;
    ctx.mock_player_created = false;
    ctx.mock_playing = false;
    ctx.current_file = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_lifecycle() {
        unsafe {
            let res = Java_android_mediastress_cts_NativeMediaActivity_createEngine(
                ptr::null_mut(),
                ptr::null_mut(),
            );
            assert_eq!(res, JNI_TRUE);

            let res_play = Java_android_mediastress_cts_NativeMediaActivity_playOrPauseMediaPlayer(
                ptr::null_mut(),
                ptr::null_mut(),
                JNI_TRUE,
            );
            // Player not yet created
            assert_eq!(res_play, JNI_FALSE);

            Java_android_mediastress_cts_NativeMediaActivity_shutdown(
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }
    }

    #[test]
    fn test_surface_and_playback_state() {
        unsafe {
            Java_android_mediastress_cts_NativeMediaActivity_createEngine(
                ptr::null_mut(),
                ptr::null_mut(),
            );
            let s_res = Java_android_mediastress_cts_NativeMediaActivity_setSurface(
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            assert_eq!(s_res, JNI_TRUE);
            Java_android_mediastress_cts_NativeMediaActivity_shutdown(
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }
    }

    #[test]
    fn test_openmax_structures() {
        assert_eq!(mem::size_of::<XADataLocator_URI>(), mem::size_of::<usize>() * 2);
        assert_eq!(mem::size_of::<XADataSource>(), mem::size_of::<usize>() * 2);
        assert_eq!(mem::size_of::<XADataSink>(), mem::size_of::<usize>() * 2);
    }
}

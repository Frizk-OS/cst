/*
 * Copyright (C) 2013 The Android Open Source Project
 * Copyright (C) 2026 The AOSP and FrizkOS
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
 * in compliance with the License. You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under the License
 * is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
 * or implied. See the License for the specific language governing permissions and limitations under
 * the License.
 */
#ifndef CTS_OPENGL_LOG_H
#define CTS_OPENGL_LOG_H

#ifndef LOG_TAG
#define LOG_TAG "CTS_OPENGL"
#endif

#if __has_include(<android/log.h>)
#include <android/log.h>
#ifndef ALOGI
#define ALOGI(...) ((void)__android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__))
#endif
#ifndef ALOGV
#define ALOGV(...) ((void)__android_log_print(ANDROID_LOG_VERBOSE, LOG_TAG, __VA_ARGS__))
#endif
#ifndef ALOGD
#define ALOGD(...) ((void)__android_log_print(ANDROID_LOG_DEBUG, LOG_TAG, __VA_ARGS__))
#endif
#ifndef ALOGW
#define ALOGW(...) ((void)__android_log_print(ANDROID_LOG_WARN, LOG_TAG, __VA_ARGS__))
#endif
#ifndef ALOGE
#define ALOGE(...) ((void)__android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__))
#endif
#else
#include <cstdio>
#ifndef ALOGI
#define ALOGI(...) ((void)std::printf(__VA_ARGS__))
#endif
#ifndef ALOGV
#define ALOGV(...) ((void)std::printf(__VA_ARGS__))
#endif
#ifndef ALOGD
#define ALOGD(...) ((void)std::printf(__VA_ARGS__))
#endif
#ifndef ALOGW
#define ALOGW(...) ((void)std::printf(__VA_ARGS__))
#endif
#ifndef ALOGE
#define ALOGE(...) ((void)std::fprintf(stderr, __VA_ARGS__))
#endif
#endif

#endif // CTS_OPENGL_LOG_H

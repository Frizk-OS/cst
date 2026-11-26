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

#include "neon_kernels.hpp"

#include <jni.h>

extern "C" {

JNIEXPORT jboolean JNICALL Java_android_cts_rscpp_RSInitTest_initTest(
    [[maybe_unused]] JNIEnv* env,
    [[maybe_unused]] jclass clazz) {
    return rscpp::neon::init() ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL Java_android_cts_rscpp_RSBlurTest_blurTest(
    JNIEnv* env,
    [[maybe_unused]] jclass clazz,
    jint x,
    jint y,
    jbyteArray input_array,
    jbyteArray output_array,
    jboolean single_channel) {
    if (!input_array || !output_array || x <= 0 || y <= 0) {
        return JNI_FALSE;
    }

    auto* input = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(input_array, nullptr));
    auto* output = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(output_array, nullptr));
    if (!input || !output) {
        if (input) env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
        if (output) env->ReleasePrimitiveArrayCritical(output_array, output, JNI_ABORT);
        return JNI_FALSE;
    }

    rscpp::neon::blur(input, output, x, y, 15, single_channel == JNI_TRUE);

    env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
    env->ReleasePrimitiveArrayCritical(output_array, output, 0);
    return JNI_TRUE;
}

JNIEXPORT jboolean JNICALL Java_android_cts_rscpp_RSConvolveTest_convolveTest(
    JNIEnv* env,
    [[maybe_unused]] jclass clazz,
    jint x,
    jint y,
    jbyteArray input_array,
    jbyteArray output_array,
    jfloatArray coeff_array,
    jboolean is3x3) {
    if (!input_array || !output_array || !coeff_array || x <= 0 || y <= 0) {
        return JNI_FALSE;
    }

    jfloat* coeffs = env->GetFloatArrayElements(coeff_array, nullptr);
    auto* input = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(input_array, nullptr));
    auto* output = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(output_array, nullptr));

    if (!coeffs || !input || !output) {
        if (coeffs) env->ReleaseFloatArrayElements(coeff_array, coeffs, JNI_ABORT);
        if (input) env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
        if (output) env->ReleasePrimitiveArrayCritical(output_array, output, JNI_ABORT);
        return JNI_FALSE;
    }

    rscpp::neon::convolve(input, output, x, y, coeffs, is3x3 == JNI_TRUE);

    env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
    env->ReleasePrimitiveArrayCritical(output_array, output, 0);
    env->ReleaseFloatArrayElements(coeff_array, coeffs, JNI_ABORT);
    return JNI_TRUE;
}

JNIEXPORT jboolean JNICALL Java_android_cts_rscpp_RSLUTTest_lutTest(
    JNIEnv* env,
    [[maybe_unused]] jclass clazz,
    jint x,
    jint y,
    jbyteArray input_array,
    jbyteArray output_array) {
    if (!input_array || !output_array || x <= 0 || y <= 0) {
        return JNI_FALSE;
    }

    auto* input = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(input_array, nullptr));
    auto* output = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(output_array, nullptr));
    if (!input || !output) {
        if (input) env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
        if (output) env->ReleasePrimitiveArrayCritical(output_array, output, JNI_ABORT);
        return JNI_FALSE;
    }

    uint8_t lut_values[256];
    for (int i = 0; i < 256; ++i) {
        lut_values[i] = static_cast<uint8_t>(255 - i);
    }

    rscpp::neon::lut(input, output, x, y, lut_values, lut_values, lut_values, nullptr);

    env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
    env->ReleasePrimitiveArrayCritical(output_array, output, 0);
    return JNI_TRUE;
}

JNIEXPORT jboolean JNICALL Java_android_cts_rscpp_RS3DLUTTest_lutTest(
    JNIEnv* env,
    [[maybe_unused]] jclass clazz,
    jint x,
    jint y,
    jint lut_size,
    jbyteArray input_array,
    jbyteArray cube_array,
    jbyteArray output_array) {
    if (!input_array || !cube_array || !output_array || x <= 0 || y <= 0 || lut_size <= 0) {
        return JNI_FALSE;
    }

    auto* input = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(input_array, nullptr));
    auto* cube = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(cube_array, nullptr));
    auto* output = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(output_array, nullptr));

    if (!input || !cube || !output) {
        if (input) env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
        if (cube) env->ReleasePrimitiveArrayCritical(cube_array, cube, JNI_ABORT);
        if (output) env->ReleasePrimitiveArrayCritical(output_array, output, JNI_ABORT);
        return JNI_FALSE;
    }

    rscpp::neon::lut3d(input, cube, output, x, y, lut_size);

    env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
    env->ReleasePrimitiveArrayCritical(cube_array, cube, JNI_ABORT);
    env->ReleasePrimitiveArrayCritical(output_array, output, 0);
    return JNI_TRUE;
}

JNIEXPORT jboolean JNICALL Java_android_cts_rscpp_RSColorMatrixTest_colorMatrixTest(
    JNIEnv* env,
    [[maybe_unused]] jclass clazz,
    jint x,
    jint y,
    jbyteArray input_array,
    jbyteArray output_array,
    jfloatArray coeff_array,
    jint option_flag) {
    if (!input_array || !output_array || x <= 0 || y <= 0) {
        return JNI_FALSE;
    }

    jfloat* coeffs = coeff_array ? env->GetFloatArrayElements(coeff_array, nullptr) : nullptr;
    auto* input = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(input_array, nullptr));
    auto* output = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(output_array, nullptr));

    if (!input || !output) {
        if (coeffs) env->ReleaseFloatArrayElements(coeff_array, coeffs, JNI_ABORT);
        if (input) env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
        if (output) env->ReleasePrimitiveArrayCritical(output_array, output, JNI_ABORT);
        return JNI_FALSE;
    }

    rscpp::neon::color_matrix(option_flag, input, output, x, y, coeffs);

    env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
    env->ReleasePrimitiveArrayCritical(output_array, output, 0);
    if (coeffs) {
        env->ReleaseFloatArrayElements(coeff_array, coeffs, JNI_ABORT);
    }
    return JNI_TRUE;
}

JNIEXPORT jboolean JNICALL Java_android_cts_rscpp_RSBlendTest_blendTest(
    JNIEnv* env,
    [[maybe_unused]] jclass clazz,
    jint x,
    jint y,
    jbyteArray input_array,
    jbyteArray output_array,
    jint option_flag) {
    if (!input_array || !output_array || x <= 0 || y <= 0) {
        return JNI_FALSE;
    }

    auto* input = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(input_array, nullptr));
    auto* output = static_cast<uint8_t*>(env->GetPrimitiveArrayCritical(output_array, nullptr));

    if (!input || !output) {
        if (input) env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
        if (output) env->ReleasePrimitiveArrayCritical(output_array, output, JNI_ABORT);
        return JNI_FALSE;
    }

    rscpp::neon::blend(option_flag, input, output, x, y);

    env->ReleasePrimitiveArrayCritical(input_array, input, JNI_ABORT);
    env->ReleasePrimitiveArrayCritical(output_array, output, 0);
    return JNI_TRUE;
}

} // extern "C"

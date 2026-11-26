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

#include "Matrix.h"

#include <cmath>
#include <cstdio>
#include <cstring>

#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
#include <arm_neon.h>
#endif

#define LOG_TAG "CTS_OPENGL"
#define LOG_NDEBUG 0

#if __has_include(<utils/Log.h>)
#include "utils/Log.h"
#elif __has_include(<android/log.h>)
#include <android/log.h>
#define ALOGI(...) ((void)__android_log_print(ANDROID_LOG_INFO, LOG_TAG, __VA_ARGS__))
#else
#define ALOGI(...) ((void)std::printf(__VA_ARGS__))
#endif

Matrix::Matrix() {
    identity();
}

Matrix::Matrix(const Matrix& src) {
    loadWith(src);
}

void Matrix::print(const char* label) {
    ALOGI("%c", *label);
    for (int i = 0; i < 4; i++) {
        const float* d = &(mData[i * 4]);
        ALOGI("%f %f %f %f\n", d[0], d[1], d[2], d[3]);
    }
}

bool Matrix::equals(const Matrix& src) {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    uint32x4_t cmp0 = vceqq_f32(vld1q_f32(mData), vld1q_f32(src.mData));
    uint32x4_t cmp1 = vceqq_f32(vld1q_f32(mData + 4), vld1q_f32(src.mData + 4));
    uint32x4_t cmp2 = vceqq_f32(vld1q_f32(mData + 8), vld1q_f32(src.mData + 8));
    uint32x4_t cmp3 = vceqq_f32(vld1q_f32(mData + 12), vld1q_f32(src.mData + 12));
    uint32x4_t all = vandq_u32(vandq_u32(cmp0, cmp1), vandq_u32(cmp2, cmp3));
    uint32x2_t low = vand_u32(vget_low_u32(all), vget_high_u32(all));
    return (vget_lane_u32(low, 0) == 0xFFFFFFFFU) && (vget_lane_u32(low, 1) == 0xFFFFFFFFU);
#else
    for (int i = 0; i < MATRIX_SIZE; i++) {
        if (mData[i] != src.mData[i]) {
            return false;
        }
    }
    return true;
#endif
}

void Matrix::loadWith(const Matrix& src) {
    std::memcpy(mData, src.mData, MATRIX_SIZE * sizeof(float));
}

void Matrix::identity() {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    static constexpr float kIdentity[16] = {
        1.0f, 0.0f, 0.0f, 0.0f,
        0.0f, 1.0f, 0.0f, 0.0f,
        0.0f, 0.0f, 1.0f, 0.0f,
        0.0f, 0.0f, 0.0f, 1.0f
    };
    std::memcpy(mData, kIdentity, sizeof(kIdentity));
#else
    mData[0] = 1.0f;
    mData[1] = 0.0f;
    mData[2] = 0.0f;
    mData[3] = 0.0f;

    mData[4] = 0.0f;
    mData[5] = 1.0f;
    mData[6] = 0.0f;
    mData[7] = 0.0f;

    mData[8] = 0.0f;
    mData[9] = 0.0f;
    mData[10] = 1.0f;
    mData[11] = 0.0f;

    mData[12] = 0.0f;
    mData[13] = 0.0f;
    mData[14] = 0.0f;
    mData[15] = 1.0f;
#endif
}

void Matrix::translate(float x, float y, float z) {
    Matrix m;
    m.mData[12] = x;
    m.mData[13] = y;
    m.mData[14] = z;
    Matrix temp(*this);
    multiply(temp, m);
}

void Matrix::scale(float x, float y, float z) {
    Matrix m;
    m.mData[0] = x;
    m.mData[5] = y;
    m.mData[10] = z;
    Matrix temp(*this);
    multiply(temp, m);
}

void Matrix::rotate(float radians, float x, float y, float z) {
    Matrix* m = newRotate(radians, x, y, z);
    if (m != nullptr) {
        Matrix temp(*this);
        multiply(temp, *m);
        delete m;
    }
}

void Matrix::multiply(const Matrix& l, const Matrix& r) {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    const float* lhs_ptr = l.mData;
    const float* rhs_ptr = r.mData;
    float* dst_ptr = mData;

    __asm__ volatile(
        "vld1.32 {d0-d3}, [%[lhs]]!\n\t"
        "vld1.32 {d4-d7}, [%[lhs]]\n\t"

        "vld1.32 {d8, d9}, [%[rhs]]!\n\t"
        "vmul.f32 q8, q0, d8[0]\n\t"
        "vmla.f32 q8, q1, d8[1]\n\t"
        "vmla.f32 q8, q2, d9[0]\n\t"
        "vmla.f32 q8, q3, d9[1]\n\t"

        "vld1.32 {d10, d11}, [%[rhs]]!\n\t"
        "vmul.f32 q9, q0, d10[0]\n\t"
        "vmla.f32 q9, q1, d10[1]\n\t"
        "vmla.f32 q9, q2, d11[0]\n\t"
        "vmla.f32 q9, q3, d11[1]\n\t"

        "vld1.32 {d12, d13}, [%[rhs]]!\n\t"
        "vmul.f32 q10, q0, d12[0]\n\t"
        "vmla.f32 q10, q1, d12[1]\n\t"
        "vmla.f32 q10, q2, d13[0]\n\t"
        "vmla.f32 q10, q3, d13[1]\n\t"

        "vld1.32 {d14, d15}, [%[rhs]]\n\t"
        "vmul.f32 q11, q0, d14[0]\n\t"
        "vmla.f32 q11, q1, d14[1]\n\t"
        "vmla.f32 q11, q2, d15[0]\n\t"
        "vmla.f32 q11, q3, d15[1]\n\t"

        "vst1.32 {d16-d19}, [%[dst]]!\n\t"
        "vst1.32 {d20-d23}, [%[dst]]\n\t"
        : [lhs] "+r"(lhs_ptr), [rhs] "+r"(rhs_ptr), [dst] "+r"(dst_ptr)
        :
        : "q0", "q1", "q2", "q3", "q4", "q5", "q6", "q7",
          "q8", "q9", "q10", "q11", "memory"
    );
#else
    float const* const lhs = l.mData;
    float const* const rhs = r.mData;
    float temp[16];
    for (int i = 0; i < 4; i++) {
        const int i4 = i * 4;
        float x = 0;
        float y = 0;
        float z = 0;
        float w = 0;

        for (int j = 0; j < 4; j++) {
            const int j4 = j * 4;
            const float e = rhs[i4 + j];
            x += lhs[j4 + 0] * e;
            y += lhs[j4 + 1] * e;
            z += lhs[j4 + 2] * e;
            w += lhs[j4 + 3] * e;
        }

        temp[i4 + 0] = x;
        temp[i4 + 1] = y;
        temp[i4 + 2] = z;
        temp[i4 + 3] = w;
    }
    std::memcpy(mData, temp, sizeof(temp));
#endif
}

Matrix* Matrix::newLookAt(float eyeX, float eyeY, float eyeZ, float centerX,
        float centerY, float centerZ, float upX, float upY, float upZ) {
    Matrix* m = new Matrix();
    if (m != nullptr) {
        float fx = centerX - eyeX;
        float fy = centerY - eyeY;
        float fz = centerZ - eyeZ;

        // Normalize f
        float rlf = 1.0f / std::sqrt(fx * fx + fy * fy + fz * fz);
        fx *= rlf;
        fy *= rlf;
        fz *= rlf;

        // compute s = f x up (cross product)
        float sx = fy * upZ - fz * upY;
        float sy = fz * upX - fx * upZ;
        float sz = fx * upY - fy * upX;

        // and normalize s
        float rls = 1.0f / std::sqrt(sx * sx + sy * sy + sz * sz);
        sx *= rls;
        sy *= rls;
        sz *= rls;

        // compute u = s x f
        float ux = sy * fz - sz * fy;
        float uy = sz * fx - sx * fz;
        float uz = sx * fy - sy * fx;

        float* d = m->mData;
        d[0] = sx;
        d[1] = ux;
        d[2] = -fx;
        d[3] = 0.0f;

        d[4] = sy;
        d[5] = uy;
        d[6] = -fy;
        d[7] = 0.0f;

        d[8] = sz;
        d[9] = uz;
        d[10] = -fz;
        d[11] = 0.0f;

        d[12] = 0.0f;
        d[13] = 0.0f;
        d[14] = 0.0f;
        d[15] = 1.0f;

        m->translate(-eyeX, -eyeY, -eyeZ);
    }
    return m;
}

Matrix* Matrix::newFrustum(float left, float right, float bottom, float top,
        float near, float far) {
    const float r_width = 1.0f / (right - left);
    const float r_height = 1.0f / (top - bottom);
    const float r_depth = 1.0f / (near - far);
    const float x = 2.0f * (near * r_width);
    const float y = 2.0f * (near * r_height);
    const float A = (right + left) * r_width;
    const float B = (top + bottom) * r_height;
    const float C = (far + near) * r_depth;
    const float D = 2.0f * (far * near * r_depth);
    Matrix* m = new Matrix();
    if (m != nullptr) {
        float* d = m->mData;
        d[0] = x;
        d[5] = y;
        d[8] = A;
        d[9] = B;
        d[10] = C;
        d[14] = D;
        d[11] = -1.0f;
        d[1] = 0.0f;
        d[2] = 0.0f;
        d[3] = 0.0f;
        d[4] = 0.0f;
        d[6] = 0.0f;
        d[7] = 0.0f;
        d[12] = 0.0f;
        d[13] = 0.0f;
        d[15] = 0.0f;
    }
    return m;
}

Matrix* Matrix::newTranslate(float x, float y, float z) {
    Matrix* m = new Matrix();
    if (m != nullptr) {
        float* d = m->mData;
        d[12] = x;
        d[13] = y;
        d[14] = z;
    }
    return m;
}

Matrix* Matrix::newScale(float x, float y, float z) {
    Matrix* m = new Matrix();
    if (m != nullptr) {
        float* d = m->mData;
        d[0] = x;
        d[5] = y;
        d[10] = z;
    }
    return m;
}

Matrix* Matrix::newRotate(float radians, float x, float y, float z) {
    Matrix* m = new Matrix();
    if (m != nullptr) {
        float* d = m->mData;
        d[3] = 0;
        d[7] = 0;
        d[11] = 0;
        d[12] = 0;
        d[13] = 0;
        d[14] = 0;
        d[15] = 1;
        float s = std::sin(radians);
        float c = std::cos(radians);
        if (1.0f == x && 0.0f == y && 0.0f == z) {
            d[5] = c;
            d[10] = c;
            d[6] = s;
            d[9] = -s;
            d[1] = 0;
            d[2] = 0;
            d[4] = 0;
            d[8] = 0;
            d[0] = 1;
        } else if (0.0f == x && 1.0f == y && 0.0f == z) {
            d[0] = c;
            d[10] = c;
            d[8] = s;
            d[2] = -s;
            d[1] = 0;
            d[4] = 0;
            d[6] = 0;
            d[9] = 0;
            d[5] = 1;
        } else if (0.0f == x && 0.0f == y && 1.0f == z) {
            d[0] = c;
            d[5] = c;
            d[1] = s;
            d[4] = -s;
            d[2] = 0;
            d[6] = 0;
            d[8] = 0;
            d[9] = 0;
            d[10] = 1;
        } else {
            float len = std::sqrt((x * x) + (y * y) + (z * z));
            if (1.0f != len && len != 0.0f) {
                float recipLen = 1.0f / len;
                x *= recipLen;
                y *= recipLen;
                z *= recipLen;
            }
            float nc = 1.0f - c;
            float xy = x * y;
            float yz = y * z;
            float zx = z * x;
            float xs = x * s;
            float ys = y * s;
            float zs = z * s;
            d[0] = x * x * nc + c;
            d[4] = xy * nc - zs;
            d[8] = zx * nc + ys;
            d[1] = xy * nc + zs;
            d[5] = y * y * nc + c;
            d[9] = yz * nc - xs;
            d[2] = zx * nc - ys;
            d[6] = yz * nc + xs;
            d[10] = z * z * nc + c;
        }
    }
    return m;
}

void Matrix::multiplyVector(float* result, const Matrix& lhs,
        const float* rhs) {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    const float* mat = lhs.mData;
    __asm__ volatile(
        "vld1.32 {d8, d9}, [%[v_in]]\n\t"
        "vld1.32 {d0-d3}, [%[m]]!\n\t"
        "vld1.32 {d4-d7}, [%[m]]\n\t"

        "vmul.f32 q0, q0, q4\n\t"
        "vmul.f32 q1, q1, q4\n\t"
        "vmul.f32 q2, q2, q4\n\t"
        "vmul.f32 q3, q3, q4\n\t"

        "vpadd.f32 d0, d0, d1\n\t"
        "vpadd.f32 d1, d2, d3\n\t"
        "vpadd.f32 d2, d4, d5\n\t"
        "vpadd.f32 d3, d6, d7\n\t"

        "vpadd.f32 d0, d0, d1\n\t"
        "vpadd.f32 d1, d2, d3\n\t"

        "vst1.32 {d0, d1}, [%[v_res]]\n\t"
        : [m] "+r"(mat)
        : [v_in] "r"(rhs), [v_res] "r"(result)
        : "q0", "q1", "q2", "q3", "q4", "memory"
    );
#else
    const float* d = lhs.mData;
    const float x = rhs[0];
    const float y = rhs[1];
    const float z = rhs[2];
    const float w = rhs[3];
    for (int i = 0; i < 4; i++) {
        const int j = i * 4;
        result[i] = d[j + 0] * x + d[j + 1] * y + d[j + 2] * z + d[j + 3] * w;
    }
#endif
}

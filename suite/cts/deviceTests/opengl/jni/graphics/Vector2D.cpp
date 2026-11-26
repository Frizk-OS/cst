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

#include "Vector2D.h"

#include <cmath>
#include <cstdlib>

#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
#include <arm_neon.h>
#endif

Vector2D::Vector2D() : mX(0.0f), mY(0.0f) {}

Vector2D::Vector2D(float x, float y) : mX(x), mY(y) {}

Vector2D Vector2D::copy() {
    return Vector2D(mX, mY);
}

void Vector2D::add(const Vector2D& v) {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    float32x2_t a = vld1_f32(&mX);
    float32x2_t b = vld1_f32(&v.mX);
    vst1_f32(&mX, vadd_f32(a, b));
#else
    mX += v.mX;
    mY += v.mY;
#endif
}

void Vector2D::sub(const Vector2D& v) {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    float32x2_t a = vld1_f32(&mX);
    float32x2_t b = vld1_f32(&v.mX);
    vst1_f32(&mX, vsub_f32(a, b));
#else
    mX -= v.mX;
    mY -= v.mY;
#endif
}

void Vector2D::scale(float s) {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    float32x2_t a = vld1_f32(&mX);
    vst1_f32(&mX, vmul_n_f32(a, s));
#else
    mX *= s;
    mY *= s;
#endif
}

float Vector2D::distance(const Vector2D& v) {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    float32x2_t a = vld1_f32(&mX);
    float32x2_t b = vld1_f32(&v.mX);
    float32x2_t diff = vsub_f32(a, b);
    float32x2_t sq = vmul_f32(diff, diff);
    float d2 = vget_lane_f32(sq, 0) + vget_lane_f32(sq, 1);
    return std::sqrt(d2);
#else
    float dx = mX - v.mX;
    float dy = mY - v.mY;
    return std::sqrt(dx * dx + dy * dy);
#endif
}

void Vector2D::normalize() {
    float m = magnitude();
    if (m > 0.0f) {
        scale(1.0f / m);
    }
}

void Vector2D::limit(float max) {
    if (magnitude() > max) {
        normalize();
        scale(max);
    }
}

void Vector2D::limit(float maxX, float maxY) {
    if (std::abs(mX) > maxX) {
        mX = (mX > 0.0f) ? maxX : -maxX;
    }
    if (std::abs(mY) > maxY) {
        mY = (mY > 0.0f) ? maxY : -maxY;
    }
}

float Vector2D::magnitude() {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    float32x2_t val = vld1_f32(&mX);
    float32x2_t sq = vmul_f32(val, val);
    float m2 = vget_lane_f32(sq, 0) + vget_lane_f32(sq, 1);
    return std::sqrt(m2);
#else
    return std::hypot(mX, mY);
#endif
}

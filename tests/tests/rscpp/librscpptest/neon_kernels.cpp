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

#include <algorithm>
#include <cmath>
#include <cstring>
#include <memory>
#include <vector>

#if defined(__ARM_NEON) || defined(__ARM_NEON__)
#include <arm_neon.h>
#endif

namespace rscpp::neon {

namespace {

[[nodiscard]] constexpr uint8_t clamp_u8(float v) noexcept {
    if (v < 0.0f) return 0;
    if (v > 255.0f) return 255;
    return static_cast<uint8_t>(v + 0.5f);
}

[[nodiscard]] constexpr uint8_t div255(int v) noexcept {
    return static_cast<uint8_t>((v + 127) / 255);
}

} // namespace

bool init() {
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
    uint8_t a[16] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};
    uint8_t b[16] = {10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160};
    uint8_t out[16] = {};

    const uint8_t* pa = a;
    const uint8_t* pb = b;
    uint8_t* po = out;

    __asm__ volatile (
        "vld1.8 {q0}, [%[pa]]\n\t"
        "vld1.8 {q1}, [%[pb]]\n\t"
        "vqadd.u8 q2, q0, q1\n\t"
        "vst1.8 {q2}, [%[po]]\n\t"
        : [po] "+r"(po)
        : [pa] "r"(pa), [pb] "r"(pb)
        : "q0", "q1", "q2", "memory"
    );

    return out[0] == 11 && out[15] == 176;
#else
    return true;
#endif
}

void blend(int option, const uint8_t* src, uint8_t* dst, int width, int height) {
    const size_t total_bytes = static_cast<size_t>(width) * static_cast<size_t>(height) * 4;
    const size_t total_pixels = static_cast<size_t>(width) * static_cast<size_t>(height);

    switch (option) {
    case 0: { // Add: saturated addition
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
        size_t i = 0;
        const uint8_t* s_ptr = src;
        uint8_t* d_ptr = dst;
        for (; i + 16 <= total_bytes; i += 16) {
            __asm__ volatile (
                "vld1.8 {q0}, [%[s]]!\n\t"
                "vld1.8 {q1}, [%[d]]\n\t"
                "vqadd.u8 q2, q1, q0\n\t"
                "vst1.8 {q2}, [%[d]]!\n\t"
                : [s] "+r"(s_ptr), [d] "+r"(d_ptr)
                :
                : "q0", "q1", "q2", "memory"
            );
        }
        for (; i < total_bytes; ++i) {
            int val = static_cast<int>(dst[i]) + static_cast<int>(src[i]);
            dst[i] = static_cast<uint8_t>(val > 255 ? 255 : val);
        }
#else
        for (size_t i = 0; i < total_bytes; ++i) {
            int val = static_cast<int>(dst[i]) + static_cast<int>(src[i]);
            dst[i] = static_cast<uint8_t>(val > 255 ? 255 : val);
        }
#endif
        break;
    }
    case 1: { // Clear
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
        size_t i = 0;
        uint8_t* d_ptr = dst;
        for (; i + 16 <= total_bytes; i += 16) {
            __asm__ volatile (
                "veor q0, q0, q0\n\t"
                "vst1.8 {q0}, [%[d]]!\n\t"
                : [d] "+r"(d_ptr)
                :
                : "q0", "memory"
            );
        }
        for (; i < total_bytes; ++i) {
            dst[i] = 0;
        }
#else
        std::memset(dst, 0, total_bytes);
#endif
        break;
    }
    case 2: // Dst: no-op
        break;
    case 3: { // DstAtop
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int sa = src[idx + 3];
            const int inv_da = 255 - dst[idx + 3];
            dst[idx + 0] = div255(sa * dst[idx + 0] + inv_da * src[idx + 0]);
            dst[idx + 1] = div255(sa * dst[idx + 1] + inv_da * src[idx + 1]);
            dst[idx + 2] = div255(sa * dst[idx + 2] + inv_da * src[idx + 2]);
            dst[idx + 3] = static_cast<uint8_t>(sa);
        }
        break;
    }
    case 4: { // DstIn
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int sa = src[idx + 3];
            dst[idx + 0] = div255(dst[idx + 0] * sa);
            dst[idx + 1] = div255(dst[idx + 1] * sa);
            dst[idx + 2] = div255(dst[idx + 2] * sa);
            dst[idx + 3] = div255(dst[idx + 3] * sa);
        }
        break;
    }
    case 5: { // DstOut
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int inv_sa = 255 - src[idx + 3];
            dst[idx + 0] = div255(dst[idx + 0] * inv_sa);
            dst[idx + 1] = div255(dst[idx + 1] * inv_sa);
            dst[idx + 2] = div255(dst[idx + 2] * inv_sa);
            dst[idx + 3] = div255(dst[idx + 3] * inv_sa);
        }
        break;
    }
    case 6: { // DstOver
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int inv_da = 255 - dst[idx + 3];
            dst[idx + 0] = static_cast<uint8_t>(std::min(255, dst[idx + 0] + div255(src[idx + 0] * inv_da)));
            dst[idx + 1] = static_cast<uint8_t>(std::min(255, dst[idx + 1] + div255(src[idx + 1] * inv_da)));
            dst[idx + 2] = static_cast<uint8_t>(std::min(255, dst[idx + 2] + div255(src[idx + 2] * inv_da)));
            dst[idx + 3] = static_cast<uint8_t>(std::min(255, dst[idx + 3] + div255(src[idx + 3] * inv_da)));
        }
        break;
    }
    case 7: { // Multiply
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
        size_t i = 0;
        const uint8_t* s_ptr = src;
        uint8_t* d_ptr = dst;
        for (; i + 8 <= total_bytes; i += 8) {
            __asm__ volatile (
                "vld1.8 {d0}, [%[s]]!\n\t"
                "vld1.8 {d1}, [%[d]]\n\t"
                "vmull.u8 q1, d0, d1\n\t"
                "vrshrn.u16 d2, q1, #8\n\t"
                "vst1.8 {d2}, [%[d]]!\n\t"
                : [s] "+r"(s_ptr), [d] "+r"(d_ptr)
                :
                : "d0", "d1", "d2", "q1", "memory"
            );
        }
        for (; i < total_bytes; ++i) {
            dst[i] = div255(dst[i] * src[i]);
        }
#else
        for (size_t i = 0; i < total_bytes; ++i) {
            dst[i] = div255(dst[i] * src[i]);
        }
#endif
        break;
    }
    case 8: { // Src
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
        size_t i = 0;
        const uint8_t* s_ptr = src;
        uint8_t* d_ptr = dst;
        for (; i + 16 <= total_bytes; i += 16) {
            __asm__ volatile (
                "vld1.8 {q0}, [%[s]]!\n\t"
                "vst1.8 {q0}, [%[d]]!\n\t"
                : [s] "+r"(s_ptr), [d] "+r"(d_ptr)
                :
                : "q0", "memory"
            );
        }
        for (; i < total_bytes; ++i) {
            dst[i] = src[i];
        }
#else
        std::memcpy(dst, src, total_bytes);
#endif
        break;
    }
    case 9: { // SrcAtop
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int da = dst[idx + 3];
            const int inv_sa = 255 - src[idx + 3];
            dst[idx + 0] = div255(da * src[idx + 0] + inv_sa * dst[idx + 0]);
            dst[idx + 1] = div255(da * src[idx + 1] + inv_sa * dst[idx + 1]);
            dst[idx + 2] = div255(da * src[idx + 2] + inv_sa * dst[idx + 2]);
        }
        break;
    }
    case 10: { // SrcIn
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int da = dst[idx + 3];
            dst[idx + 0] = div255(src[idx + 0] * da);
            dst[idx + 1] = div255(src[idx + 1] * da);
            dst[idx + 2] = div255(src[idx + 2] * da);
            dst[idx + 3] = div255(src[idx + 3] * da);
        }
        break;
    }
    case 11: { // SrcOut
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int inv_da = 255 - dst[idx + 3];
            dst[idx + 0] = div255(src[idx + 0] * inv_da);
            dst[idx + 1] = div255(src[idx + 1] * inv_da);
            dst[idx + 2] = div255(src[idx + 2] * inv_da);
            dst[idx + 3] = div255(src[idx + 3] * inv_da);
        }
        break;
    }
    case 12: { // SrcOver
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int inv_sa = 255 - src[idx + 3];
            dst[idx + 0] = static_cast<uint8_t>(std::min(255, src[idx + 0] + div255(dst[idx + 0] * inv_sa)));
            dst[idx + 1] = static_cast<uint8_t>(std::min(255, src[idx + 1] + div255(dst[idx + 1] * inv_sa)));
            dst[idx + 2] = static_cast<uint8_t>(std::min(255, src[idx + 2] + div255(dst[idx + 2] * inv_sa)));
            dst[idx + 3] = static_cast<uint8_t>(std::min(255, src[idx + 3] + div255(dst[idx + 3] * inv_sa)));
        }
        break;
    }
    case 13: { // Subtract: saturated subtraction
#if (defined(__ARM_NEON) || defined(__ARM_NEON__)) && defined(__arm__)
        size_t i = 0;
        const uint8_t* s_ptr = src;
        uint8_t* d_ptr = dst;
        for (; i + 16 <= total_bytes; i += 16) {
            __asm__ volatile (
                "vld1.8 {q0}, [%[s]]!\n\t"
                "vld1.8 {q1}, [%[d]]\n\t"
                "vqsub.u8 q2, q1, q0\n\t"
                "vst1.8 {q2}, [%[d]]!\n\t"
                : [s] "+r"(s_ptr), [d] "+r"(d_ptr)
                :
                : "q0", "q1", "q2", "memory"
            );
        }
        for (; i < total_bytes; ++i) {
            int val = static_cast<int>(dst[i]) - static_cast<int>(src[i]);
            dst[i] = static_cast<uint8_t>(val < 0 ? 0 : val);
        }
#else
        for (size_t i = 0; i < total_bytes; ++i) {
            int val = static_cast<int>(dst[i]) - static_cast<int>(src[i]);
            dst[i] = static_cast<uint8_t>(val < 0 ? 0 : val);
        }
#endif
        break;
    }
    case 14: { // Xor
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const int inv_da = 255 - dst[idx + 3];
            const int inv_sa = 255 - src[idx + 3];
            dst[idx + 0] = div255(src[idx + 0] * inv_da + dst[idx + 0] * inv_sa);
            dst[idx + 1] = div255(src[idx + 1] * inv_da + dst[idx + 1] * inv_sa);
            dst[idx + 2] = div255(src[idx + 2] * inv_da + dst[idx + 2] * inv_sa);
            dst[idx + 3] = div255(src[idx + 3] * inv_da + dst[idx + 3] * inv_sa);
        }
        break;
    }
    default:
        break;
    }
}

void color_matrix(int option, const uint8_t* src, uint8_t* dst, int width, int height,
                  const float* coeffs) {
    const size_t total_pixels = static_cast<size_t>(width) * static_cast<size_t>(height);

    switch (option) {
    case 0: { // 3x3 matrix on RGB
        const float m0 = coeffs[0], m1 = coeffs[1], m2 = coeffs[2];
        const float m3 = coeffs[3], m4 = coeffs[4], m5 = coeffs[5];
        const float m6 = coeffs[6], m7 = coeffs[7], m8 = coeffs[8];

        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const float r = src[idx + 0];
            const float g = src[idx + 1];
            const float b = src[idx + 2];
            dst[idx + 0] = clamp_u8(m0 * r + m3 * g + m6 * b);
            dst[idx + 1] = clamp_u8(m1 * r + m4 * g + m7 * b);
            dst[idx + 2] = clamp_u8(m2 * r + m5 * g + m8 * b);
            dst[idx + 3] = src[idx + 3];
        }
        break;
    }
    case 1: { // Greyscale
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const float y = 0.299f * src[idx + 0] + 0.587f * src[idx + 1] + 0.114f * src[idx + 2];
            const uint8_t y_u8 = clamp_u8(y);
            dst[idx + 0] = y_u8;
            dst[idx + 1] = y_u8;
            dst[idx + 2] = y_u8;
            dst[idx + 3] = src[idx + 3];
        }
        break;
    }
    case 2: // 4x4 matrix
    case 5: { // 4x4 matrix with Add
        const float m0 = coeffs[0],  m1 = coeffs[1],  m2 = coeffs[2],  m3 = coeffs[3];
        const float m4 = coeffs[4],  m5 = coeffs[5],  m6 = coeffs[6],  m7 = coeffs[7];
        const float m8 = coeffs[8],  m9 = coeffs[9],  m10 = coeffs[10], m11 = coeffs[11];
        const float m12 = coeffs[12], m13 = coeffs[13], m14 = coeffs[14], m15 = coeffs[15];
        const float a0 = (option == 5) ? 5.3f : 0.0f;
        const float a1 = (option == 5) ? 2.1f : 0.0f;
        const float a2 = (option == 5) ? 0.3f : 0.0f;
        const float a3 = (option == 5) ? 4.4f : 0.0f;

        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const float r = src[idx + 0];
            const float g = src[idx + 1];
            const float b = src[idx + 2];
            const float a = src[idx + 3];
            dst[idx + 0] = clamp_u8(m0 * r + m4 * g + m8 * b + m12 * a + a0);
            dst[idx + 1] = clamp_u8(m1 * r + m5 * g + m9 * b + m13 * a + a1);
            dst[idx + 2] = clamp_u8(m2 * r + m6 * g + m10 * b + m14 * a + a2);
            dst[idx + 3] = clamp_u8(m3 * r + m7 * g + m11 * b + m15 * a + a3);
        }
        break;
    }
    case 3: { // YUV to RGB
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const float y = src[idx + 0];
            const float u = src[idx + 1] - 128.0f;
            const float v = src[idx + 2] - 128.0f;
            dst[idx + 0] = clamp_u8(y + 1.402f * v);
            dst[idx + 1] = clamp_u8(y - 0.344136f * u - 0.714136f * v);
            dst[idx + 2] = clamp_u8(y + 1.772f * u);
            dst[idx + 3] = src[idx + 3];
        }
        break;
    }
    case 4: { // RGB to YUV
        for (size_t i = 0; i < total_pixels; ++i) {
            const size_t idx = i * 4;
            const float r = src[idx + 0];
            const float g = src[idx + 1];
            const float b = src[idx + 2];
            dst[idx + 0] = clamp_u8(0.299f * r + 0.587f * g + 0.114f * b);
            dst[idx + 1] = clamp_u8(-0.168736f * r - 0.331264f * g + 0.5f * b + 128.0f);
            dst[idx + 2] = clamp_u8(0.5f * r - 0.418688f * g - 0.081312f * b + 128.0f);
            dst[idx + 3] = src[idx + 3];
        }
        break;
    }
    default:
        break;
    }
}

void convolve(const uint8_t* src, uint8_t* dst, int width, int height,
              const float* coeffs, bool is3x3) {
    const int radius = is3x3 ? 1 : 2;

    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            float sum = 0.0f;
            int kidx = 0;
            for (int dy = -radius; dy <= radius; ++dy) {
                const int py = std::clamp(y + dy, 0, height - 1);
                for (int dx = -radius; dx <= radius; ++dx) {
                    const int px = std::clamp(x + dx, 0, width - 1);
                    sum += coeffs[kidx++] * static_cast<float>(src[py * width + px]);
                }
            }
            dst[y * width + x] = clamp_u8(sum);
        }
    }
}

void blur(const uint8_t* src, uint8_t* dst, int width, int height,
          int radius, bool single_channel) {
    if (radius <= 0) {
        const size_t bytes = static_cast<size_t>(width) * height * (single_channel ? 1 : 4);
        std::memcpy(dst, src, bytes);
        return;
    }

    const int channels = single_channel ? 1 : 4;
    std::vector<float> kernel(radius * 2 + 1);
    const float sigma = radius * 0.57735f + 0.3333f;
    const float two_sigma_sq = 2.0f * sigma * sigma;
    float sum = 0.0f;

    for (int i = -radius; i <= radius; ++i) {
        const float w = std::exp(-static_cast<float>(i * i) / two_sigma_sq);
        kernel[i + radius] = w;
        sum += w;
    }
    for (float& w : kernel) {
        w /= sum;
    }

    std::vector<float> temp(static_cast<size_t>(width) * height * channels);

    // Horizontal pass
    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            for (int c = 0; c < channels; ++c) {
                float val = 0.0f;
                for (int dx = -radius; dx <= radius; ++dx) {
                    const int px = std::clamp(x + dx, 0, width - 1);
                    val += kernel[dx + radius] * src[(y * width + px) * channels + c];
                }
                temp[(y * width + x) * channels + c] = val;
            }
        }
    }

    // Vertical pass
    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            for (int c = 0; c < channels; ++c) {
                float val = 0.0f;
                for (int dy = -radius; dy <= radius; ++dy) {
                    const int py = std::clamp(y + dy, 0, height - 1);
                    val += kernel[dy + radius] * temp[(py * width + x) * channels + c];
                }
                dst[(y * width + x) * channels + c] = clamp_u8(val);
            }
        }
    }
}

void lut(const uint8_t* src, uint8_t* dst, int width, int height,
         const uint8_t* lut_r, const uint8_t* lut_g, const uint8_t* lut_b, const uint8_t* lut_a) {
    const size_t total_pixels = static_cast<size_t>(width) * static_cast<size_t>(height);
    for (size_t i = 0; i < total_pixels; ++i) {
        const size_t idx = i * 4;
        dst[idx + 0] = lut_r ? lut_r[src[idx + 0]] : src[idx + 0];
        dst[idx + 1] = lut_g ? lut_g[src[idx + 1]] : src[idx + 1];
        dst[idx + 2] = lut_b ? lut_b[src[idx + 2]] : src[idx + 2];
        dst[idx + 3] = lut_a ? lut_a[src[idx + 3]] : src[idx + 3];
    }
}

void lut3d(const uint8_t* src, const uint8_t* cube, uint8_t* dst,
           int width, int height, int lut_size) {
    const size_t total_pixels = static_cast<size_t>(width) * static_cast<size_t>(height);
    const float scale = static_cast<float>(lut_size - 1) / 255.0f;

    for (size_t i = 0; i < total_pixels; ++i) {
        const size_t idx = i * 4;
        const float r = src[idx + 0] * scale;
        const float g = src[idx + 1] * scale;
        const float b = src[idx + 2] * scale;

        const int r0 = std::clamp(static_cast<int>(r), 0, lut_size - 1);
        const int g0 = std::clamp(static_cast<int>(g), 0, lut_size - 1);
        const int b0 = std::clamp(static_cast<int>(b), 0, lut_size - 1);
        const int r1 = std::min(r0 + 1, lut_size - 1);
        const int g1 = std::min(g0 + 1, lut_size - 1);
        const int b1 = std::min(b0 + 1, lut_size - 1);

        const float dr = r - r0;
        const float dg = g - g0;
        const float db = b - b0;

        auto get_pixel = [&](int rx, int gy, int bz, int c) -> float {
            const size_t c_idx = ((static_cast<size_t>(bz) * lut_size + gy) * lut_size + rx) * 4 + c;
            return static_cast<float>(cube[c_idx]);
        };

        for (int c = 0; c < 3; ++c) {
            const float c000 = get_pixel(r0, g0, b0, c);
            const float c100 = get_pixel(r1, g0, b0, c);
            const float c010 = get_pixel(r0, g1, b0, c);
            const float c110 = get_pixel(r1, g1, b0, c);
            const float c001 = get_pixel(r0, g0, b1, c);
            const float c101 = get_pixel(r1, g0, b1, c);
            const float c011 = get_pixel(r0, g1, b1, c);
            const float c111 = get_pixel(r1, g1, b1, c);

            const float c00 = c000 * (1.0f - dr) + c100 * dr;
            const float c10 = c010 * (1.0f - dr) + c110 * dr;
            const float c01 = c001 * (1.0f - dr) + c101 * dr;
            const float c11 = c011 * (1.0f - dr) + c111 * dr;

            const float c0 = c00 * (1.0f - dg) + c10 * dg;
            const float c1 = c01 * (1.0f - dg) + c11 * dg;

            const float val = c0 * (1.0f - db) + c1 * db;
            dst[idx + c] = clamp_u8(val);
        }
        dst[idx + 3] = src[idx + 3];
    }
}

} // namespace rscpp::neon

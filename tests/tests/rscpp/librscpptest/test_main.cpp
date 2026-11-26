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

#include <cassert>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <vector>

#if defined(__ANDROID__)
extern "C" {
__attribute__((visibility("default"))) void android_set_abort_message([[maybe_unused]] const char* msg) {}
}
#endif

void test_init() {
    assert(rscpp::neon::init());
    std::puts("[PASS] init");
}

void test_blend() {
    const int w = 4;
    const int h = 4;
    const size_t total = static_cast<size_t>(w) * h * 4;

    std::vector<uint8_t> src(total, 50);
    std::vector<uint8_t> dst(total, 100);

    // Option 0: Add
    rscpp::neon::blend(0, src.data(), dst.data(), w, h);
    for (size_t i = 0; i < total; ++i) {
        assert(dst[i] == 150);
    }

    // Option 13: Subtract
    rscpp::neon::blend(13, src.data(), dst.data(), w, h);
    for (size_t i = 0; i < total; ++i) {
        assert(dst[i] == 100);
    }

    // Option 8: Src
    std::vector<uint8_t> src2(total, 222);
    rscpp::neon::blend(8, src2.data(), dst.data(), w, h);
    for (size_t i = 0; i < total; ++i) {
        assert(dst[i] == 222);
    }

    // Option 1: Clear
    rscpp::neon::blend(1, src.data(), dst.data(), w, h);
    for (size_t i = 0; i < total; ++i) {
        assert(dst[i] == 0);
    }

    std::puts("[PASS] blend");
}

void test_color_matrix() {
    const int w = 2;
    const int h = 2;
    const size_t total = static_cast<size_t>(w) * h * 4;

    std::vector<uint8_t> src(total);
    for (size_t i = 0; i < total; i += 4) {
        src[i + 0] = 100; // R
        src[i + 1] = 150; // G
        src[i + 2] = 200; // B
        src[i + 3] = 255; // A
    }
    std::vector<uint8_t> dst(total, 0);

    // Greyscale
    rscpp::neon::color_matrix(1, src.data(), dst.data(), w, h, nullptr);
    for (size_t i = 0; i < total; i += 4) {
        uint8_t expected = static_cast<uint8_t>(0.299f * 100 + 0.587f * 150 + 0.114f * 200 + 0.5f);
        assert(dst[i + 0] == expected);
        assert(dst[i + 1] == expected);
        assert(dst[i + 2] == expected);
        assert(dst[i + 3] == 255);
    }

    std::puts("[PASS] color_matrix");
}

void test_convolve() {
    const int w = 4;
    const int h = 4;
    std::vector<uint8_t> src(static_cast<size_t>(w) * h, 10);
    std::vector<uint8_t> dst(static_cast<size_t>(w) * h, 0);

    float coeffs3x3[9] = {
        0.0f, 0.0f, 0.0f,
        0.0f, 1.0f, 0.0f,
        0.0f, 0.0f, 0.0f
    };
    rscpp::neon::convolve(src.data(), dst.data(), w, h, coeffs3x3, true);
    for (size_t i = 0; i < static_cast<size_t>(w) * h; ++i) {
        assert(dst[i] == 10);
    }

    std::puts("[PASS] convolve");
}

void test_lut() {
    const int w = 4;
    const int h = 4;
    const size_t total = static_cast<size_t>(w) * h * 4;

    std::vector<uint8_t> src(total);
    for (size_t i = 0; i < total; ++i) {
        src[i] = static_cast<uint8_t>(i % 256);
    }
    std::vector<uint8_t> dst(total, 0);

    uint8_t lut_inv[256];
    for (int i = 0; i < 256; ++i) {
        lut_inv[i] = static_cast<uint8_t>(255 - i);
    }

    rscpp::neon::lut(src.data(), dst.data(), w, h, lut_inv, lut_inv, lut_inv, nullptr);
    for (size_t i = 0; i < total; i += 4) {
        assert(dst[i + 0] == static_cast<uint8_t>(255 - src[i + 0]));
        assert(dst[i + 1] == static_cast<uint8_t>(255 - src[i + 1]));
        assert(dst[i + 2] == static_cast<uint8_t>(255 - src[i + 2]));
        assert(dst[i + 3] == src[i + 3]);
    }

    std::puts("[PASS] lut");
}

int main() {
    test_init();
    test_blend();
    test_color_matrix();
    test_convolve();
    test_lut();
    std::puts("All RenderScript C++23 NEON unit tests passed successfully!");
    return 0;
}

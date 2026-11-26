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

#pragma once

#include <cstddef>
#include <cstdint>

namespace rscpp::neon {

bool init();

void blend(int option, const uint8_t* src, uint8_t* dst, int width, int height);

void color_matrix(int option, const uint8_t* src, uint8_t* dst, int width, int height,
                  const float* coeffs);

void convolve(const uint8_t* src, uint8_t* dst, int width, int height,
              const float* coeffs, bool is3x3);

void blur(const uint8_t* src, uint8_t* dst, int width, int height,
          int radius, bool single_channel);

void lut(const uint8_t* src, uint8_t* dst, int width, int height,
         const uint8_t* lut_r, const uint8_t* lut_g, const uint8_t* lut_b, const uint8_t* lut_a);

void lut3d(const uint8_t* src, const uint8_t* cube, uint8_t* dst,
           int width, int height, int lut_size);

} // namespace rscpp::neon

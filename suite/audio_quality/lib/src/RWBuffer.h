/*
 * Copyright (C) 2012 The Android Open Source Project
 * Copyright (C) 2026 The AOSP and FrizkOS.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not
 * use this file except in compliance with the License. You may obtain a copy of
 * the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
 * License for the specific language governing permissions and limitations under
 * the License.
 */


#ifndef CTSAUDIO_RWBUFFER_H
#define CTSAUDIO_RWBUFFER_H

#include <cstdint>
#include <cstring>
#include <vector>
#include <string>
#include "Log.h"

/// utility for R/W buffer
class RWBuffer {
public:
    explicit RWBuffer(int capacity)
        : mCapacity(capacity),
          mWrPoint(0),
          mRdPoint(0) {
        mBuffer.resize(static_cast<size_t>(capacity));
    }

    ~RWBuffer() = default;

    void reset() {
        mWrPoint = 0;
        mRdPoint = 0;
    }

    void resetWr() {
        mWrPoint = 0;
    }

    void resetRd() {
        mRdPoint = 0;
    }

    [[nodiscard]] const char* getBuffer() const {
        return mBuffer.data();
    }
    [[nodiscard]] char* getUnwrittenBuffer() {
        return mBuffer.data() + mWrPoint;
    }

    inline void assertWriteCapacity(int sizeToWrite) {
        ASSERT((mWrPoint + sizeToWrite) <= mCapacity);
    }
    void increaseWritten(int size) {
        assertWriteCapacity(0); // damage already done, but detect and panic if happened
        mWrPoint += size;
    }

    [[nodiscard]] int getSizeWritten() const {
        return mWrPoint;
    }

    [[nodiscard]] int getSizeRead() const {
        return mRdPoint;
    }

    template <typename T> void write(T v) {
        assertWriteCapacity(sizeof(T));
        std::memcpy(mBuffer.data() + mWrPoint, &v, sizeof(T));
        mWrPoint += sizeof(T);
    }
    void writeStr(const std::string& str) {
        size_t len = str.length();
        assertWriteCapacity(static_cast<int>(len));
        std::memcpy(mBuffer.data() + mWrPoint, str.data(), len);
        mWrPoint += len;
    }
    template <typename T> [[nodiscard]] T read() {
        T v;
        ASSERT((mRdPoint + sizeof(T)) <= static_cast<size_t>(mWrPoint));
        std::memcpy(&v, mBuffer.data() + mRdPoint, sizeof(T));
        mRdPoint += sizeof(T);
        return v;
    }

private:
    int mCapacity;
    int mWrPoint;
    int mRdPoint;
    std::vector<char> mBuffer;
};


#endif // CTSAUDIO_RWBUFFER_H

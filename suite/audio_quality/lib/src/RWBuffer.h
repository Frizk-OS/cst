/*
 * Copyright (C) 2012 The Android Open Source Project
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
    RWBuffer(int capacity)
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

    const char* getBuffer() {
        return mBuffer.data();
    }
    char* getUnwrittenBuffer() {
        return mBuffer.data() + mWrPoint;
    }

    inline void assertWriteCapacity(int sizeToWrite) {
        ASSERT((mWrPoint + sizeToWrite) <= mCapacity);
    }
    void increaseWritten(int size) {
        assertWriteCapacity(0); // damage already done, but detect and panic if happened
        mWrPoint += size;
    }

    int getSizeWritten() {
        return mWrPoint;
    }

    int getSizeRead() {
        return mRdPoint;
    }

    template <typename T> void write(T v) {
        char* src = (char*)&v;
        assertWriteCapacity(sizeof(T));
        std::memcpy(mBuffer.data() + mWrPoint, &v, sizeof(T));
        mWrPoint += sizeof(T);
    }
    void writeStr(const std::string& str) {
        size_t len = str.length();
        assertWriteCapacity(len);
        std::memcpy(mBuffer.data() + mWrPoint, str.data(), len);
        mWrPoint += len;
    }
    template <typename T> T read() {
        T v;
        ASSERT((mRdPoint + sizeof(T)) <= mWrPoint);
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

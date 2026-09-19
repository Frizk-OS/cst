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
#include <time.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>

#include "Log.h"
#include "Settings.h"
#include "StringUtil.h"
#include "FileUtil.h"


// This class is used by Log. So we cannot use LOG? macros here.
#define _LOGD_(x...) do { fprintf(stderr, x); fprintf(stderr, "\n"); } while(0)

// reported generated under reports/YYYY_MM_DD_HH_MM_SS dir
const char reportTopDir[] = "reports";
std::string FileUtil::mDirPath;

bool FileUtil::prepare(std::string& dirPath)
{
    if (!mDirPath.empty()) {
        dirPath = mDirPath;
        _LOGD_("mDirPath %s", mDirPath.c_str());
        return true;
    }

    time_t timeNow = time(NULL);
    if (timeNow == ((time_t)-1)) {
        _LOGD_("time error");
       return false;
    }
    // tm is allocated in static buffer, and should not be freed.
    struct tm* tm = localtime(&timeNow);
    if (tm == NULL) {
        _LOGD_("localtime error");
        return false;
    }
    int result = mkdir(reportTopDir, S_IRWXU);
    if ((result == -1) && (errno != EEXIST)) {
        _LOGD_("mkdir of topdir failed, error %d", errno);
        return false;
    }
    char reportTimeBuf[64];
    if (snprintf(reportTimeBuf, sizeof(reportTimeBuf), "%04d_%02d_%02d_%02d_%02d_%02d", tm->tm_year + 1900,
                tm->tm_mon + 1, tm->tm_mday, tm->tm_hour, tm->tm_min, tm->tm_sec) < 0) {
            return false;
    }
    std::string reportTime(reportTimeBuf);
    Settings::Instance()->addSetting(Settings::EREPORT_TIME, reportTime);
    char pathBuf[1024];
    if (snprintf(pathBuf, sizeof(pathBuf), "%s/%s", reportTopDir, reportTime.c_str()) < 0) {
        return false;
    }
    result = mkdir(pathBuf, S_IRWXU);
    if ((result == -1) && (errno != EEXIST)) {
        _LOGD_("mkdir of report dir failed, error %d", errno);
        return false;
    }
    mDirPath = pathBuf;
    dirPath = mDirPath;

    return true;
}

FileUtil::FileUtil()
    : mBuffer(DEFAULT_BUFFER_SIZE),
      mBufferSize(DEFAULT_BUFFER_SIZE)
{
}

bool FileUtil::init(const char* fileName)
{
    if (fileName == NULL) {
        return true;
    }

    mFile.open(fileName, std::ios::out | std::ios::trunc);
    if (!mFile.is_open()) {
            return false;
        }
    return true;
}

bool FileUtil::doVprintf(bool fileOnly, int logLevel, const char *fmt, va_list ap)
{
    // prevent messed up log in multi-thread env. Still multi-line logs can be messed up.
    std::lock_guard<std::mutex> lock(mWriteLock);
    while (1) {
        int start = 0;
        if (logLevel != -1) {
            mBuffer[0] = static_cast<char>('0' + logLevel);
            mBuffer[1] = '>';
            start = 2;
        }
        va_list attempt;
        va_copy(attempt, ap);
        int size = vsnprintf(mBuffer.data() + start, mBufferSize - start - 2, fmt, attempt);
        va_end(attempt);
        if (size < 0) {
            fprintf(stderr, "FileUtil::vprintf failed");
            return false;
        }
        if (static_cast<size_t>(size + start + 2) > mBufferSize) {
            mBufferSize = static_cast<size_t>(size + start + 2);
            mBuffer.resize(mBufferSize);
            // re-try
            continue;
        }
        size += start;
        mBuffer[size] = '\n';
        size++;
        mBuffer[size] = 0;

        if (!fileOnly) {
            fprintf(stdout, "%s", mBuffer.data());
        }
        if (mFile.is_open()) {
            mFile << mBuffer.data();
        }
        return true;
    }
}

bool FileUtil::doPrintf(const char* fmt, ...)
{
    va_list ap;
    va_start(ap, fmt);
    bool result = doVprintf(false, -1, fmt, ap);
    va_end(ap);
    return result;
}

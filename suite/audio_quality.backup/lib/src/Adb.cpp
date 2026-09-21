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
#include <stdlib.h>
#include <cstdio>

#include <StringUtil.h>
#include "Adb.h"

Adb::Adb(const std::string& device)
    : mDevice(device)
{

}

Adb::~Adb()
{

}

bool Adb::setPortForwarding(int hostPort, int devicePort)
{
    char buf[128];
    if (snprintf(buf, sizeof(buf), "forward tcp:%d tcp:%d", hostPort, devicePort) < 0) {
        return false;
    }
    if (executeCommand(std::string(buf)) != 0) {
        return false;
    }
    return true;
}

bool Adb::launchClient(const std::string& clientBinary, const std::string& component)
{
    char buf[1024];
    if (snprintf(buf, sizeof(buf), "install -r %s", clientBinary.c_str()) < 0) {
        return false;
    }
    if (executeCommand(std::string(buf)) != 0) {
        return false;
    }
    if (snprintf(buf, sizeof(buf), "shell am start -W -n %s", component.c_str()) < 0) {
        return false;
    }
    if (executeCommand(std::string(buf)) != 0) {
        return false;
    }
    return true;
}

/** @param command ADB command except adb -s XYZW */
int Adb::executeCommand(const std::string& command)
{
    char adbBuf[2048];
    if (mDevice.empty()) {
        if (snprintf(adbBuf, sizeof(adbBuf), "adb %s", command.c_str()) < 0) {
            return -1;
        }
    } else {
        if (snprintf(adbBuf, sizeof(adbBuf), "adb -s %s %s", mDevice.c_str(),
                command.c_str()) < 0) {
            return -1;
        }
    }
    return system(adbBuf);
}


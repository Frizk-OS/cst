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

#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>

#include "Log.h"
#include "StringUtil.h"
#include <cstring>

std::vector<std::string>* StringUtil::split(const std::string& str, char delimiter)
{
    auto tokens = new std::vector<std::string>();
    size_t lastTokenEnd = 0;
    for (size_t i = 0; i < str.size(); i++) {
        if (str[i] == delimiter) {
            if ((i - lastTokenEnd) > 0) {
                tokens->push_back(substr(str, lastTokenEnd, i - lastTokenEnd));
            }
            lastTokenEnd = i + 1; // 1 for skipping delimiter
        }
    }
    if (lastTokenEnd < str.size()) {
        tokens->push_back(substr(str, lastTokenEnd, str.size() - lastTokenEnd));
    }
    return tokens;
}

std::string StringUtil::substr(const std::string& str, size_t pos, size_t n)
{
    size_t l = str.size();

    if (pos >= l) {
        return std::string();
    }
    if ((pos + n) > l) {
        n = l - pos;
    }
    return std::string(str.data() + pos, n);
}

int StringUtil::compare(const std::string& str, const char* other)
{
    return strcmp(str.c_str(), other);
}

bool StringUtil::endsWith(const std::string& str, const char* other)
{
    size_t l1 = str.size();
    size_t l2 = strlen(other);
    if (l2 > l1) {
        return false;
    }
    return str.compare(l1 - l2, l2, other) == 0;
}

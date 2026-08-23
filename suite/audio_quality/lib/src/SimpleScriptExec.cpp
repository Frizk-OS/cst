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
#include <stdio.h>

#include "Log.h"
#include "StringUtil.h"

#include "SimpleScriptExec.h"

const char* SimpleScriptExec::PYTHON_PATH = "/usr/bin/python";
const char* PASS_MAGIC_WORD = "___CTS_AUDIO_PASS___";

bool SimpleScriptExec::checkPythonEnv()
{
    std::string script("test_description/conf/check_conf.py");
    std::string param;
    std::string result;
    if (!runScript(script, param, result)) {
        return false;
    }

    std::string rePattern;
    return checkIfPassed(result, rePattern);
}

bool SimpleScriptExec::checkIfPassed(const std::string& str, const std::string& reMatch,
        int nmatch, regmatch_t pmatch[])
{
    std::string match;
    match.append(PASS_MAGIC_WORD);
    match.append(reMatch);
    LOGV("re match %s", match.c_str());
    regex_t re;
    int cflags = REG_EXTENDED;
    if (nmatch == 0) {
        cflags |= REG_NOSUB;
    }
    if (regcomp(&re, match.c_str(), cflags) != 0) {
        LOGE("regcomp failed");
        return false;
    }
    bool result = false;
    if (regexec(&re, str.c_str(), nmatch, pmatch, 0) == 0) {
        // match found. passed
        result = true;
    }
    regfree(&re);
    return result;
}

bool SimpleScriptExec::runScript(const std::string& script, const std::string& param,
        std::string& result)
{
    FILE *fpipe;
    std::string command = std::string(PYTHON_PATH) + " " + script + " " + param;
    const int READ_SIZE = 1024;
    char buffer[READ_SIZE];
    size_t len = 0;

    if ( !(fpipe = (FILE*)popen(command.c_str(),"r")) ) {
        LOGE("cannot execute python");
        return false;
    }
    result.clear();
    while((len = fread(buffer, 1, READ_SIZE, fpipe)) > 0) {
        result.append(buffer, len);
    }
    pclose(fpipe);

    return true;
}




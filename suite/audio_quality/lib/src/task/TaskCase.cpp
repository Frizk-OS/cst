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

#include <sys/types.h>
#include <regex.h>
#include <cstdlib>
#include <cstdio>
#include <memory>
#include <string>
#include <utility>

#include "Log.h"
#include "audio/RemoteAudio.h"
#include "ClientImpl.h"
#include "Report.h"
#include "Settings.h"
#include "StringUtil.h"
#include "task/TaskCase.h"

static const std::string STR_NAME("name");
static const std::string STR_VERSION("version");
static const std::string STR_DESCRIPTION("description");

TaskCase::TaskCase()
    : TaskGeneric(TaskGeneric::ETaskCase),
      mClient(nullptr)
{
    const std::string* list[] = {&STR_NAME, &STR_VERSION, &STR_DESCRIPTION, nullptr};
    registerSupportedStringAttributes(list);
}

TaskCase::~TaskCase()
{
    delete mClient;
}

bool TaskCase::getCaseName(std::string& name) const
{
    if (!findStringAttribute(STR_NAME, name)) {
        LOGW("TaskCase no name");
        return false;
    }
    return true;
}

bool TaskCase::addChild(TaskGeneric* child)
{
    if ((child->getType() != TaskGeneric::ETaskSetup)
            &&  (child->getType() != TaskGeneric::ETaskAction)
            &&  (child->getType() != TaskGeneric::ETaskSave)) {
        LOGE("TestCase::addChild wrong child type %d", child->getType());
        return false;
    }
    return TaskGeneric::addChild(child);
}

template <typename T>
static bool registerGeneric(std::map<std::string, T>& map, const std::string& name, T& data)
{
    auto it = map.find(name);
    if (it != map.end()) {
        LOGV("registerGeneric key %s already registered", name.c_str());
        return false;
    }
    LOGD("registerGeneric registered key %s", name.c_str());
    map[name] = data;
    return true;
}

template <typename T>
static bool findGeneric(const std::map<std::string, T>& map, const std::string& name, T& data)
{
    LOGD("findGeneric key %s", name.c_str());
    auto it = map.find(name);
    if (it == map.end()) {
        return false;
    }
    data = it->second;
    return true;
}

template <typename T>
static bool updateGeneric(std::map<std::string, T>& map, const std::string& name, T& data)
{
    LOGD("updateGeneric key %s", name.c_str());
    auto it = map.find(name);
    if (it == map.end()) {
        return false;
    }
    it->second = data;
    return true;
}

// return all the matches for the given regular expression.
// name string and the data itself is copied.
template <typename T>
static std::unique_ptr<std::list<std::pair<std::string, T>>> findAllGeneric(
        const std::map<std::string, T>& map, const char* re)
{
    regex_t regex;
    if (regcomp(&regex, re, REG_EXTENDED | REG_NOSUB) != 0) {
        LOGE("regcomp failed");
        return nullptr;
    }
    auto list = std::make_unique<std::list<std::pair<std::string, T>>>();
    for (const auto& [key, val] : map) {
        if (regexec(&regex, key.c_str(), 0, nullptr, 0) == 0) {
            list->emplace_back(key, val);
        }
    }
    regfree(&regex);
    return list->empty() ? nullptr : std::move(list);
}


bool TaskCase::registerBuffer(const std::string& orig, std::shared_ptr<Buffer>& buffer)
{
    std::string translated;
    if (!translateVarName(orig, translated)) {
        return false;
    }
    return registerGeneric<std::shared_ptr<Buffer>>(mBufferList, translated, buffer);
}

bool TaskCase::updateBuffer(const std::string& orig, std::shared_ptr<Buffer>& buffer)
{
    std::string translated;
    if (!translateVarName(orig, translated)) {
        return false;
    }
    return updateGeneric<std::shared_ptr<Buffer>>(mBufferList, translated, buffer);
}

std::shared_ptr<Buffer> TaskCase::findBuffer(const std::string& orig)
{
    std::string translated;
    std::shared_ptr<Buffer> result;
    if (!translateVarName(orig, translated)) {
        return result;
    }
    findGeneric<std::shared_ptr<Buffer>>(mBufferList, translated, result);
    return result;
}

std::unique_ptr<std::list<TaskCase::BufferPair>> TaskCase::findAllBuffers(const std::string& re)
{
    std::string translated;
    if (!translateVarName(re, translated)) {
        return nullptr;
    }
    return findAllGeneric<std::shared_ptr<Buffer>>(mBufferList, translated.c_str());
}


bool TaskCase::registerValue(const std::string& orig, Value& val)
{
    std::string translated;
    if (!translateVarName(orig, translated)) {
        return false;
    }
    LOGD("str %p", static_cast<const void*>(translated.c_str()));
    return registerGeneric<Value>(mValueList, translated, val);
}

bool TaskCase::updateValue(const std::string& orig, Value& val)
{
    std::string translated;
    if (!translateVarName(orig, translated)) {
        return false;
    }
    return updateGeneric<Value>(mValueList, translated, val);
}

bool TaskCase::findValue(const std::string& orig, Value& val)
{
    std::string translated;
    if (!translateVarName(orig, translated)) {
        return false;
    }
    return findGeneric<Value>(mValueList, translated, val);
}

std::unique_ptr<std::list<TaskCase::ValuePair>> TaskCase::findAllValues(const std::string& re)
{
    std::string translated;
    if (!translateVarName(re, translated)) {
        return nullptr;
    }
    return findAllGeneric<Value>(mValueList, translated.c_str());
}

bool TaskCase::registerIndex(const std::string& name, int value)
{
    return registerGeneric<int>(mIndexList, name, value);
}

bool TaskCase::updateIndex(const std::string& name, int value)
{
    return updateGeneric<int>(mIndexList, name, value);
}

bool TaskCase::findIndex(const std::string& name, int& val)
{
    return findGeneric<int>(mIndexList, name, val);
}

std::unique_ptr<std::list<TaskCase::IndexPair>> TaskCase::findAllIndices(const std::string& re)
{
    std::string translated;
    if (!translateVarName(re, translated)) {
        return nullptr;
    }
    return findAllGeneric<int>(mIndexList, translated.c_str());
}

bool TaskCase::translateVarName(const std::string& orig, std::string& translated)
{
    const char* src = orig.c_str();
    constexpr int nmatch = 2;
    regmatch_t pmatch[nmatch];
    regex_t re;
    size_t strStart = 0;

    if (regcomp(&re, "[a-z0-9_]*[$]([a-z0-9]+)[_]*", REG_EXTENDED) != 0) {
        LOGE("regcomp failed");
        return false;
    }
    size_t matchStart = 0;
    size_t matchEnd = 0;
    while (regexec(&re, src, nmatch, pmatch, 0) == 0) {
        matchStart = strStart + static_cast<size_t>(pmatch[1].rm_so);
        matchEnd = strStart + static_cast<size_t>(pmatch[1].rm_eo);
        translated.append(StringUtil::substr(orig, strStart, static_cast<size_t>(pmatch[1].rm_so - 1))); //-1 for $
        std::string indexName;
        indexName.append(StringUtil::substr(orig, matchStart, matchEnd - matchStart));
        int val = 0;
        if (!findIndex(indexName, val)) {
            LOGE("TaskCase::translateVarName no index with name %s", indexName.c_str());
            regfree(&re);
            return false;
        }
        translated += std::to_string(val);
        LOGD("match found strStart %zu, matchStart %zu, matchEnd %zu, converted str %s",
                strStart, matchStart, matchEnd, translated.c_str());
        src += pmatch[1].rm_eo;
        strStart += static_cast<size_t>(pmatch[1].rm_eo);
    }
    if (matchEnd < orig.length()) {
        //LOGD("%d %d", matchEnd, orig.length());
        translated.append(StringUtil::substr(orig, matchEnd, orig.length() - matchEnd));
    }
    LOGD("translated str %s to %s", orig.c_str(), translated.c_str());
    regfree(&re);
    return true;
}

std::shared_ptr<RemoteAudio>& TaskCase::getRemoteAudio()
{
    if (mClient == nullptr) {
        mClient = new ClientImpl();
        ASSERT(mClient->init(Settings::Instance()->getSetting(Settings::EADB)));
    }
    return mClient->getAudio();
}

void TaskCase::releaseRemoteAudio()
{
    delete mClient;
    mClient = nullptr;
}

void TaskCase::setDetails(std::string details)
{
    mDetails = std::move(details);
}

const std::string& TaskCase::getDetails() const
{
    return mDetails;
}


TaskGeneric::ExecutionResult TaskCase::run()
{
    std::string name;
    std::string version;
    //LOGI("str %d, %d", strlen(STR_NAME), strlen(STR_VERSION));
    if (!findStringAttribute(STR_NAME, name) || !findStringAttribute(STR_VERSION, version)) {
        LOGW("TaskCase::run no name or version information");
    }
    MSG("== Test case %s version %s started ==", name.c_str(), version.c_str());
    auto i = getChildren().begin();
    auto end = getChildren().end();
    TaskGeneric* setup = *i;
    i++;
    TaskGeneric* action = *i;
    i++;
    TaskGeneric* save = (i == end) ? nullptr : *i;
    if (save == nullptr) {
        LOGW("No save stage in test case");
    }
    bool testPassed = true;
    TaskGeneric::ExecutionResult result = setup->run();
    TaskGeneric::ExecutionResult resultAction(TaskGeneric::EResultOK);
    if (result != TaskGeneric::EResultOK) {
        MSG("== setup stage failed %d ==", result);
        testPassed = false;
    } else {
        resultAction = action->run();
        if (resultAction != TaskGeneric::EResultPass) {
            MSG("== action stage failed %d ==", resultAction);
            testPassed = false;
        }
        // save done even for failure if possible
        if (save != nullptr) {
            result = save->run();
        }
        if (result != TaskGeneric::EResultOK) {
            MSG("== save stage failed %d ==", result);
            testPassed = false;
        }
    }
    if (testPassed) {
        result = TaskGeneric::EResultPass;
        MSG("== Case %s Passed ==", name.c_str());
        Report::Instance()->addCasePassed(this);
    } else {
        if (resultAction != TaskGeneric::EResultOK) {
            result = resultAction;
        }
        MSG("== Case %s Failed ==", name.c_str());
        Report::Instance()->addCaseFailed(this);
    }
    // release remote audio for other cases to use
    releaseRemoteAudio();
    return result;
}

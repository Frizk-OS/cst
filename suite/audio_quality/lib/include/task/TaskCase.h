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


#ifndef CTSAUDIO_TASKCASE_H
#define CTSAUDIO_TASKCASE_H

#include <stdint.h>
#include <map>
#include <list>
#include <utility>
#include <string>
#include <memory>
#include "Log.h"
#include "audio/Buffer.h"
#include "TaskGeneric.h"

class RemoteAudio;
class ClientInterface;

class TaskCase: public TaskGeneric {
public:
    TaskCase();
    virtual ~TaskCase();
    virtual bool addChild(TaskGeneric* child);
    virtual TaskGeneric::ExecutionResult run();

    bool getCaseName(std::string& name) const;

    bool registerBuffer(const std::string& name, std::shared_ptr<Buffer>& buffer);
    // update already existing buffer. Actually the old buffer will be deleted.
    bool updateBuffer(const std::string& name, std::shared_ptr<Buffer>& buffer);
    /// find buffer with given id. sp will be NULL if not found
    std::shared_ptr<Buffer> findBuffer(const std::string& name);
    typedef std::pair<std::string, std::shared_ptr<Buffer> > BufferPair;
    /// find all buffers with given regular expression. returns NULL if not found
    std::unique_ptr<std::list<BufferPair>> findAllBuffers(const std::string& re);

    std::shared_ptr<RemoteAudio>& getRemoteAudio();

    class Value {
    public:
        enum Type {
            ETypeDouble,
            ETypeI64
        };
        inline Value(): mValue(), mType(ETypeDouble) {};
        inline Value(Type type): mValue(), mType(type) {};
        inline Value(double val): mType(ETypeDouble) {
            setDouble(val);
        };
        inline Value(int64_t val): mType(ETypeI64) {
            setInt64(val);
        };
        inline Type getType() {
            return mType;
        };
        inline void setType(Type type) {
            mType = type;
        };
        inline void setDouble(double val) {
            mValue.asDouble = val;
            mType = ETypeDouble;
            //LOGD("Value set %f 0x%x", val, this);
        };
        inline double getDouble() {
            //LOGD("Value get %f 0x%x", mValue[0], this);
            return mValue.asDouble;
        };
        inline void setInt64(int64_t val) {
            mValue.asInt64 = val;
            mType = ETypeI64;
            //LOGD("Value set %lld 0x%x", val, this);
        }
        inline int64_t getInt64() {
            return mValue.asInt64;
        }
        void* getPtr() {
            return &mValue;
        }
        bool operator ==(const Value& b) const {
            return mType == b.mType &&
                    (mType == ETypeDouble ? mValue.asDouble == b.mValue.asDouble
                                           : mValue.asInt64 == b.mValue.asInt64);
        };

    private:
        union ValueStorage {
            constexpr ValueStorage() : asDouble(0.0) {}
            double asDouble;
            int64_t asInt64;
        } mValue;
        Type mType;
    };

    bool registerValue(const std::string& name, Value& val);
    bool updateValue(const std::string& name, Value& val);
    bool findValue(const std::string& name, Value& val);
    typedef std::pair<std::string, Value> ValuePair;
    /// find all Values with given regular expression. returns NULL if not found
    std::unique_ptr<std::list<ValuePair>> findAllValues(const std::string& re);

    bool registerIndex(const std::string& name, int value = -1);
    bool updateIndex(const std::string& name, int value);
    bool findIndex(const std::string& name, int& val);
    typedef std::pair<std::string, int> IndexPair;
    /// find all Indices with given regular expression. returns NULL if not found
    std::unique_ptr<std::list<IndexPair>> findAllIndices(const std::string& re);

    /**
     * Translate variable name like $i into index variable
     * All xxxValue and xxxBuffer calls do translation inside.
     */
    bool translateVarName(const std::string& orig, std::string& translated);

    void setDetails(std::string details);
    const std::string& getDetails() const;
private:
    void releaseRemoteAudio();

private:
    std::map<std::string, std::shared_ptr<Buffer> > mBufferList;
    std::map<std::string, int> mIndexList;
    std::map<std::string, Value> mValueList;
    ClientInterface* mClient;
    std::string mDetails;
};


#endif // CTSAUDIO_TASKCASE_H

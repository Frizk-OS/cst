#pragma once

#include "utils/StrongPointer.h"

namespace android {

struct Message {
    int what = 0;
    int arg1 = 0;
    int arg2 = 0;
    void* obj = nullptr;
};

class MessageHandler {
public:
    virtual ~MessageHandler() = default;
    virtual void handleMessage(const Message& /*message*/) {}
};

class Looper {
public:
    bool sendMessage(const sp<MessageHandler>& /*handler*/, const Message& /*message*/) { return true; }
    bool sendMessageDelayed(const sp<MessageHandler>& /*handler*/, const Message& /*message*/, int /*delayMs*/) { return true; }
};

}  // namespace android

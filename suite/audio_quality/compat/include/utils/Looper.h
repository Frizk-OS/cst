#pragma once

#include "utils/StrongPointer.h"
#include <vector>
#include <queue>
#include <mutex>
#include <poll.h>
#include <unistd.h>
#include <functional>
#include <memory>
#include <cstring>
#include "Looper_impl.h"

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
    // event flags
    static const int ALOOPER_EVENT_INPUT = POLLIN;

    Looper(bool /*allowNonCallbacks*/ = false);
    ~Looper();

    // set Looper instance for current thread
    static void setForThread(const std::shared_ptr<Looper>& looper);

    // add file descriptor to poll; callback should return 0 to continue, 1 to remove fd, other values ignored
    using fd_callback_t = int (*)(int fd, int events, void* data);
    bool addFd(int fd, int ident, int events, fd_callback_t callback, void* data);

    // poll once and dispatch events. timeout in milliseconds
    int pollOnce(int timeoutMs);

    bool sendMessage(const sp<MessageHandler>& handler, const Message& msg);
    bool sendMessageDelayed(const sp<MessageHandler>& /*handler*/, const Message& /*message*/, int /*delayMs*/) { return false; }

private:
    struct FdEntry {
        int fd;
        int ident;
        int events;
        fd_callback_t callback;
        void* data;
    };

    std::vector<FdEntry> mFds;
    std::queue<std::pair<sp<MessageHandler>, Message>> mMsgQueue;
    std::mutex mLock;
    int mWakeRead = -1;
    int mWakeWrite = -1;
};

}  // namespace android

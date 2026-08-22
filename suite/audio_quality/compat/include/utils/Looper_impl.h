// Looper implementation for the minimal shim
#pragma once

#include <sys/eventfd.h>
#include <cstring>
#include <unistd.h>

namespace android {

inline Looper::Looper(bool)
{
    int fds[2];
    if (pipe(fds) == 0) {
        mWakeRead = fds[0];
        mWakeWrite = fds[1];
    } else {
        mWakeRead = -1;
        mWakeWrite = -1;
    }
}

inline Looper::~Looper()
{
    if (mWakeRead >= 0) close(mWakeRead);
    if (mWakeWrite >= 0) close(mWakeWrite);
}

inline void Looper::setForThread(const std::shared_ptr<Looper>& /*looper*/)
{
    // No-op for this shim. Could store thread-local if needed.
}

inline bool Looper::addFd(int fd, int /*ident*/, int events, fd_callback_t callback, void* data)
{
    FdEntry e;
    e.fd = fd;
    e.ident = 0;
    e.events = events;
    e.callback = callback;
    e.data = data;
    std::lock_guard<std::mutex> lk(mLock);
    mFds.push_back(e);
    return true;
}

inline int Looper::pollOnce(int timeoutMs)
{
    std::vector<pollfd> pfds;
    {
        std::lock_guard<std::mutex> lk(mLock);
        if (mWakeRead >= 0) {
            pollfd p;
            p.fd = mWakeRead;
            p.events = POLLIN;
            p.revents = 0;
            pfds.push_back(p);
        }
        for (const auto& e : mFds) {
            pollfd p;
            p.fd = e.fd;
            p.events = e.events;
            p.revents = 0;
            pfds.push_back(p);
        }
    }

    int rc = ::poll(pfds.data(), pfds.size(), timeoutMs);
    if (rc <= 0) return rc;

    // handle wake pipe
    size_t idx = 0;
    if (mWakeRead >= 0) {
        if (pfds[0].revents & POLLIN) {
            char buf[64];
            while (read(mWakeRead, buf, sizeof(buf)) > 0) {}
            // process queued messages
            std::queue<std::pair<std::shared_ptr<MessageHandler>, Message>> q;
            {
                std::lock_guard<std::mutex> lk(mLock);
                q.swap(mMsgQueue);
            }
            while (!q.empty()) {
                auto &p = q.front();
                if (p.first) p.first->handleMessage(p.second);
                q.pop();
            }
        }
        idx = 1;
    }

    // handle fds
    std::lock_guard<std::mutex> lk(mLock);
    for (size_t i = 0; i < mFds.size(); ++i) {
        if ((idx + i) >= pfds.size()) break;
        if (pfds[idx + i].revents) {
            auto cb = mFds[i].callback;
            if (cb) cb(mFds[i].fd, pfds[idx + i].revents, mFds[i].data);
        }
    }
    return rc;
}

inline bool Looper::sendMessage(const std::shared_ptr<MessageHandler>& handler, const Message& msg)
{
    {
        std::lock_guard<std::mutex> lk(mLock);
        mMsgQueue.emplace(handler, msg);
    }
    if (mWakeWrite >= 0) {
        const char b = 1;
        write(mWakeWrite, &b, 1);
    }
    return true;
}

} // namespace android

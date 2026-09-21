#pragma once

#include <condition_variable>
#include <mutex>

namespace android {

class Mutex {
public:
    class Autolock {
    public:
        explicit Autolock(Mutex& mutex) : mMutex(mutex) { mMutex.lock(); }
        ~Autolock() { mMutex.unlock(); }
    private:
        Mutex& mMutex;
    };

    void lock() { mMutex.lock(); }
    void unlock() { mMutex.unlock(); }

private:
    friend class Condition;
    std::mutex mMutex;
};

class Condition {
public:
    void wait(Mutex& mutex) {
        std::unique_lock<std::mutex> lock(mutex.mMutex);
        mCondition.wait(lock);
    }
    template <typename Predicate>
    void wait(Mutex& mutex, Predicate predicate) {
        while (!predicate()) {
            wait(mutex);
        }
    }
    void notify_one() { mCondition.notify_one(); }
    void notify_all() { mCondition.notify_all(); }

private:
    std::condition_variable mCondition;
};

class Thread {
public:
    virtual ~Thread() = default;

    virtual bool run() { return true; }
    virtual void requestExit() { mExitRequested = true; }
    bool join() { return true; }
    bool start() { return true; }

protected:
    bool mExitRequested = false;
};

}  // namespace android

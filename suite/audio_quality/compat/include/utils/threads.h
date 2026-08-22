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

#include <thread>
#include <atomic>

class Thread {
public:
    virtual ~Thread();

    // start the thread; return 0 (NO_ERROR) on success, negative on failure
    virtual int run();

    // request the thread to exit; thread should check mExitRequested
    virtual void requestExit();

    // request exit and wait for thread to finish
    virtual void requestExitAndWait();

    // join the thread (if running)
    virtual void join();

protected:
    // thread entry point calls readyToRun() then repeatedly threadLoop()
    virtual bool threadLoop() { return false; }
    virtual bool readyToRun() { return true; }

    std::atomic<bool> mExitRequested{false};

private:
    void threadEntry();
    std::thread mThread;
};

}  // namespace android

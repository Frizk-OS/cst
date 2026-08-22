#pragma once

#include <memory>

namespace android {

template <typename T>
class sp {
public:
    sp() = default;
    sp(T* ptr) : mPtr(ptr) {}
    sp(const sp&) = default;
    sp(sp&&) noexcept = default;
    sp& operator=(const sp&) = default;
    sp& operator=(sp&&) noexcept = default;
    sp& operator=(T* ptr) {
        mPtr.reset(ptr);
        return *this;
    }

    T* get() const { return mPtr.get(); }
    T* operator->() const { return mPtr.get(); }
    T& operator*() const { return *mPtr; }
    explicit operator bool() const { return static_cast<bool>(mPtr); }
    void clear() { mPtr.reset(); }
    void reset(T* ptr = nullptr) { mPtr.reset(ptr); }

private:
    std::shared_ptr<T> mPtr;
};

class RefBase {
public:
    virtual ~RefBase() = default;
};

}  // namespace android

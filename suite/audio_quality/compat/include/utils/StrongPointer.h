#pragma once

#include <memory>

namespace android {

template <typename T>
using sp = std::shared_ptr<T>;

class RefBase {
public:
    virtual ~RefBase() = default;
};

}  // namespace android

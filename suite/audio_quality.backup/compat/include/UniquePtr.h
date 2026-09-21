#pragma once

#include <cstddef>
#include <memory>

namespace android {

template <typename T>
using sp = std::shared_ptr<T>;

}  // namespace android

template <typename T, typename D = std::default_delete<T>>
using UniquePtr = std::unique_ptr<T, D>;

template <typename T>
struct DefaultDelete : std::default_delete<T> {
};

template <typename T>
struct DefaultDelete<T[]> : std::default_delete<T[]> {
};

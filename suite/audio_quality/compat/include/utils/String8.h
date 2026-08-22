#pragma once

#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <filesystem>
#include <string>

namespace android {

class String8 {
public:
    String8() = default;
    String8(const char* value) : mValue(value ? value : "") {}
    String8(const std::string& value) : mValue(value) {}
    String8(const std::string& value, size_t length) : mValue(value, 0, length) {}
    String8(const char* value, size_t length) : mValue(value ? std::string(value, length) : std::string()) {}

    String8(const String8&) = default;
    String8(String8&&) noexcept = default;
    String8& operator=(const String8&) = default;
    String8& operator=(String8&&) noexcept = default;
    String8& operator=(const char* value) {
        mValue = value ? value : "";
        return *this;
    }
    String8& operator=(const std::string& value) {
        mValue = value;
        return *this;
    }

    char operator[](size_t index) const { return mValue[index]; }
    size_t length() const { return mValue.size(); }
    bool empty() const { return mValue.empty(); }
    void clear() { mValue.clear(); }

    const char* string() const { return mValue.c_str(); }
    const std::string& stdString() const { return mValue; }
    std::string toString() const { return mValue; }

    int append(const char* value) {
        if (!value) {
            return 0;
        }
        mValue += value;
        return 0;
    }
    int append(const String8& value) {
        mValue += value.mValue;
        return 0;
    }
    int appendFormat(const char* fmt, ...) {
        if (!fmt) {
            return -1;
        }
        va_list args;
        va_start(args, fmt);
        char buffer[4096];
        const int written = vsnprintf(buffer, sizeof(buffer), fmt, args);
        va_end(args);
        if (written < 0) {
            return -1;
        }
        mValue += buffer;
        return 0;
    }

    static String8 format(const char* fmt, ...) {
        String8 result;
        if (!fmt) {
            return result;
        }
        va_list args;
        va_start(args, fmt);
        char buffer[4096];
        const int written = vsnprintf(buffer, sizeof(buffer), fmt, args);
        va_end(args);
        if (written >= 0) {
            result.mValue = buffer;
        }
        return result;
    }

    String8 getPathDir() const {
        const std::filesystem::path path(mValue);
        const auto parent = path.parent_path();
        return parent.empty() ? String8() : String8(parent.string().c_str());
    }

    String8 getPathLeaf() const {
        const std::filesystem::path path(mValue);
        const auto leaf = path.filename();
        return leaf.empty() ? String8() : String8(leaf.string().c_str());
    }

    bool startsWith(const char* prefix) const {
        if (!prefix) {
            return false;
        }
        const std::string needle(prefix);
        return mValue.rfind(needle, 0) == 0;
    }

    bool endsWith(const char* suffix) const {
        if (!suffix) {
            return false;
        }
        const std::string needle(suffix);
        if (needle.size() > mValue.size()) {
            return false;
        }
        return mValue.compare(mValue.size() - needle.size(), needle.size(), needle) == 0;
    }

    int compare(const char* other) const {
        return std::strcmp(mValue.c_str(), other ? other : "");
    }

    explicit operator bool() const { return !mValue.empty(); }

    friend bool operator==(const String8& lhs, const String8& rhs) {
        return lhs.mValue == rhs.mValue;
    }
    friend bool operator!=(const String8& lhs, const String8& rhs) {
        return !(lhs == rhs);
    }
    friend bool operator<(const String8& lhs, const String8& rhs) {
        return lhs.mValue < rhs.mValue;
    }

private:
    std::string mValue;
};

}  // namespace android

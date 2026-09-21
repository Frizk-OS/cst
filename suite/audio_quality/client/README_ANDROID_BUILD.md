This folder contains the legacy Android client source for CTS audio quality.

Goal: try to build/adapt the client with a modern Java toolchain (JDK 21) and a modern Android Gradle Plugin.

Prerequisites (on Fedora):
  - JDK 21 installed and available as JAVA_HOME
  - Android SDK command-line tools installed and ANDROID_HOME set
  - Gradle (or use Gradle wrapper)

Quick steps:
  1. Install JDK 21 and Android SDK (sdkmanager). Set environment:
       export JAVA_HOME=/usr/lib/jvm/java-21-openjdk
       export ANDROID_HOME=$HOME/Android/Sdk
       export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$PATH

  2. In this directory, create a Gradle wrapper (if you don't have Gradle):
       gradle wrapper --gradle-version 8.6

  3. Download Android SDK packages required (with sdkmanager):
       sdkmanager "platform-tools" "platforms;android-34" "build-tools;34.0.0"

  4. Build the client (from project root or client/):
       cd client
       ./gradlew assembleDebug

Notes:
  - AGP and Gradle will download dependencies from the network during the first build.
  - If AGP rejects Java 21, try switching JAVA_HOME to jdk 17 or 21 depending on AGP support.
  - This Gradle project is a minimal scaffold to compile the Java sources into an Android APK.

Device compatibility:
  - Galaxy S3 and S3 Mini are ARMv7 32-bit devices; this client has no native
   ARM64 dependency.
  - The APK minimum API is 16 to cover Android 4.1 firmware commonly shipped
   on these devices. Host-side ARM32 profiles are in `../meson/arm32-*.ini`.

If you want, I can:
  - add a Gradle wrapper into client/ (requires network to download Gradle distribution when run),
  - tune AGP version to match your available JDK and Android platform,
  - or convert the client to a pure JVM app targeting Java 21 instead of an Android APK.

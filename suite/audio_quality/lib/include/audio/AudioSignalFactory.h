/*
 * Copyright (C) 2012 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not
 * use this file except in compliance with the License. You may obtain a copy of
 * the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 * WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
 * License for the specific language governing permissions and limitations under
 * the License.
 */


#ifndef CTSAUDIO_AUDIOSIGNALFACTORY_H
#define CTSAUDIO_AUDIOSIGNALFACTORY_H

#include <memory>

#include "AudioHardware.h"
#include "Buffer.h"
/**
 * factory for creating various audio signals
 */
class AudioSignalFactory {
public:
    static std::shared_ptr<Buffer> generateSineWave(AudioHardware::BytesPerSample BPS,
            int maxPositive, AudioHardware::SamplingRate samplingRate, int signalFreq, int samples,
            bool stereo = true);
    static std::shared_ptr<Buffer> generateWhiteNoise(AudioHardware::BytesPerSample BPS,
            int maxPositive, int samples, bool stereo = true);
    static std::shared_ptr<Buffer> generateZeroSound(AudioHardware::BytesPerSample BPS,
            int samples, bool stereo = true);
};


#endif // CTSAUDIO_AUDIOSIGNALFACTORY_H

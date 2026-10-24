#!/usr/bin/env python3

# Copyright (C) 2012 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#       http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import numpy as np
import scipy as sp
import scipy.fftpack as fft
import scipy.linalg as la
import math

def calc_thd(data, signalFrequency, samplingRate, frequencyMargin):
    # only care about magnitude
    fftData = abs(fft.fft(data * np.hanning(len(data))))
    fftData[0] = 0 # ignore DC
    fftLen = len(fftData) // 2
    baseI = int(fftLen * signalFrequency * 2 / samplingRate)
    iMargain = int(baseI * frequencyMargin)
    baseSignalLoc = baseI - iMargain // 2 + \
        int(np.argmax(fftData[baseI - iMargain // 2 : baseI + iMargain // 2]))
    peakLoc = int(np.argmax(fftData[:fftLen]))
    if peakLoc != baseSignalLoc:
        print("**ERROR Wrong peak signal", peakLoc, baseSignalLoc)
        return 1.0
    print(baseI, baseSignalLoc)
    P0 = math.pow(float(la.norm(fftData[int(baseSignalLoc - iMargain // 2) : int(baseSignalLoc + iMargain // 2)])), 2)
    i = baseSignalLoc * 2
    Pothers = 0.0
    while i < fftLen:
        Pothers += math.pow(float(la.norm(fftData[int(i - iMargain // 2) : int(i + iMargain // 2)])), 2)
        i += baseSignalLoc
    print("P0", P0, "Pothers", Pothers)

    return float(Pothers / P0)

# test code
if __name__=="__main__":
    samplingRate = 44100
    durationInSec = 10
    signalFrequency = 1000
    samples = int(float(samplingRate) * float(durationInSec))
    index = np.linspace(0.0, samples, num=samples, endpoint=False)
    time = index / samplingRate
    multiplier = 2.0 * np.pi * signalFrequency / float(samplingRate)
    data = np.sin(index * multiplier)
    thd = calc_thd(data, signalFrequency, samplingRate, 0.02)
    print("THD", thd)

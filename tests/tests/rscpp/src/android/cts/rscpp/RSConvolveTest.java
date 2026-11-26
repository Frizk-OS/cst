/*
 * Copyright (C) 2026 The AOSP and FrizkOS
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package android.cts.rscpp;

public class RSConvolveTest extends RSCppTest {
    static {
        System.loadLibrary("rscpptest_jni");
    }

    private static final int X = 64;
    private static final int Y = 64;

    public static native boolean convolveTest(int X, int Y, byte[] input, byte[] output, float[] coeffs, boolean is3x3);

    public void testConvolve3x3() {
        byte[] input = new byte[X * Y];
        byte[] output = new byte[X * Y];
        float[] coeffs = {0.1f, 0.1f, 0.1f, 0.1f, 0.2f, 0.1f, 0.1f, 0.1f, 0.1f};
        assertTrue(convolveTest(X, Y, input, output, coeffs, true));
    }

    public void testConvolve5x5() {
        byte[] input = new byte[X * Y];
        byte[] output = new byte[X * Y];
        float[] coeffs = new float[25];
        for (int i = 0; i < 25; i++) {
            coeffs[i] = 0.04f;
        }
        assertTrue(convolveTest(X, Y, input, output, coeffs, false));
    }
}

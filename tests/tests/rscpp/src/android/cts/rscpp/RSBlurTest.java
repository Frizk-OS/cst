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

public class RSBlurTest extends RSCppTest {
    static {
        System.loadLibrary("rscpptest_jni");
    }

    private static final int X = 64;
    private static final int Y = 64;

    public static native boolean blurTest(int X, int Y, byte[] input, byte[] output, boolean singleChannel);

    public void testRSBlurOneChannel() {
        byte[] input = new byte[X * Y];
        byte[] output = new byte[X * Y];
        for (int i = 0; i < input.length; i++) {
            input[i] = (byte)(i & 0xFF);
        }
        assertTrue(blurTest(X, Y, input, output, true));
    }

    public void testRSBlurRGBA() {
        byte[] input = new byte[X * Y * 4];
        byte[] output = new byte[X * Y * 4];
        for (int i = 0; i < input.length; i++) {
            input[i] = (byte)(i & 0xFF);
        }
        assertTrue(blurTest(X, Y, input, output, false));
    }
}

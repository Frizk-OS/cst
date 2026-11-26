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

import java.util.Random;

public class RSUtils {
    public static void genRandom(long seed, int max, int factor, int offset, int array[]) {
        Random r = new Random(seed);
        for (int i = 0; i < array.length; i++) {
            array[i] = (r.nextInt(max) * factor + offset);
        }
    }

    public static void genRandom(long seed, int factor, int offset, float array[]) {
        Random r = new Random(seed);
        for (int i = 0; i < array.length; i++) {
            array[i] = r.nextFloat() * factor + offset;
        }
    }
}

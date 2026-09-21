/*
 * Copyright (C) 2026 The AOSP and FrizkOS.
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

package com.android.cts.xmlgenerator;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

final class TestCase implements Comparable<TestCase> {

    private final String name;
    private final List<Test> tests = new ArrayList<>();

    public TestCase(String name) {
        this.name = Objects.requireNonNull(name, "TestCase name must not be null");
    }

    public String getName() {
        return name;
    }

    public void addTest(String testName, int timeout) {
        tests.add(new Test(testName, timeout));
    }

    public Collection<Test> getTests() {
        return Collections.unmodifiableCollection(tests);
    }

    @Override
    public int compareTo(TestCase another) {
        return name.compareTo(another.name);
    }
}

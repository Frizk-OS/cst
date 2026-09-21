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
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

final class TestSuite implements Comparable<TestSuite> {

    private final String name;
    private final Map<String, TestSuite> suites = new LinkedHashMap<>();
    private final List<TestCase> cases = new ArrayList<>();

    public TestSuite(String name) {
        this.name = Objects.requireNonNull(name, "TestSuite name must not be null");
    }

    public String getName() {
        return name;
    }

    public boolean hasSuite(String suiteName) {
        return suites.containsKey(suiteName);
    }

    public TestSuite getSuite(String suiteName) {
        return suites.get(suiteName);
    }

    public void addSuite(TestSuite suite) {
        suites.put(suite.getName(), suite);
    }

    public Collection<TestSuite> getSuites() {
        return Collections.unmodifiableCollection(suites.values());
    }

    public void addCase(TestCase testCase) {
        cases.add(testCase);
    }

    public Collection<TestCase> getCases() {
        return Collections.unmodifiableCollection(cases);
    }

    @Override
    public int compareTo(TestSuite another) {
        return name.compareTo(another.name);
    }
}

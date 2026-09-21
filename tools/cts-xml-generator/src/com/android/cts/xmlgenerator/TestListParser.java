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

import java.io.InputStream;
import java.util.Collection;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.Scanner;

/**
 * Parser of test lists that are in the format of:
 *
 * suite:android.holo.cts
 * case:HoloTest
 * test:testHolo
 * test:testHoloDialog[:timeout_value]
 */
final class TestListParser {

    public Collection<TestSuite> parse(InputStream input) {
        final Map<String, TestSuite> suiteMap = new LinkedHashMap<>();
        TestSuite currentSuite = null;
        TestCase currentCase = null;

        try (var scanner = new Scanner(input)) {
            while (scanner.hasNextLine()) {
                var line = scanner.nextLine().trim();
                if (line.isEmpty() || line.startsWith("#")) {
                    continue;
                }

                var tokens = line.split(":", 3);
                if (tokens.length < 2) {
                    continue;
                }

                var key = tokens[0];
                var value = tokens[1];

                switch (key) {
                    case "suite" -> currentSuite = handleSuite(suiteMap, value);
                    case "case" -> currentCase = handleCase(currentSuite, value);
                    case "test" -> {
                        int timeout = -1;
                        if (tokens.length == 3) {
                            try {
                                timeout = Integer.parseInt(tokens[2]);
                            } catch (NumberFormatException ignored) {
                                timeout = -1;
                            }
                        }
                        handleTest(currentCase, value, timeout);
                    }
                    default -> {
                        // Ignore unknown directives
                    }
                }
            }
        }

        return suiteMap.values();
    }

    private TestSuite handleSuite(Map<String, TestSuite> suiteMap, String fullSuite) {
        var suites = fullSuite.split("\\.");
        TestSuite lastSuite = null;

        for (var name : suites) {
            if (lastSuite != null) {
                if (lastSuite.hasSuite(name)) {
                    lastSuite = lastSuite.getSuite(name);
                } else {
                    var newSuite = new TestSuite(name);
                    lastSuite.addSuite(newSuite);
                    lastSuite = newSuite;
                }
            } else if (suiteMap.containsKey(name)) {
                lastSuite = suiteMap.get(name);
            } else {
                lastSuite = new TestSuite(name);
                suiteMap.put(name, lastSuite);
            }
        }

        return lastSuite;
    }

    private TestCase handleCase(TestSuite suite, String caseName) {
        var testCase = new TestCase(caseName);
        if (suite != null) {
            suite.addCase(testCase);
        }
        return testCase;
    }

    private void handleTest(TestCase testCase, String test, int timeout) {
        if (testCase != null) {
            testCase.addTest(test, timeout);
        }
    }
}

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

import vogar.Expectation;
import vogar.ExpectationStore;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.List;

/**
 * Generator of TestPackage XML files for native tests.
 */
class XmlGenerator {

    private final String appNamespace;
    private final String appPackageName;
    private final String name;
    private final String runner;
    private final String targetBinaryName;
    private final String targetNameSpace;
    private final String jarPath;
    private final String testType;
    private final String outputPath;
    private final ExpectationStore expectations;

    XmlGenerator(ExpectationStore expectations, String appNamespace, String appPackageName,
            String name, String runner, String targetBinaryName, String targetNameSpace,
            String jarPath, String testType, String outputPath) {
        this.appNamespace = appNamespace;
        this.appPackageName = appPackageName;
        this.name = name;
        this.runner = runner;
        this.targetBinaryName = targetBinaryName;
        this.targetNameSpace = targetNameSpace;
        this.jarPath = jarPath;
        this.testType = testType;
        this.outputPath = outputPath;
        this.expectations = expectations;
    }

    public void writePackageXml() throws IOException {
        OutputStream output = System.out;
        if (outputPath != null) {
            output = new FileOutputStream(new File(outputPath));
        }

        try (var writer = new PrintWriter(output, false, StandardCharsets.UTF_8)) {
            writer.println("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
            writeTestPackage(writer);
            writer.flush();
        }
    }

    private void writeTestPackage(PrintWriter writer) {
        writer.append("<TestPackage");
        if (appNamespace != null) {
            writer.append(" appNameSpace=\"").append(appNamespace).append('"');
        }

        writer.append(" appPackageName=\"").append(appPackageName).append('"');
        writer.append(" name=\"").append(name).append('"');

        if (runner != null) {
            writer.append(" runner=\"").append(runner).append('"');
        }

        if (appNamespace != null && targetNameSpace != null && !appNamespace.equals(targetNameSpace)) {
            writer.append(" targetBinaryName=\"").append(targetBinaryName).append('"');
            writer.append(" targetNameSpace=\"").append(targetNameSpace).append('"');
        }

        if (testType != null && !testType.isEmpty()) {
            writer.append(" testType=\"").append(testType).append('"');
        }

        if (jarPath != null) {
            writer.append(" jarPath=\"").append(jarPath).append('"');
        }

        writer.println(" version=\"1.0\">");

        var parser = new TestListParser();
        var suites = parser.parse(System.in);
        var nameCollector = new StringBuilder();
        writeTestSuites(writer, suites, nameCollector);
        writer.println("</TestPackage>");
    }

    private void writeTestSuites(PrintWriter writer, Collection<TestSuite> suites,
            StringBuilder nameCollector) {
        for (var suite : sortCollection(suites)) {
            writer.append("<TestSuite name=\"").append(suite.getName()).println("\">");

            var namePart = suite.getName();
            if (!nameCollector.isEmpty()) {
                namePart = "." + namePart;
            }
            nameCollector.append(namePart);

            writeTestSuites(writer, suite.getSuites(), nameCollector);
            writeTestCases(writer, suite.getCases(), nameCollector);

            nameCollector.delete(nameCollector.length() - namePart.length(), nameCollector.length());
            writer.println("</TestSuite>");
        }
    }

    private void writeTestCases(PrintWriter writer, Collection<TestCase> cases,
            StringBuilder nameCollector) {
        for (var testCase : sortCollection(cases)) {
            var caseName = testCase.getName();
            writer.append("<TestCase name=\"").append(caseName).println("\">");
            nameCollector.append('.').append(caseName);

            writeTests(writer, testCase.getTests(), nameCollector);

            nameCollector.delete(nameCollector.length() - caseName.length() - 1, nameCollector.length());
            writer.println("</TestCase>");
        }
    }

    private void writeTests(PrintWriter writer, Collection<Test> tests,
            StringBuilder nameCollector) {
        for (var test : sortCollection(tests)) {
            nameCollector.append('#').append(test.getName());
            writer.append("<Test name=\"").append(test.getName()).append('"');
            if (isKnownFailure(expectations, nameCollector.toString())) {
                writer.append(" expectation=\"failure\"");
            }
            if (test.getTimeout() >= 0) {
                writer.append(" timeout=\"").append(String.valueOf(test.getTimeout())).append('"');
            }
            writer.println(" />");

            nameCollector.delete(nameCollector.length() - test.getName().length() - 1, nameCollector.length());
        }
    }

    private <E extends Comparable<E>> List<E> sortCollection(Collection<E> col) {
        var list = new ArrayList<>(col);
        Collections.sort(list);
        return list;
    }

    public static boolean isKnownFailure(ExpectationStore expectationStore, String testName) {
        return expectationStore != null && expectationStore.get(testName) != Expectation.SUCCESS;
    }
}

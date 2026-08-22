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

#include <gtest/gtest.h>
#include <StringUtil.h>


class StringUtilTest : public testing::Test {

};

TEST_F(StringUtilTest, compareTest) {
    std::string str("hello");
    ASSERT_TRUE(StringUtil::compare(str, "hello") == 0);
    ASSERT_TRUE(StringUtil::compare(str, "hi") != 0);
}

TEST_F(StringUtilTest, substrTest) {
    std::string str("hello there");

    std::string sub1 = StringUtil::substr(str, 0, 5);
    ASSERT_TRUE(StringUtil::compare(sub1, "hello") == 0);

    std::string sub2 = StringUtil::substr(str, 10, 5);
    ASSERT_TRUE(StringUtil::compare(sub2, "e") == 0);

    std::string sub3 = StringUtil::substr(str, 6, 5);
    ASSERT_TRUE(StringUtil::compare(sub3, "there") == 0);

    std::string sub4 = StringUtil::substr(str, 100, 5);
    ASSERT_TRUE(sub4.length() == 0);
}

TEST_F(StringUtilTest, endsWithTest) {
    std::string str("hello there");
    ASSERT_TRUE(StringUtil::endsWith(str, "there"));
    ASSERT_TRUE(StringUtil::endsWith(str, "hello there"));
    ASSERT_TRUE(!StringUtil::endsWith(str, "not there"));
}

TEST_F(StringUtilTest, splitTest) {
    std::string str("hello:there:break:this:");
    std::vector<std::string>* tokens = StringUtil::split(str, ':');
    ASSERT_TRUE(tokens != NULL);
    ASSERT_TRUE(tokens->size() == 4);
    ASSERT_TRUE(StringUtil::compare(tokens->at(0), "hello") == 0);
    ASSERT_TRUE(StringUtil::compare(tokens->at(1), "there") == 0);
    ASSERT_TRUE(StringUtil::compare(tokens->at(2), "break") == 0);
    ASSERT_TRUE(StringUtil::compare(tokens->at(3), "this") == 0);
    delete tokens;

    std::string str2("::::");
    std::vector<std::string>* tokens2 = StringUtil::split(str2, ':');
    ASSERT_TRUE(tokens2 != NULL);
    ASSERT_TRUE(tokens2->size() == 0);
    delete tokens2;
}




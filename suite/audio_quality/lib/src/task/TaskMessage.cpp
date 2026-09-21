/*
 * Copyright (C) 2012 The Android Open Source Project
 * Copyright (C) 2026 The AOSP and FrizkOS.
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

#include "task/TaskMessage.h"


TaskMessage::TaskMessage()
    : TaskGeneric(TaskGeneric::ETaskMessage)
{}

TaskMessage::~TaskMessage() = default;

TaskGeneric::ExecutionResult TaskMessage::run()
{
    //TODO
    return TaskGeneric::EResultError;
}

bool TaskMessage::parseAttribute([[maybe_unused]] const std::string& name, [[maybe_unused]] const std::string& value)
{
    //TODO
    return false;
}

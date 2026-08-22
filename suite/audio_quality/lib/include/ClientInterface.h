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

#ifndef CTSAUDIO_CLIENTINTERFACE_H
#define CTSAUDIO_CLIENTINTERFACE_H

#include <string>
#include <memory>

#include "audio/RemoteAudio.h"
#include "ClientSocket.h"

class ClientInterface {
public:
    virtual ~ClientInterface() {};
    /**
     * launch client and perform initial connection
     * @param param parameter for connection. It will be device serial number or empty string
     */
    virtual bool init(const std::string& param) = 0;

    virtual ClientSocket& getSocket()  = 0;

    virtual std::shared_ptr<RemoteAudio>& getAudio() = 0;

};


#endif // CTSAUDIO_CLIENTINTERFACE_H

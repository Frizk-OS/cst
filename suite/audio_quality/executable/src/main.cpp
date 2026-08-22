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
#include <getopt.h>
#include <stdio.h>

#include <string>

#include <UniquePtr.h>

#include "GenericFactory.h"
#include "Log.h"
#include "Report.h"
#include "Settings.h"
#include "task/TaskGeneric.h"
#include "task/ModelBuilder.h"
#include <filesystem>

// For flushing report and log before exiting
class CleanupStatics {
public:

    CleanupStatics() {

    }
    ~CleanupStatics() {
        Log::Finalize();
        Report::Finalize();
        // create zip file after log and report files are closed.
            std::string reportFile = Settings::Instance()->getSetting(Settings::EREPORT_FILE);
        std::filesystem::path reportPath(reportFile);
        std::string reportDirPath = reportPath.parent_path().c_str();
        std::string zipFilename = reportPath.filename().c_str();
        std::string command = std::string("cd ") + reportDirPath + "; zip -r ../" + zipFilename + ".zip *";
        fprintf(stderr, "\n\nexecuting %s\n", command.c_str());
        if (system(command.c_str()) == -1) {
            fprintf(stderr, "cannot create zip file with command %s\n", command.c_str());
        }
        Settings::Finalize();
    }
};

void usage(char* bin)
{
    fprintf(stderr, "%s [-l log_level][-s serial] test_xml\n", bin);
}
int main(int argc, char *argv[])
{
    if (argc < 2) {
        fprintf(stderr, "%s [-l log_level][-s serial] test_xml\n", argv[0]);
        return 1;
    }
    int logLevel = Log::ELogW;
    char* serial = NULL;
    int opt;
    while ((opt = getopt(argc, argv, "l:s:")) != -1) {
        switch (opt) {
        case 'l':
            logLevel = atoi(optarg);
            break;
        case 's':
            serial = optarg;
            break;
        default:
            usage(argv[0]);
            return 1;
        }
    }
    if (optind >= argc) {
        usage(argv[0]);
        return 1;
    }

    std::string xmlFile(argv[optind]);

    std::string dirName;
    if (!FileUtil::prepare(dirName)) {
        fprintf(stderr, "cannot prepare report dir");
        return 1;
    }

    UniquePtr<CleanupStatics> staticStuffs(new CleanupStatics());
    if (Settings::Instance() == NULL) {
        fprintf(stderr, "caanot create Settings");
        return 1;
    }
    if (serial != NULL) {
        std::string strSerial(serial);
        Settings::Instance()->addSetting(Settings::EADB, strSerial);
    }
    if (Log::Instance(dirName.c_str()) == NULL) {
        fprintf(stderr, "cannot create Log");
        return 1;
    }
    Log::Instance()->setLogLevel((Log::LogLevel)logLevel);
    // Log can be used from here
    if (Report::Instance(dirName.c_str()) == NULL) {

        LOGE("cannot create log");
        return 1;
    }

    GenericFactory factory;
    ClientInterface* client = factory.createClientInterface();
    if (client == NULL) {
        fprintf(stderr, "cannot create ClientInterface");
        return 1;
    }
    if (!client->init(Settings::Instance()->getSetting(Settings::EADB))) {
        fprintf(stderr, "cannot init ClientInterface");
        return 1;
    }
    std::string deviceInfo;
    if (!client->getAudio()->getDeviceInfo(deviceInfo)) {
        fprintf(stderr, "cannot get device info");
        return 1;
    }
    delete client; // release so that it can be used in tests
    Settings::Instance()->addSetting(Settings::EDEVICE_INFO, deviceInfo);

    ModelBuilder modelBuilder;
    UniquePtr<TaskGeneric> topTask(modelBuilder.parseTestDescriptionXml(xmlFile));
    if (topTask.get() == NULL) {
        LOGE("Parsing of %x failed", xmlFile.c_str());
        return 1;
    }
    Settings::Instance()->addSetting(Settings::ETEST_XML, xmlFile);
    topTask->run();

    return 0;
}


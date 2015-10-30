#include "pd_view.h"
#include "pd_backend.h"
#include <stdlib.h>
#include <stdio.h>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct ThreadsData {
    int selectedThread;
    int threadId;
    bool requestData;
    bool setSelectedThread;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void* createInstance(PDUI* uiFuncs, ServiceFunc* serviceFunc)
{
    (void)serviceFunc;
    (void)uiFuncs;

    ThreadsData* user_data = (ThreadsData*)malloc(sizeof(ThreadsData));

    user_data->selectedThread = 0;
    user_data->threadId = 0;

    return user_data;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroyInstance(void* user_data)
{
    free(user_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void showInUI(ThreadsData* user_data, PDReader* reader, PDUI* uiFuncs)
{
    PDReaderIterator it;
    ThreadsData* data = (ThreadsData*)user_data;

    if (PDRead_findArray(reader, &it, "threads", 0) == PDReadStatus_notFound)
        return;

    uiFuncs->text("");

    uiFuncs->columns(3, "threads", true);
    uiFuncs->text("Id"); uiFuncs->next_column();
    uiFuncs->text("Name"); uiFuncs->next_column();
    uiFuncs->text("Function"); uiFuncs->next_column();

    int i = 0;

    PDVec2 size = { 0.0f, 0.0f };

    data->setSelectedThread = false;

    int oldSelectedThread = data->selectedThread;

    while (PDRead_getNextEntry(reader, &it)) {
        uint64_t id;
        const char* name = "";
        const char* function = "";

        PDRead_findU64(reader, &id, "id", it);
        PDRead_findString(reader, &name, "name", it);
        PDRead_findString(reader, &function, "function", it);

        char label[32];
        sprintf(label, "%llx", id);

        if (uiFuncs->selectable(label, data->selectedThread == i, 1 << 1, size)) {
            data->selectedThread = i;
            data->threadId = (int)id;
        }

        uiFuncs->next_column();
        uiFuncs->text(name); uiFuncs->next_column();
        uiFuncs->text(function); uiFuncs->next_column();

        i++;
    }

    if (oldSelectedThread != data->selectedThread) {
        data->setSelectedThread = true;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int update(void* user_data, PDUI* uiFuncs, PDReader* inEvents, PDWriter* outEvents)
{
    uint32_t event = 0;

    ThreadsData* data = (ThreadsData*)user_data;

    data->requestData = false;
    data->setSelectedThread = false;

    while ((event = PDRead_getEvent(inEvents)) != 0) {
        switch (event) {
            case PDEventType_setThreads:
            {
                showInUI((ThreadsData*)user_data, inEvents, uiFuncs);
                break;
            }

            case PDEventType_setExceptionLocation:
            {
                data->requestData = true;
                break;
            }
        }
    }

    // Request threads data

    if (data->setSelectedThread) {
        PDWrite_eventBegin(outEvents, PDEventType_selectThread);
        printf("writing thread id %d\n", data->threadId);
        PDWrite_u32(outEvents, "thread_id", (uint32_t)data->threadId);
        PDWrite_eventEnd(outEvents);
    }

    if (data->requestData) {
        PDWrite_eventBegin(outEvents, PDEventType_getThreads);
        PDWrite_eventEnd(outEvents);
    }

    return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static PDViewPlugin plugin =
{
    "Threads",
    createInstance,
    destroyInstance,
    update,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

extern "C"
{

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    PD_EXPORT void InitPlugin(RegisterPlugin* registerPlugin, void* private_data)
    {
        registerPlugin(PD_VIEW_API_VERSION, &plugin, private_data);
    }

}


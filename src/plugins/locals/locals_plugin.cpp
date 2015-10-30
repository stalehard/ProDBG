#include "pd_view.h"
#include "pd_backend.h"
#include <stdlib.h>
#include <stdio.h>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct LocalsData
{
    int dummy;
};



///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void* createInstance(PDUI* uiFuncs, ServiceFunc* serviceFunc)
{
    (void)serviceFunc;
    (void)uiFuncs;
    LocalsData* user_data = (LocalsData*)malloc(sizeof(LocalsData));

    return user_data;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroyInstance(void* user_data)
{
    free(user_data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void showInUI(LocalsData* data, PDReader* reader, PDUI* uiFuncs)
{
    PDReaderIterator it;
    (void)data;

    if (PDRead_findArray(reader, &it, "locals", 0) == PDReadStatus_notFound)
        return;

    uiFuncs->text("");

    uiFuncs->columns(3, "callstack", true);
    uiFuncs->text("Name"); uiFuncs->next_column();
    uiFuncs->text("Value"); uiFuncs->next_column();
    uiFuncs->text("Type"); uiFuncs->next_column();

    while (PDRead_getNextEntry(reader, &it))
    {
        const char* name = "";
        const char* value = "";
        const char* type = "";

        PDRead_findString(reader, &name, "name", it);
        PDRead_findString(reader, &value, "value", it);
        PDRead_findString(reader, &type, "type", it);

        uiFuncs->text(name); uiFuncs->next_column();
        uiFuncs->text(value); uiFuncs->next_column();
        uiFuncs->text(type); uiFuncs->next_column();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static int update(void* user_data, PDUI* uiFuncs, PDReader* inEvents, PDWriter* outEvents)
{
    uint32_t event = 0;

    while ((event = PDRead_getEvent(inEvents)) != 0)
    {
        switch (event)
        {
            case PDEventType_setLocals:
            {
                showInUI((LocalsData*)user_data, inEvents, uiFuncs);
                break;
            }
        }
    }

    // Request callstack data
    // TODO: Dont' request locals all the time

    PDWrite_eventBegin(outEvents, PDEventType_getLocals);
    PDWrite_u8(outEvents, "dummy_remove", 0);   // TODO: Remove me
    PDWrite_eventEnd(outEvents);

    return 0;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static PDViewPlugin plugin =
{
    "Locals",
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


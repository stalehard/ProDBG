#ifndef _PDVIEW_H_
#define _PDVIEW_H_

#include "pd_common.h"
#include "pd_readwrite.h"
#include "pd_ui.h"
#include "pd_io.h"

#ifdef _cplusplus
extern "C" {
#endif

struct PDUI;
struct PDUIPainter;
struct PDSaveState;
struct PDLoadState;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#define PD_VIEW_API_VERSION "ProDBG View 1"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct PDViewPlugin
{
    const char* name;

    void* (*createInstance)(PDUI* uiFuncs, ServiceFunc* serviceFunc);
    void (*destroyInstance)(void* user_data);

    // Updates and Returns the current state of the plugin.
    int (*update)(void* user_data, PDUI* uiFuncs, PDReader* inEvents, PDWriter* outEvents);

    // save/load state
	int (*saveState)(void* user_data, struct PDSaveState* saveState);
	int (*loadState)(void* user_data, struct PDLoadState* loadState);

} PDViewPlugin;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#ifdef _cplusplus
}
#endif

#endif

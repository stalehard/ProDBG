#include "api/plugin_instance.h"
#include "core/core.h"
#include "core/log.h"
#include "core/file_monitor.h"
#include "core/math.h"
#include "core/alloc.h"
#include "core/plugin_handler.h"
#include "session/session.h"
#include "settings.h"
#include "core/settings.h"
#include "ui/wx/wx_plugin_ui.h"
#include "ui/bgfx/bgfx_plugin_ui.h"
#include "ui/bgfx/dialogs.h"
#include "ui/bgfx/ui_dock.h"
#include "ui/menu.h"
#include "core/input_state.h"
#include "ui/plugin.h"
#include "i3wm_docking.h"

//#include <bgfx.h>
#include <stdio.h>
#include <stdlib.h>

#include <remotery.h>
#include <assert.h>

#include <foundation/environment.h>
#include <foundation/fs.h>
#include <foundation/string.h>
#include <foundation/path.h>
#include <foundation/time.h>

// TODO: Fix me

int Window_buildPluginMenu(PluginData** plugins, int count);
void Window_addMenu(const char* name, PDMenuItem* items, uint32_t idOffset);

void ProDBG_event(int id);
void ProDBG_setMouseState(int button, int state);
void ProDBG_setMousePos(float x, float y);

// TODO(marco): need to figure out where this goes
bool IMGUI_beginMainMenuBar();
void IMGUI_endMainMenuBar();

bool IMGUI_beginMenu(const char *menuName);
bool IMGUI_menuItem(const char *itemName);
void IMGUI_endMenu();

void IMGUI_openPopup(const char *popupId);
bool IMGUI_beginPopup(const char *popupName);
void IMGUI_endPopup();

bool IMGUI_isItemHovered();

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Context
{
    int width;
    int height;
    uint64_t time;
    Session* session;   // one session right now
    bool popupActive;
    bool popupHover;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static Context s_context;
//Remotery* s_remotery;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static const char* s_plugins[] =
{
    "sourcecode_plugin",
    "disassembly_plugin",
    "locals_plugin",
    "threads_plugin",
    "callstack_plugin",
    "registers_plugin",
    "breakpoints_plugin",
    "hex_memory_plugin",
    "console_plugin",
    "c64_vice_plugin",
#ifdef PRODBG_MAC
    "lldb_plugin",
#endif
#ifdef PRODBG_WIN
    "dbgeng_plugin",
#endif
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void loadLayout(Session* session, int width, int height)
{
    Session_createDockingGrid(session, width, height);

	if (docksys_load_layout("data/current_layout_2.json"))
		return;

	docksys_load_layout("data/default_layout_2.json");

	/*
    if (Session_loadLayout(session, "data/current_layout.json", width, height))
        return;

    if (Session_loadLayout(session, "data/default_layout.json", width, height))
        return;
    */

}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

uint32_t findMenuIdRange(PDMenu* menu)
{
    uint32_t idStart = uint32_t(~0);
    uint32_t idEnd = 0;

    while (menu->name)
    {
        PDMenuItem* menuItems = menu->items;

        while (menuItems->name)
        {
            const uint16_t id = menuItems->id;

            if (id > idEnd)
                idEnd = id;

            if (id < idStart)
                idStart = id;

            menuItems++;
        }

        menu++;
    }

    return (idEnd << 16) | idStart;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void createMenusForPlugins()
{
    int count = 0;

    PluginData** plugins = PluginHandler_getBackendPlugins(&count);

    uint32_t menuIdStart = PRODBG_MENU_PLUGINS_START;

    for (int i = 0; i < count; ++i)
    {
        PluginData* pluginData = plugins[i];

        PDBackendPlugin* plugin = (PDBackendPlugin*)pluginData->plugin;

        if (!plugin)
            continue;

        if (!plugin->registerMenu)
            continue;

        PDMenu* menus = plugin->registerMenu();
        uint32_t menuRange = findMenuIdRange(menus);

        while (menus->name)
        {
            Window_addMenu(menus->name, menus->items, menuIdStart);
            menus++;
        }

        pluginData->menuStart = menuIdStart;
        pluginData->menuEnd = menuIdStart + (menuRange >> 16);

        menuIdStart += (menuRange >> 16);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static bool findDataDirectory()
{
    // check if the data dir is directly here

    if (fs_is_directory("data"))
        return true;

    const char* exe_file_path = environment_executable_path();
    const char* exe_path = path_directory_name(exe_file_path );

    char* path = string_clone(exe_path);

    // search max 10 levels up

    for (int i = 0; i < 10; ++i)
    {
    #if defined(PRODBG_WIN)
        path = string_append(path, "\\..");
    #else
        path = string_append(path, "/..");
    #endif

        environment_set_current_working_directory(path);

        if (fs_is_directory("data"))
            return true;
    }

    return false;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void ProDBG_create(void* window, int width, int height)
{
    Context* context = &s_context;
    //Rect settingsRect;
    //

    Core_init();

    findDataDirectory();

    g_pluginUI = new BgfxPluginUI;
    //g_pluginUI = new WxPluginUI;

	Settings_loadSettings("data/settings.json");

    Session_globalInit(true);
    Settings_registerService();

    context->session = Session_create();
    context->time = time_current();

    g_pluginUI->create(window, width, height);

    loadLayout(context->session, width, height - g_pluginUI->getStatusBarSize());

    (void)window;

    for (uint32_t i = 0; i < sizeof_array(s_plugins); ++i)
    {
        if (!PluginHandler_findPluginByFilename(s_plugins[i]))
            PluginHandler_addPlugin(OBJECT_DIR, s_plugins[i]);
    }


    context->width = width;
    context->height = height;
}


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void IMGUI_menuUpdate(Context *context)
{
    int count = 0;
    PluginData** viewPlugins = PluginHandler_getViewPlugins(&count);
    PDMenuItem* menu = (PDMenuItem*)alloc_zero(sizeof(PDMenuItem) * (count + 1)); // + 1 as array needs to end with zeros
    if (IMGUI_beginMainMenuBar())
    {
	// NOTE(marco): these are static values
	if (IMGUI_beginMenu("File"))
	{
	    PDMenuItem *desc = &g_fileMenu[0];
	    while (desc->name)
	    {
		if (IMGUI_menuItem(desc->name))
		{
		    ProDBG_event(desc->id);
		}
		desc++;
	    }
	    IMGUI_endMenu();
	}

	if (IMGUI_beginMenu("Debug"))
	{
	    PDMenuItem *desc = &g_debugMenu[0];
	    while (desc->name)
	    {
		if (IMGUI_menuItem(desc->name))
		{
		    ProDBG_event(desc->id);
		}
		desc++;
	    }
	    IMGUI_endMenu();
	}

	for (int i = 0; i < count; ++i)
	{
	    PluginData* pluginData = viewPlugins[i];
	    PDPluginBase* pluginBase = (PDPluginBase*)pluginData->plugin;
	    PDMenuItem* entry = &menu[i];

	    // TODO: Hack hack!

	    if (!strstr(pluginData->type, "View"))
		continue;

	    entry->name = pluginBase->name;

	    // TODO: Only shortcuts for the first range but we should really have this in a config instead.

	    if (i < 10)
	    {
		entry->id = PRODBG_MENU_PLUGIN_START + i;
		entry->key = '1' + i;
	    }

	    entry->macMod = PRODBG_KEY_COMMAND;
	    entry->winMod = PRODBG_KEY_CTRL;
	}
	
	uint32_t menuIdStart = PRODBG_MENU_PLUGINS_START;
	if (IMGUI_beginMenu("Plugins"))
	{	    
	    PDMenuItem *desc = &menu[0];
	    while (desc->name)
	    {
		if (IMGUI_menuItem(desc->name))
		{
		    ProDBG_event(desc->id);
		}
		desc++;
	    }
	    IMGUI_endMenu();
	}

	PluginData **backendPlugins = PluginHandler_getBackendPlugins(&count);
	for (int i = 0; i < count; ++i)
	{
	    PluginData* pluginData = backendPlugins[i];

	    PDBackendPlugin* plugin = (PDBackendPlugin*)pluginData->plugin;

	    if (!plugin)
		continue;

	    if (!plugin->registerMenu)
		continue;

	    PDMenu* menus = plugin->registerMenu();
	    uint32_t menuRange = findMenuIdRange(menus);

	    while (menus->name)
	    {
		if (IMGUI_beginMenu(menus->name))
		{
		    PDMenuItem *desc = &menus->items[0];
		    while(desc->name)
		    {
			if (IMGUI_menuItem(desc->name))
			{
			    ProDBG_event(desc->id + menuIdStart);
			}
			desc++;
		    }
		    IMGUI_endMenu();
		}
		menus++;
	    }

	    pluginData->menuStart = menuIdStart;
	    pluginData->menuEnd = menuIdStart + (menuRange >> 16);

	    menuIdStart += (menuRange >> 16);
	}
	
	IMGUI_endMainMenuBar();
    }	
	
#if 1
    InputState *inputState = InputState_getState();
    if (inputState->mouseDown[MouseButton_Right])
    {	
	static float savedMousePosX = 0.0f;
	static float savedMousePosY = 0.0f;
	// NOTE(marco): save the mouse position only the first time we activate
	// the popup
	if (!context->popupActive)
	{
	    savedMousePosX = inputState->mousePos.x;
	    savedMousePosY = inputState->mousePos.y;
	    context->popupActive = true;
	}
	IMGUI_openPopup("context_menu");
	context->popupHover = false;

	// TODO(marco): need to update this to read from new context menu list
	if (IMGUI_beginPopup("context_menu"))
	{
	    const char *splitHorz = "Split Horizontally";
	    if (IMGUI_beginMenu(splitHorz))
	    {
		uint32_t startId = PRODBG_MENU_POPUP_SPLIT_HORZ;
		if (IMGUI_menuItem(splitHorz))
		{
		    float tempX = inputState->mousePos.x;
		    float tempY = inputState->mousePos.y;
		    ProDBG_setMousePos(savedMousePosX, savedMousePosY);
		    ProDBG_event(startId);
		    ProDBG_setMousePos(tempX, tempY);
		    ProDBG_setMouseState(MouseButton_Right, 0);
		}
		if (IMGUI_isItemHovered())
		{
		    context->popupHover = true;
		}
		
		uint32_t idMask = PRODBG_MENU_POPUP_SPLIT_HORZ_SHIFT;
		PDMenuItem *desc = &menu[0];
		int popupMenuCount = 0;
		while (desc->name)
		{
		    if (IMGUI_menuItem(desc->name))
		    {
			float tempX = inputState->mousePos.x;
			float tempY = inputState->mousePos.y;
			ProDBG_setMousePos(savedMousePosX, savedMousePosY);	     
			ProDBG_event((uint32_t)popupMenuCount | idMask);
			ProDBG_setMousePos(tempX, tempY);
			ProDBG_setMouseState(MouseButton_Right, 0);
		    }
		    if (IMGUI_isItemHovered())
		    {
			context->popupHover = true;
		    }
		    ++popupMenuCount;
		    desc++;
		}
		IMGUI_endMenu();
		if (IMGUI_isItemHovered())
		{
		    context->popupHover = true;
		}
	    }

	    const char *splitVert = "Split Vertically";
	    if (IMGUI_beginMenu(splitVert))
	    {
		uint32_t startId = PRODBG_MENU_POPUP_SPLIT_VERT;
		if (IMGUI_menuItem(splitVert))
		{
		    float tempX = inputState->mousePos.x;
		    float tempY = inputState->mousePos.y;
		    ProDBG_setMousePos(savedMousePosX, savedMousePosY);
		    ProDBG_event(startId);
		    ProDBG_setMousePos(tempX, tempY);
		    ProDBG_setMouseState(MouseButton_Right, 0);
		}
		if (IMGUI_isItemHovered())
		{
		    context->popupHover = true;
		}
		uint32_t idMask = PRODBG_MENU_POPUP_SPLIT_VERT_SHIFT;
		PDMenuItem *desc = &menu[0];
		int popupMenuCount = 0;
		while (desc->name)
		{
		    if (IMGUI_menuItem(desc->name))
		    {
			float tempX = inputState->mousePos.x;
			float tempY = inputState->mousePos.y;
			ProDBG_setMousePos(savedMousePosX, savedMousePosY);	     
			ProDBG_event((uint32_t)popupMenuCount | idMask);
			ProDBG_setMousePos(tempX, tempY);
			ProDBG_setMouseState(MouseButton_Right, 0);
		    }
		    if (IMGUI_isItemHovered())
		    {
			context->popupHover = true;
		    }
		    ++popupMenuCount;
		    desc++;
		}
		IMGUI_endMenu();
		if (IMGUI_isItemHovered())
		{
		    context->popupHover = true;
		}
	    }	    
	    
	    IMGUI_endPopup();
	}
    }
    else
    {
	context->popupActive = false;
    }
#endif
}


void ProDBG_update()
{
    Context* context = &s_context;

    //uint64_t currentTime = time_current();
    //float dt = time_elapsed(context->time);
    //context->time = currentTime;

    //updateDock(context);

    g_pluginUI->preUpdate();

    Session_update(context->session);
    IMGUI_menuUpdate(context);


    g_pluginUI->postUpdate();

    FileMonitor_update();
}

// Temprory test for monkey

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void ProDBG_applicationLaunched()
{
    int pluginCount = 0;
    // NOTE(marco): this is where the native menu is created. We need to abstract this
    // so that we can use ImGUI to render the menu, while also being able to allow the
    // use of the native version (i.e. for Mac)
    Window_buildPluginMenu(PluginHandler_getViewPlugins(&pluginCount), pluginCount);
    createMenusForPlugins();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void ProDBG_destroy()
{
    Context* context = &s_context;

    //rmt_DestroyGlobalInstance(s_remotery);

	docksys_save_layout("data/current_layout_2.json");

    Session_destroy(context->session);

    Settings_save();

    g_pluginUI->destroy();

    Settings_destroy();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void ProDBG_timedUpdate()
{
    ProDBG_update();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void onLoadRunExec(Session* session, const char* filename)
{
    PluginData* pluginData = PluginHandler_findPlugin(0, "lldb_plugin", "LLDB", true);

    if (!pluginData)
    {
        pd_error("Unable to find LLDB backend\n");
        return;
    }

    Session_startLocal(session, (PDBackendPlugin*)pluginData->plugin, filename);

    // Temp test
    // Session_startLocal(context->session, (PDBackendPlugin*)pluginData->plugin, "t2-output/macosx-clang-debug-default/ProDBG.app/Contents/MacOS/prodbg");
    // Session_startLocal(context->session, (PDBackendPlugin*)pluginData->plugin, OBJECT_DIR "/crashing_native");
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// We should mave this place some where better, I really don't like this approach but will have to do for now

Con* getCoveredCon(int x, int y)
{
	return docksys_con_by_user_data(Session_getViewAt(s_context.session, x, y, 0));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Events

void ProDBG_event(int eventId)
{
    Context* context = &s_context;

    int count;

    PluginData** pluginsData = PluginHandler_getViewPlugins(&count);

    pd_info("eventId 0x%x\n", eventId);

    Vec2 mousePos = InputState_getState()->mousePos;
    (void)mousePos;

#if PRODBG_USING_DOCKING
    if (eventId & PRODBG_MENU_POPUP_SPLIT_HORZ_SHIFT)
    {
        eventId &= (PRODBG_MENU_POPUP_SPLIT_HORZ_SHIFT - 1);

        ViewPluginInstance* instance = g_pluginUI->createViewPlugin(pluginsData[eventId]);
		Con* con = getCoveredCon((int)mousePos.x, (int)mousePos.y);

		docksys_horizontal_split(con, instance);

        //UIDock_splitHorizontalAt(Session_getDockingGrid(context->session), (int)mousePos.x, (int)mousePos.y, instance);

        Session_addViewPlugin(context->session, instance);
        return;
    }

    if (eventId & PRODBG_MENU_POPUP_SPLIT_VERT_SHIFT)
    {
        eventId &= (PRODBG_MENU_POPUP_SPLIT_VERT_SHIFT - 1);

        ViewPluginInstance* instance = g_pluginUI->createViewPlugin(pluginsData[eventId]);
		Con* con = getCoveredCon((int)mousePos.x, (int)mousePos.y);

		docksys_vertical_split(con, instance);

		printf("con %p\n", con);

        //UIDock_splitVerticalAt(Session_getDockingGrid(context->session), (int)mousePos.x, (int)mousePos.y, instance);

        Session_addViewPlugin(context->session, instance);
        return;
    }

#endif

    // TODO: This code really needs to be made more robust.

#if 0
    if (eventId >= PRODBG_MENU_PLUGIN_START && eventId < PRODBG_MENU_PLUGIN_START + 9)
    {
        ViewPluginInstance* instance = g_pluginUI->createViewPlugin(pluginsData[eventId - PRODBG_MENU_PLUGIN_START]);

        UIDockingGrid* grid = Session_getDockingGrid(context->session);
        UIDock* dockAtMouse = UIDock_getDockAt(grid, 0, 0);
        UIDock_splitVertical(Session_getDockingGrid(context->session), dockAtMouse, instance);

        Session_addViewPlugin(context->session, instance);
        return;
    }
#endif

    switch (eventId)
    {
        case PRODBG_MENU_FILE_OPEN_AND_RUN_EXE:
        {
            char filename[4096];

            if (Dialog_open(filename))
            {
                onLoadRunExec(context->session, filename);
            }

            break;
        }

        case PRODBG_MENU_FILE_OPEN_SOURCE:
        {
            char filename[4096];

            if (Dialog_open(filename))
            {
                Session_loadSourceFile(context->session, filename);
            }

            break;
        }

        case PRODBG_MENU_DEBUG_START:
        {
            Session_action(context->session, PDAction_run);
            printf("run....\n");
            break;
        }

        case PRODBG_MENU_DEBUG_ATTACH_TO_REMOTE:
        {
            Session_startRemote(context->session, "127.0.0.1", 1340);
            break;
        }

        case PRODBG_MENU_DEBUG_TOGGLE_BREAKPOINT:
        {
            Session_toggleBreakpointCurrentLine(context->session);
            break;
        }

        case PRODBG_MENU_DEBUG_STEP_OVER:
        {
            Session_stepOver(context->session);
            break;
        }

        case PRODBG_MENU_DEBUG_STEP_IN:
        {
            Session_stepIn(context->session);
            break;
        }
    }

    if (eventId >= PRODBG_MENU_PLUGINS_START)
    {
        Session_onMenu(context->session, eventId);
    }
}




///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

bool ProDBG_getPopupHoverState()
{
    Context* context = &s_context;
    return context->popupHover;
}




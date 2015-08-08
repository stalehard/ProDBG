#pragma once

#include "../plugin.h"

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class BgfxPluginUI : public PluginUI
{
    void create(void* windowHandle, int width, int height);
    void destroy();

    void init(ViewPluginInstance* instance);
    State updateInstance(ViewPluginInstance* instance, PDReader* reader, PDWriter* writer);

    void preUpdate();   // update before plugin update
    void postUpdate(); // update after plugins

    int getStatusBarSize();
    int getMenuBarSize();
    void setStatusTextNoFormat(const char* text);

private:
    int m_statusSize = 18;
    int m_menuBarSize = 19;
    char m_statusText[4096];
};


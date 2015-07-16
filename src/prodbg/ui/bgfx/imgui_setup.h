#pragma once

struct InputState;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void IMGUI_setup(int width, int height);
void IMGUI_updateSize(int width, int height);
void IMGUI_preUpdate(const InputState* inputState, float deltaTime);
void IMGUI_postUpdate();

void IMGUI_setInputState(const InputState* inputState);
void IMGUI_setKeyDown(int key, int modifier);
void IMGUI_setKeyUp(int key, int modifier);

void IMGUI_addInputCharacter(unsigned short c);

bool IMGUI_beginMainMenuBar();
void IMGUI_endMainMenuBar();

bool IMGUI_beginMenu(const char *menuName);
bool IMGUI_menuItem(const char *itemName);
void IMGUI_endMenu();

void IMGUI_openPopup(const char *popupId);
bool IMGUI_beginPopup(const char *popupName);
void IMGUI_endPopup();

bool IMGUI_isItemHovered();

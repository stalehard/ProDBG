#pragma once

#include <stdbool.h>

struct Con;
struct json_t;
typedef void* DockHandle;

#define DOCKSYS_SUPPORTS_LOAD_SAVE 

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef enum DockSysCursor
{
    DockSysCursor_Default = 0,
    DockSysCursor_SizeHorizontal,
    DockSysCursor_SizeVertical,
} DockSysCursor;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct DockSysCallbacks {
	void (*update_window_size)(void *userData, int x, int y, int width, int height);
	void (*set_cursor_style)(DockSysCursor cursor);
	void (*save_user_data)(struct json_t* item, void* user_data);
	void* (*load_user_data)(struct json_t* item);
} DockSysCallbacks;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct DockSysAPI {
	void (*set_callbacks)(DockSysCallbacks* callbacks);

	void (*horz_split)(void* user_data, DockHandle handle);
	void (*vert_split)(void* user_data, DockHandle handle);

	void (*update_size)(int width, int height);
	void (*set_mouse)(void* user_data, float x, float y, bool left_down);
	bool (*is_hovering_border)();

	DockHandle (*get_handle_at)(float x, float y);

	void (*save_state)(const char* filename);
	void (*load_state)(const char* filename);

	void (*update)();

} DcokSysAPI;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void docksys_set_callbacks(DockSysCallbacks* callbacks);

void docksys_horizontal_split(struct Con *con, void *user_data);
void docksys_vertical_split(struct Con *con, void *user_data);

struct Con* docksys_create_workspace(const char *name);
struct Con* docksys_con_by_user_data(void* user_data);

void docksys_close_con(void* user_data);

bool docksys_is_hovering_border();

void docksys_create(int x, int y, int width, int height);
void docksys_set_mouse(void* user_data, int x, int y, bool leftDown); 
void docksys_update_size(int width, int height);

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#if defined(DOCKSYS_SUPPORTS_LOAD_SAVE) 

void docksys_save_layout(const char* filename);
bool docksys_load_layout(const char* filename);

#endif


void docksys_update();

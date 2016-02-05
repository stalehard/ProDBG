#ifndef _PD_DOCKING_H_
#define _PD_DOCKING_H_ 

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This is the API used for docking of views within ProDBG. It's possible to use this API to replace the
// existing one

typedef void* PDDockHandle;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef enum PDDockingCursor
{
    PDDockingCursor_Default = 0,
    PDDockingCursor_SizeHorizontal,
    PDDockingCursor_SizeVertical,
} PDDockingCursor;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct PDDockingCallbacks {
	void (*update_window_size)(void* user_data, int x, int y, int width, int height);
	void (*set_cursor_style)(void* user_data, PDDockingCursor cursor);
	void (*save_user_data)(void* item, void* user_data);
	void* (*load_user_data)(void* item);
} PDDockingCallbacks;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

typedef struct PDDocking {
	const char* name;

	void* (*create_instance)(int x, int y, int width, int height);
	void (*destroy_instance)(void* instance);

	void (*set_callbacks)(void* instance, PDDockingCallbacks* callbacks);

	void (*horz_split)(void* instance, void* user_data, PDDockHandle handle);
	void (*vert_split)(void* instance, void* user_data, PDDockHandle handle);

	void (*close_dock)(void* instance, PDDockHandle handle);

	void (*update_size)(void* instance, void* user_data, int width, int height);
	void (*set_mouse)(void* instance, void* user_data, float x, float y, bool left_down);
	bool (*is_hovering_border)(void* instance, void* user_data);

	PDDockHandle (*get_handle_at)(void* instance, float x, float y);

	void (*save_state)(void* instance, const char* filename);
	void (*load_state)(void* instance, void* user_data, const char* filename);

	void (*update)(void* instance);

} PDDocking;

#endif

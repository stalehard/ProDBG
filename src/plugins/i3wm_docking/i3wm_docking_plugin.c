#include "pd_docking.h"
#include "pd_common.h"
#include "i3wm_docking.h"
#include <stdlib.h>

struct DummyData {
	int data;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/*
typedef struct PDDocking {
	void (*set_callbacks)(void* user_data, PDDockingCallbacks* callbacks);

	void (*horz_split)(void* user_data, PDDockHandle handle);
	void (*vert_split)(void* user_data, PDDockHandle handle);

	void (*update_size)(void* user_data, int width, int height);
	void (*set_mouse)(void* user_data, float x, float y, bool left_down);
	bool (*is_hovering_border)(void* user_data);

	PDDockHandle (*get_handle_at)(void* user_data, float x, float y);

	void (*save_state)(void* user_data, const char* filename);
	void (*load_state)(void* user_data, const char* filename);

	void (*update)(void* user_data);

} PDDocking;
*/

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void* create_instance(int x, int y, int width, int height) {
	docksys_create(x, y, width, height);
	return malloc(sizeof(DummyData));
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void destroy_instance(void* data) {
	free(data);
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

PD_EXPORT PDDocking docking_system = {
	create_instance,
	destroy_instance,

	docksys_set_callbacks,

	docksys_horizontal_split,
	docksys_vertical_split,

	docksys_close_con,
	docksys_update_size,

};



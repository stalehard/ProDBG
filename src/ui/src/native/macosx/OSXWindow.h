#import <Cocoa/Cocoa.h>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// This needs to match Rust version in macos/mod.rs
typedef struct MouseData {
	float x, y;
	float sx, sy;
	uint8_t mouse_state[5];
} MouseData;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// This needs to match Rust version in macos/mod.rs
typedef struct RustWindow {
	void* window_handle;
	MouseData mouse_data;
} RustWindow;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

@interface OSXWindow : NSWindow
{
	NSView* childContentView;
	@public void (*key_callback)(void* user_data, int key, int state);
	@public RustWindow* rust_window;
	@public bool should_close;
}
@end


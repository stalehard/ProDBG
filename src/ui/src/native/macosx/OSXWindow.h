#import <Cocoa/Cocoa.h>

@interface OSXWindow : NSWindow
{
	NSView* childContentView;
	@public void (*key_callback)(void* user_data, int key, int state);
	@public int width;
	@public int height;
	@public int scale;
	@public void* rust_data;
	@public bool should_close;
}

@end

@interface WindowDelegate : NSObject
{
    OSXWindow* window;
}

- (id)initWithwWindow:(OSXWindow*)initWindow;

@end

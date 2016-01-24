#import "OSXWindowFrameView.h"

@implementation OSXWindowFrameView

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

- (BOOL)acceptsFirstResponder {
	printf("acceptsFirstResponder\n");
    return YES;
}

- (BOOL)acceptFirstMouse:(NSEvent*)event {
	(void)event;
	return YES;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
- (void)mouseUp:(NSEvent*)event {
    (void)event;
    printf("mouse up\n");
}

- (void)mouseDown:(NSEvent*)event {
    (void)event;
    printf("mouse down\n");
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

-(void) viewWillMoveToWindow:(NSWindow*)newWindow {
	printf("viewWillMoveToWindow\n");
    NSTrackingArea* trackingArea = [[NSTrackingArea alloc] initWithRect:[self frame]
                                    options: (NSTrackingAreaOptions)(NSTrackingInVisibleRect | NSTrackingMouseMoved | NSTrackingActiveAlways) owner:self userInfo:nil];
    [self addTrackingArea:trackingArea];
    (void)newWindow;
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/*
- (void)drawRect:(NSRect)rect
{
	CGContextRef context = [[NSGraphicsContext currentContext] graphicsPort];

	CGColorSpaceRef space = CGColorSpaceCreateDeviceRGB();
	CGDataProviderRef provider = CGDataProviderCreateWithData(NULL, draw_buffer, width * height * 4, NULL); 

	CGImageRef img = CGImageCreate(width, height, 8, 32, width * 4, space, kCGImageAlphaNoneSkipFirst | kCGBitmapByteOrder32Little, 
								   provider, NULL, false, kCGRenderingIntentDefault);

	CGColorSpaceRelease(space);
	CGDataProviderRelease(provider);

	CGContextDrawImage(context, CGRectMake(0, 0, width * scale, height * scale), img);

	CGImageRelease(img);
}
*/

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

- (void)lockFocus {
    //NSOpenGLContext* context = (NSOpenGLContext*)bgfx::nativeContext();

    [super lockFocus];

    //if ([context view] != self)
    //  [context setView:self];

    //[context makeCurrentContext];
    //[[self window] setTitle:PRODBG_VERSION];
}

@end


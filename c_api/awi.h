#include <stdint.h>

/// An input event.
union AwiInput {
	// No more input events from last frame. 
	none;
};

/// Window subsystem (graphics & input) should be initialized.
#define AWI_GRAPHICS 1
/// Audio subsystem should be initialized.
#define AWI_AUDIO 2

/// Create a new window.
/// ```
/// #include "awi.h"
///
/// void* window;
/// awi_new(&window, AWI_GRAPHICS | AWI_AUDIO);
///
/// uint16_t width, height;
/// awi_wh(window, &width, &height);
///
/// AwiInput input;
/// awi_input(window, &input);
///
/// awi_update(window);
///
/// awi_free(window);
/// ```
void awi_new(void** window, uint32_t flags);

/// Get Input
void awi_input(void* window, AwiInput* input);

/// Update window.
void awi_update(void* window);

/// Close the window.
void awi_free(void* window);

/// Get the window width and height.
void awi_wh(void* window, uint16_t* width, uint16_t* height);

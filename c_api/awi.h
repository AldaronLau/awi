#include <stdint.h>

#define AwiWindow void

union AwiInput {
	AwiInput
};

typedef struct AwiWh {
	uint16_t w;
	uint16_t h;
};

AwiWindow* awi_new(void);

AwiInput awi_input(AwiWindow* window);

void awi_drop(AwiWindow* window);

AwiWh awi_wh(AwiWindow* window);

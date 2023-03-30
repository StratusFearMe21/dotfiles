#ifndef BUILD_WOB_H
#define BUILD_WOB_H

#include <stdio.h>
#include "config.h"

#define INPUT_BUFFER_LENGTH 255

int wob_run(struct wob_config *config, FILE* fd);

#endif

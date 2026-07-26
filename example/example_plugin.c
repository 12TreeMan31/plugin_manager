#include "includes/abi.h"
#include <stdio.h>

char *hello(char *name) {
    printf("Hello, %s!\n", name);
    return NULL;
}

char *goodbye(char *name) {
    printf("Goodbye %s\n", name);
    return NULL;
}

const PluginFunction functions[] = {
    {"hello", hello},
    {"goodbye", goodbye},
};

PluginInfo info = {
    .plugin_name = "example_plugin",
    .fn_count = 2,
    .fns = functions,
};

PluginInfo *plg_endpoints(void) {
    return &info;
}

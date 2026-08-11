/// Example on how to write a plugin in C though it should be about the same in Rust.
#include <stdio.h>
#include <stdlib.h>

#include "abi.h"

PluginString hello_fmt(const char *name) {
    char *buffer = calloc(20, sizeof(char));
    snprintf(buffer, 20, "Hello %s!", name);

    PluginString ret = {
        .data = buffer,
        .kind = PLUGIN_STRING_OWNED,
    };

    return ret;
}

PluginString goodbye(const char *_json) {
    PluginString ret = {
        .data = "Goodbye everyone!",
        .kind = PLUGIN_STRING_STATIC,
    };

    return ret;
}

const PluginFunction functions[] = {
    {"hello", hello_fmt},
    {"goodbye", goodbye},
};

void string_free(PluginString str) {
    if (str.kind == PLUGIN_STRING_STATIC) {
        return;
    }

    free(str.data);
}

PluginInfo info = {
    .plugin_name = "example_plugin",
    .version = 1,
    .fn_count = 2,
    .fns = functions,
    .string_free = string_free,
};

PluginInfo *plg_endpoints(void) {
    return &info;
}

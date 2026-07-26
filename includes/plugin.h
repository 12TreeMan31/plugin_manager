#ifndef PLUGIN_H
#define PLUGIN_H

#include <stdbool.h>

#include "includes/abi.h"

typedef struct Plugin {
    void *handle;
    PluginInfo *(*plg_endpoints)(void);
    PluginInfo *info;

    struct Plugin *next;
} Plugin;

typedef struct PluginError {
    bool isError;
    union {
        char *error;
        Plugin *plg;
    } data;
} PluginError;

PluginError plugin_register(const char *path);
void plugin_deregister(Plugin *plg);
int plugin_append(Plugin **plgs, Plugin *new_plg);

#endif
#ifndef PLUGIN_H
#define PLUGIN_H

#include "abi.h"

typedef struct Plugin {
    void *handle;
    PluginInfo *(*plg_endpoints)(void);
    PluginInfo *info;
} Plugin;

typedef struct PluginError {
    bool is_error;
    union {
        char *error;
        Plugin plg;
    } data;
} PluginError;

PluginError plugin_register(const char *restrict path);
void plugin_deregister(Plugin self);
char *plugin_name(const Plugin *self);
PluginString plugin_call(const Plugin *restrict self, const char *function, const char *json);
unsigned int plugin_version(const Plugin *self);

void plugin_string_free(const Plugin *self, PluginString str);

#endif
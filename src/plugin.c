#include <dlfcn.h>
#include <stdlib.h>
#include <string.h> // strcmp

#include "abi.h"
#include "plugin.h"

char *plugin_name(const Plugin *self) {
    return self->info->plugin_name;
}

unsigned int plugin_version(const Plugin *self) {
    return self->info->version;
}

PluginString plugin_call(const Plugin *restrict self, const char *function, const char *json) {
    PluginInfo *info = self->info;

    for (int i = 0; i < info->fn_count; i++) {
        if (strcmp(function, info->fns[i].name) == 0) {
            return info->fns[i].function(json);
        }
    }

    PluginString err = {
        .data = NULL,
        .kind = PLUGIN_STRING_STATIC,
    };

    return err;
}

PluginError plugin_register(const char *restrict path) {
    char *err_msg = NULL;

    void *handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);
    if (!handle) {
        err_msg = dlerror();
        goto LINK_ERR;
    }

    // See dlsym(3)
    dlerror();
    PluginInfo *(*plg_endpoints)(void) = dlsym(handle, "plg_endpoints");
    err_msg = dlerror();
    if (err_msg != NULL) {
        goto ERROR;
    }

    // Get the plugins functions
    PluginInfo *info = plg_endpoints();
    if (!info) {
        err_msg = "plg_endpoint failed";
        goto ERROR;
    }

    Plugin endpoint = {
        .handle = handle,
        .info = info,
        .plg_endpoints = plg_endpoints,
    };

    PluginError ok = {
        .is_error = false,
        .data.plg = endpoint,
    };

    return ok;

ERROR:
    dlclose(handle);
LINK_ERR:
    PluginError err = {
        .is_error = true,
        .data.error = err_msg,
    };

    return err;
}

void plugin_deregister(Plugin self) {
    // Please read the man pages for this
    dlclose(self.handle);
}

void plugin_string_free(const Plugin *self, PluginString str) {
    self->info->string_free(str);
}
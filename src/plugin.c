#include <dlfcn.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h> // strcmp

#include "includes/abi.h"
#include "includes/plugin.h"

PluginError plugin_register(const char *path) {
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

    Plugin *endpoint = malloc(sizeof(*endpoint));
    if (!endpoint) {
        err_msg = "malloc failed";
        goto ERROR;
    }

    endpoint->handle = handle;
    endpoint->info = info;
    endpoint->plg_endpoints = plg_endpoints;

    PluginError ok = {
        .isError = false,
        .data.plg = endpoint,
    };

    return ok;

ERROR:
    dlclose(handle);
LINK_ERR:
    PluginError err = {
        .isError = true,
        .data.error = err_msg,
    };

    return err;
}

void plugin_deregister(Plugin *plg) {
    // Please read the man pages for this
    dlclose(plg->handle);
    free(plg);
}

// In order to make reloading simpler, we will just override the namespace
int plugin_append(Plugin **plgs, Plugin *new_plg) {
    if (!new_plg || !plgs) {
        return -1;
    }

    // First plugin to be loaded
    if (*plgs == NULL) {
        *plgs = new_plg;
        new_plg->next = NULL;

        return 0;
    }

    // Sliding window of width 2
    Plugin *curr = *plgs;
    Plugin *prev = NULL;

    while (curr) {
        // Replace plugin with same namespace
        if (strcmp(curr->info->plugin_name, new_plg->info->plugin_name) == 0) {
            new_plg->next = curr->next;

            // RACE CONDITION: Not atomic or locked
            if (prev == NULL) {
                // Replace the head
                *plgs = new_plg;
            } else {
                // Insert in place
                prev->next = new_plg;
            }

            deregister_plugin(curr);

            return 0;
        }

        prev = curr;
        curr = curr->next;
    }

    // Add plugin to list
    new_plg->next = *plgs;
    *plgs = new_plg;

    return 0;
}

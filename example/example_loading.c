/// YES THIS IS BAD. I just wanted a working example
#include <dlfcn.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "includes/abi.h"

typedef struct {
    void *handle;
    PluginInfo *(*plg_endpoints)(void);
    PluginInfo *info;
} PluginEndpoint;

PluginEndpoint *register_plugin(char *path) {
    void *handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);
    if (!handle)
        return NULL;

    // This is incorrect, see dlsym(3) for proper error handling
    PluginInfo *(*plg_endpoints)(void) = dlsym(handle, "plg_endpoints");
    if (!plg_endpoints)
        return NULL;
    PluginInfo *info = plg_endpoints();

    PluginEndpoint *endpoint = malloc(sizeof(*endpoint));
    endpoint->handle = handle;
    endpoint->info = info;
    endpoint->plg_endpoints = plg_endpoints;

    return endpoint;
}

const char *menu = "Dynamic Loading\n\n"
                   "1) Load libdyn.so\n"
                   "2) Run funtion by name\n";

int numInput() {
    printf("> ");
    int opt = 0;
    scanf("%d", &opt);
    fflush(stdin);

    return opt;
}

void list_functions(PluginInfo *info) {
    for (int i = 0; i < info->fn_count; i++) {
        printf("%02d) %s\n", i, info->fns[i].name);
    }
}

int main() {
    PluginEndpoint *plugin;

    bool running = true;
    while (running) {
        printf("%s", menu);
        int opt = numInput();

        PluginFn fn;

        switch (opt) {
        case 1:
            plugin = register_plugin("./libdyn.so");
            if (!plugin) {
                printf("Failed to load\n");
                break;
            }

            break;
        case 2:
            printf("-[Functions]-----------\n");
            list_functions(plugin->info);

            opt = numInput();
            fn = plugin->info->fns[opt].function;
            fn("Dave");

            break;
        }
    }

    return 0;
}
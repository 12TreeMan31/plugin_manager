#ifndef ABI_H
#define ABI_H

/// There are a couple of things to keep in mind while making a plugin
/// - All types below expect to have a static lifetime.
/// - In the case of `PluginFn`, you do not control the lifetime, the calling program does.
/// - The plugin is only reqired to implement the function with the signature `const
/// PluginInfo *plg_endpoints(void)`. Return NULL if error.
/// - Function will always receiving and returning a JSON object.
/// - All plugins are expected the be thread safe if you want multiple computers using it.

/// Pointer to function being exported
typedef char *(*PluginFn)(char *);

/// Here name can be anything as long as the function pointer is valid.
/// Name should be used as an alias to the function as its what CC will call it
typedef struct {
    const char *name;
    const PluginFn function;
} PluginFunction;

typedef struct {
    /// The namespace given to this plugin to be referenced by CC
    const char *plugin_name;

    const unsigned long fn_count;
    const PluginFunction *fns;
} PluginInfo;

#endif
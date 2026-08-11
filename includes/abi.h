#ifndef ABI_H
#define ABI_H

/// There are two things to keep in mind while designing a plugin:
/// - All pointers in `PluginInfo` MUST be valid for the lifetime of the plugin.
/// - All functions MUST be thread safe.
/// Failure to do so will result in a program crash.
///
/// A plugin must implement:
/// - `plg_endpoints` which returns `PluginInfo`. `plg_endpoints` is the first thing
/// called when registering a plugin. It may return NULL if there is an error.
/// - All functions to be used across the ABI must be `PluginFn`.
/// - `PluginFreeFn` which for for freeing `PluginString`s.
/// - All `PluginString` must be valid null terminated UTF8 string.
/// - Anything not listed here means its free game

/// String lifetimes
typedef enum {
    PLUGIN_STRING_STATIC,
    PLUGIN_STRING_OWNED,
} PluginStringKind;

/// A string with a static or dynamic lifetime
typedef struct {
    char *data;
    PluginStringKind kind;
} PluginString;

/// Pointer to functions being exported
typedef PluginString (*PluginFn)(const char *);
typedef void (*PluginFreeFn)(PluginString str);

/// Here name can be anything as long as the function pointer is valid.
/// Name should be used as an alias to the function as its what CC will call it
typedef struct {
    const char *name;
    const PluginFn function;
} PluginFunction;

typedef struct {
    /// The name given to this plugin to be referenced by CC
    char *plugin_name;
    /// Version used for debug information
    unsigned int version;
    /// Number of elements in `fns`
    unsigned long fn_count;

    /// Function call table
    PluginFunction *fns;
    PluginFreeFn string_free;
} PluginInfo;

/// Plugin entrypoint
typedef const PluginInfo *(*plg_endpoints)(void);

#endif

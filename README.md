# Plugin Manager
See `includes/abi.h` and `example/example_plugin.c` for how to write plugins.

## Usage

### Console
After running the program you will be dropped into the console. There are currently three commands that are supported:
- `load [path]` - Loads a plugin at the given **absolute** path
- `remove [name]` - Removes the plugins with the given name
- `list` - Lists all currently loaded plugins and their versions

### Computer Craft

#### Opening a connections
Look at `example/client.lua` for how you open a websocket connections. You will notice that there are two additional headers required to make a connection:
- `Plugin-Name` - The name of the **loaded** plugin you would like to use
- `Computer-Name` - A memorable name that can be used to id this computer in `plugin-manager` logs

Note: In a later version you will be able to connect to plugin before its loaded, so it is a good idea to have a place holder spot for validating your connections.

#### Calling a plugin function
Its pretty easy to call a function once you are connected. The syntax is {function name}-{input data}. 

For example to call `hello` in `example_plugin` you would send:
```
hello-John C
```
You would then get a string returned to you with the following message:
```
Hello John C
```

Now what if the function you want to call doesn't take any inputs? For now you would make a call like so:
```
example-{}
```
This will change in later versions though
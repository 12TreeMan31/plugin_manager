build:
	gcc -Wall -I. src/main.c

lib:
	gcc -Wall -I. -fPIC -shared -o libdyn.so src/example_plugin.c
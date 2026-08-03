exlib:
	clang -Wall -Iincludes -std=c23 -fPIC -shared -o libexample.so example/example_plugin.c
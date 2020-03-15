# Define required macros here
SHELL = /bin/sh

release:
	make -C oil-level-monitor
clean:
	make -C oil-level-monitor clean
database:
	make -C database
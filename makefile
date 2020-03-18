# Define required macros here
SHELL = /bin/sh

local:
	make -C oil-level-monitor
	make -C database docker
	make -C frontend
release:
	make -C oil-level-monitor
clean:
	make -C oil-level-monitor clean
database:
	make -C database
run:
	DATABASE_HOST="postgres" DATABASE_PORT="5432" docker-compose up
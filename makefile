# Define required macros here
SHELL = /bin/sh

release:
	make -C oil-level-monitor
clean:
	make -C oil-level-monitor clean
database:
	make -C database
run:
	DATABASE_HOST="postgres" DATABASE_PORT="5432" DATABASE_USERNAME="oil_level_user" DATABASE_PASSWORD="password" docker-compose up
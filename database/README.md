# Liquibase schema definition

A makefile has been included for convenience, as long as you have maven installled, the make command will apply any schema changes you make to the target database

3 environment variables are used to control the database location and credentials:

1. DATABASE_HOST - The hostname of the server of the database - if running in docker-compose this is postgres
2. DATABASE_PORT - The port on the server for the database
3. DATABASE_USERNAME - The username for the database user
4. DATABASE_PASSWORD - The password for the database user

You should set these values when running docker-compose at the top level


# Self hosted Oil Tank Level Monitoring
This project consists of the arduino code and wiring instructions in `oil-level-sender`, a REST API `oil-level-monitor` written in Rust  for sending a receiving Current Heating Oil Level and History and a Front End web portal `frontend` for viewing the level. The project is managed through docker-compose, data is stored in a postgreSQL `database` which uses Liquibase to manage schema updates

## modules

### oil-level-sender
[View README.md](https://github.com/stevenleadbeater/oil-level-monitor/blob/master/oil-level-sender/README.md)

Arduino sketch for reading the oil level with an ultra sonic sensor and POSTing the reading to the back end REST API

### oil-level-monitor
[View README.md](https://github.com/stevenleadbeater/oil-level-monitor/blob/master/oil-level-monitor/README.md)

REST API written in Rust, POST end point for receiving the reading, GET end points for current level and history, websocket support for currrent level

### frontend
[View README.md](https://github.com/stevenleadbeater/oil-level-monitor/blob/master/frontend/README.md)

React web application with a nice looking mobile enabled UI

### database
[View README.md](https://github.com/stevenleadbeater/oil-level-monitor/blob/master/database/README.md)

Liquibase changsets defining the database schema

 
##Running

There are some complexities around how the project is run, there are 4 separate containers that will be running on the server which provides access to the API and the front end they are as follows:

1. REST API
2. Frontend
3. Postgres Database
4. Maven container with liquibase scripts inside it 

In order to build the REST API, you will need to have [rust](https://www.rust-lang.org/tools/install) installed, the `x86_64-unknown-linux-musl` target `rustup target add x86_64-unknown-linux-musl` and the musl libc tools. This is because the REST API runs on an Alpine linux docker container, Alpine does not use GCC so statically linked binaries have to be copied in to the container in order to run. This has thus far only been tested on Ubuntu: `sudo apt install musl-tools`

Using the 4th container to hold the liquibase scripts ensures that the docker network is used for the REST API and liquibase to talk to the postgres database. This means that the port for the DB is only open within the docker network created by the compose command and it is not open to the host machine

As the frontend is entirely static files being served by an nginx based container, the address it uses for the REST API has to be interpolated as part of the build

All of this is taken care of when running the default profile for the make command in the root of the repo, simply pass the BACKEND_HOST and BACKEND_PORT:

`REACT_APP_BACKEND_HOST=192.168.1.7 REACT_APP_BACKEND_PORT=8120 make`

The below command will start the containers and run liquibase:

`DATABASE_USERNAME="oil_level_user" DATABASE_PASSWORD="password"  make run`
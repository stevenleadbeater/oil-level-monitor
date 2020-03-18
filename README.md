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

The below command will start the containers and run liquibase:

`DATABASE_HOST="postgres" DATABASE_PORT="5432" DATABASE_USERNAME="oil_level_user" DATABASE_PASSWORD="password" docker-compose up`
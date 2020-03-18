# Frontend
This is the react front end for the oil level monitor, the web app is very simple but you will need to build the docker container yourself. There is static config that is interpolated at build time

## Building

On the server that will host the front end:

1. First run `REACT_APP_BACKEND_HOST=192.168.1.7 REACT_APP_BACKEND_PORT=8120 make build` making sure you replace `192.168.1.7` with the IP address or DNS name of the server on which you are running the REST API
2. Run `make docker`

You now have a docker container with the front end built inside it.
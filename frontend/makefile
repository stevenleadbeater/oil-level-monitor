build:
	npm run build
	make docker

docker:
	make docker-target
	make docker-build

docker-target:
	mkdir -p docker
	cp Dockerfile docker
	cp -r build docker

docker-build:
	cd docker && docker build . -t oil-level-monitor-frontend:latest

clean:
	docker rmi -f oil-level-monitor-frontend:latest
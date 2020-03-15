# Define required macros here
SHELL = /bin/sh

local:
	make rust-build
	make docker-clean
	make docker-target
	make docker-build

rust-test:
	CC="musl-gcc -static -Os" CXX="/usr/bin/g++" cargo test --release --target=x86_64-unknown-linux-musl

rust-build:
	CC="musl-gcc -static -Os" CXX="/usr/bin/g++" cargo build --release --target=x86_64-unknown-linux-musl

docker-clean:
	docker rmi -f registry.gitlab.com/ledsys/oil-level-monitor:0.1.0
	rm -rf docker
	mkdir -p docker

docker:
	make docker-target
	make docker-build

docker-target:
	mkdir -p docker
	cp *.toml docker
	cp Dockerfile docker
	cp target/x86_64-unknown-linux-musl/release/oil-level-monitor docker

docker-build:
	docker rmi -f registry.gitlab.com/ledsys/oil-level-monitor:0.1.0 || true
	cd docker && docker build . -t registry.gitlab.com/ledsys/oil-level-monitor:0.1.0

clean:
	docker rmi -f registry.gitlab.com/ledsys/oil-level-monitor:0.1.0
	rm -rf target/x86_64-unknown-linux-musl/release

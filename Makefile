.PHONY: watch
watch:
	cargo watch -x check -x "fmt --check" -x "clippy -- -D warnings" -x test -x run

.PHONY: build-docker
build-docker:
	docker build --tag http-mqtt-bridge --file Dockerfile .

.PHONY: run-docker
run-docker: build-docker
	docker run -p 8000:8000 http-mqtt-bridge

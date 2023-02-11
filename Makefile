.PHONY: watch
watch:
	cargo watch -x check -x "fmt --check" -x "clippy -- -D warnings" -x test -x run

.PHONY: build-docker
build-docker:
	docker build --tag http-mqtt-bridge --file Dockerfile .

.PHONY: run-docker
run-docker: build-docker
	docker run --env-file .env -p 8000:8000 http-mqtt-bridge

.PHONY: start-docker
start-docker: build-docker
	docker run -d --env-file .env -p 8000:8000 --name http-mqtt-bridge http-mqtt-bridge

.PHONY: stop-docker
stop-docker:
	docker rm -f http-mqtt-bridge

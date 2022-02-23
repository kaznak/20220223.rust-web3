NAME=web3-server

.PHONY: dev
dev:
	docker-compose up

.PHONY: run
run: build
	docker run --rm web3-server

.PHONY: build
build:
	docker build -t $(NAME) .

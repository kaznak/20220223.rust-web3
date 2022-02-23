NAME=web3-server

.PHONY: create-repository
create-repository:
	gcloud artifacts repositories create $(NAME)	\
		--repository-format=docker	\
		--location=us-central1	\
		--description="https://github.com/kaznak/20220223.rust-web3"

.PHONY: dev
dev:
	docker-compose up

.PHONY: run
run-local: build
	docker run --rm web3-server

.PHONY: build
build:
	docker build -t $(NAME) .

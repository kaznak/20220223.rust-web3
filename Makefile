NAME=web3-server
TAG=latest

HOST=us-central1-docker.pkg.dev
PROJECT=sandbox-266608
REPOSITORY=rust-web3
IMAGE_URI=$(HOST)/$(PROJECT)/$(REPOSITORY)/$(NAME):$(TAG)

.PHONY: deploy
deploy:
	gcloud compute instances create-with-container $(NAME)	\
		--container-image $(IMAGE_URI)


.PHONY: push
push:
	gcloud auth configure-docker $(HOST)
	docker login $(HOST)
	docker tag $(NAME) $(IMAGE_URI)
	docker push $(IMAGE_URI)

.PHONY: create-repository
create-repository:
	gcloud artifacts repositories create $(REPOSITORY)	\
		--repository-format=docker	\
		--location=us-central1	\
		--description="https://github.com/kaznak/20220223.rust-web3"

.PHONY: dev
dev:
	docker-compose up

.PHONY: run
run-local:
	docker run --rm -p 8080:8080 $(NAME)

.PHONY: build
build:
	docker build -t $(NAME) .

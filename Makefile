.PHONY: docker-build docker-dev

docker-build:
	@docker compose build

docker-dev: docker-build
	@docker compose up --abort-on-container-exit


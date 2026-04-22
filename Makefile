.PHONY: docker-build docker-dev lint lint-prettier lint-shell lint-dockerfile

docker-build:
	@docker compose build --pull

docker-dev: docker-build
	@docker compose up --abort-on-container-exit


lint-prettier:
	@echo "Running prettier on JSON and YAML files..."
	@npx -y prettier --write "**/*.{json,yaml,yml}"

lint-shell:
	@echo "Running shfmt on shell scripts..."
	@docker run --rm -u "$(id -u):$(id -g)" -v ".:/mnt" -w /mnt mvdan/shfmt:latest -w -i 2 scripts/entrypoint.sh scripts/docker/*.sh

lint-dockerfile:
	@echo "Running hadolint on Dockerfiles..."
	@docker run --rm -i hadolint/hadolint < Dockerfile

lint: lint-prettier lint-shell lint-dockerfile

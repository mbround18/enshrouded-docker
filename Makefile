.PHONY: lint dev

lint:
	@docker compose build linter
	@docker compose run --rm linter

dev:
	@docker compose up --build --abort-on-container-exit
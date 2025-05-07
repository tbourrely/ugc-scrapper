run-scrapper:
	cargo watch -x run -- cargo run --bin ugc_scrapper

run-day:
	cargo watch -x run -- cargo run --bin select_day

test:
	cargo test

dev-start:
	docker compose --profile dev up

integration-tests:
	rm -f venom.*
	venom run --stop-on-failure venom/

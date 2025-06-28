run-scrapper:
	cargo run --bin scrapper

run-day:
	cargo run --bin select_day

run-movie:
	cargo run --bin select_movie

watch-scrapper:
	cargo watch -x run -- cargo run --bin scrapper

watch-day:
	cargo watch -x run -- cargo run --bin select_day

watch-movie:
	cargo watch -x run -- cargo run --bin select_movie

test:
	cargo test

dev-start:
	docker compose --profile dev up

integration-tests:
	rm -f venom.*
	venom run --stop-on-failure venom/

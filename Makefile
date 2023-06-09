run:
	cargo run | npx bunyan
watch:
	cargo watch -x check -x test -x run | npx bunyan
coverage:
	cargo tarpaulin --ignore-tests
lint:
	cargo clippy -- -D warnings
format:
	cargo fmt -- --check
audit:
	cargo audit
build-image:
	docker build -t zero2prod -f ./Dockerfile .
run-image:
	docker run -p 8080:8080 zero2prod
init-db:
	./scripts/init_db.sh

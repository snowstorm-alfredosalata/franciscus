DATA_DIR ?= ../franciscus-data
DB_OUTPUT = app/static/franciscus.db

.PHONY: all db app dev clean

all: db app

db:
	cargo run --manifest-path server/Cargo.toml -- build --data-dir $(DATA_DIR) --output $(DB_OUTPUT)

app: db
	cd app && npm run build

dev: db
	cd app && npm run dev

clean:
	rm -f $(DB_OUTPUT)
	rm -rf app/build
	rm -rf server/target

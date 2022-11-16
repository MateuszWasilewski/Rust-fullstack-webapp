
ifndef VERBOSE
.SILENT:
else
GENERAL_ARGS += -v
endif

.PHONY: \
	build-backend \
	build-frontend \
	run-server \

build-backend:
	cargo build

build-frontend:
	cd frontend && trunk build

all: run-server

run-server: build-backend build-frontend
	cargo run
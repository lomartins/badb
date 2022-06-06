exec_name :=badb

build:
	cargo build --release


install:
	@echo "Installing..."
	cp target/release/$(exec_name) /bin/$(exec_name)


install-user:
	@echo "Installing for user..."
	cp target/release/$(exec_name) ~/.local/bin/$(exec_name)

all: build install-user
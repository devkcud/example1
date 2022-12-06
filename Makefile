all: compile

compile:
	@cargo build -q --release

try: compile
	@./target/release/example1 ./samples/test3.txt

env_add: compile
	sudo cp ./target/release/example1 /usr/local/bin

clean:
	@rm -rf ./target

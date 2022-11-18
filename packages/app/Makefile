all: app dist

app:
	cargo lambda build --release --target aarch64-unknown-linux-gnu

dist:
	mkdir -p ./.dist
	cp -r target/lambda/* ./.dist/
	ls -al ./.dist/
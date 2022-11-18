all: transcribe transcribe_post_process subtitle index_subtitle search_subtitle image_frame dist

transcribe:
	cargo lambda build --release --bin transcribe --target aarch64-unknown-linux-gnu

transcribe_post_process:
	cargo lambda build --release --bin transcribe_post_process --target aarch64-unknown-linux-gnu

subtitle:
	cargo lambda build --release --bin subtitle --target aarch64-unknown-linux-gnu

index_subtitle:
	cargo lambda build --release --bin index_subtitle --target aarch64-unknown-linux-gnu

search_subtitle:
	cargo lambda build --release --bin search_subtitle --target aarch64-unknown-linux-gnu

image_frame:
	cargo lambda build --release --bin image_frame --target aarch64-unknown-linux-gnu

dist:
	mkdir -p ./.dist
	cp -r target/lambda/* ./.dist/
	ls -al ./.dist/
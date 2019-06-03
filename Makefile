.PHONY: publish build-release

publish:
	cargo publish

build-lambda-release:
	docker run --rm -it -v "$(shell pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release -p aws-codebuild-status_lambda
	cp ./target/x86_64-unknown-linux-musl/release/aws-codebuild-status_lambda bootstrap
	zip -j rust.zip bootstrap

build-release:
	cargo build --release -p aws-codebuild-status
	strip target/release/aws-codebuild-status
	cp target/release/aws-codebuild-status .

release: build-release build-lambda-release
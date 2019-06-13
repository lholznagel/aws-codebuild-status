.PHONY: publish build-release

publish:
	cargo publish

build-cli-release:
	cargo build --release -p aws-codebuild-status
	strip target/release/aws-codebuild-status
	cp target/release/aws-codebuild-status .

build-server-release:
	cargo build --release -p aws-codebuild-status_server
	strip target/release/aws-codebuild-status_server
	cp target/release/aws-codebuild-status_server .

release: build-cli-release build-server-release
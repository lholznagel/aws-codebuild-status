.PHONY: publish build-release

publish:
	cargo publish

build-release:
	cargo build --release
	strip target/release/aws-codebuild-status
	cp target/release/aws-codebuild-status .
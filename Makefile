.PHONY: release release-cli release-cli-musl release-server release-server-musl

release-cli:
	mkdir -p dist
	cargo build --release -p aws-codebuild-status
	strip target/release/aws-codebuild-status
	cp target/release/aws-codebuild-status dist/aws-codebuild-status

release-cli-musl:
	mkdir -p dist
	docker run --rm -it -v "$(shell pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release -p aws-codebuild-status_lambda
	cp ./target/x86_64-unknown-linux-musl/release/aws-codebuild-status dist/aws-codebuild-status-musl

release-server:
	mkdir -p dist
	cargo build --release -p aws-codebuild-status_server
	strip target/release/aws-codebuild-status_server
	cp target/release/aws-codebuild-status_server dist/aws_codebuild-status_server

release-server-musl:
	mkdir -p dist
	docker run --rm -it -v "$(shell pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release -p aws-codebuild-status_lambda
	cp ./target/x86_64-unknown-linux-musl/release/aws-codebuild-status_server dist/aws-codebuild-status_server-musl

release: release-cli release-cli-musl release-server release-server-musl
.PHONY: publish build-release

publish:
	cd derive; cargo publish
	cd terminal; cargo publish
	cd web; cargo publish
	cd core; cargo publish

build-release:
	cargo build --release
	strip target/release/aws-codebuild-status
	cp target/release/aws-codebuild-status .
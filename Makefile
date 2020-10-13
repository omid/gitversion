_CRATE_VERSION=$(shell cargo pkgid | cut -d# -f2 | cut -d: -f2)

build_docker:
	docker build -f docker/Dockerfile --pull -t omidmr/gitversion -t omidmr/gitversion:$(_CRATE_VERSION) .

push_docker:
	docker push omidmr/gitversion:$(_CRATE_VERSION)
	docker push omidmr/gitversion:latest

cargo_publish:
	cargo publish

check:
	@cargo +nightly fmt
	@cargo clippy
	@cargo outdated -R
	@cargo update --dry-run

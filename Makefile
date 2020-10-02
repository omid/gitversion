CRATE_VERSION=$(shell cargo pkgid | cut -d# -f2 | cut -d: -f2)

build_docker:
	docker build --no-cache -t omidmr/gitversion -t omidmr/gitversion:$(CRATE_VERSION) docker

push_docker:
	docker push omidmr/gitversion:$(CRATE_VERSION)
	docker push omidmr/gitversion:latest

cargo_publish:
	cargo publish

CRATE_VERSION=$(shell cargo pkgid | cut -d# -f2 | cut -d: -f2)

build_docker:
	docker build -f docker/Dockerfile --pull -t omidmr/gitversion -t omidmr/gitversion:$(CRATE_VERSION) .

push_docker:
	docker push omidmr/gitversion:$(CRATE_VERSION)
	docker push omidmr/gitversion:latest

cargo_publish:
	cargo publish

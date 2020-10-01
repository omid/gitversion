CRATE_VERSION=$(shell head -4 Cargo.toml | grep ^version | awk '{print $$3}' | tr -d \")

build_docker:
	docker build --no-cache -t omidmr/gitversion -t omidmr/gitversion:$(CRATE_VERSION) docker

push_docker:
	docker push omidmr/gitversion:latest
	docker push omidmr/gitversion:$(CRATE_VERSION)

cargo_publish:
	cargo publish

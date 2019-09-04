build_docker:
	docker build --no-cache -t omidmr/gitversion docker

push_docker:
	docker push omidmr/gitversion

cargo_publish:
	cargo publish

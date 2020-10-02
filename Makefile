build_docker:
	docker build -f docker/Dockerfile --pull -t omidmr/gitversion .

push_docker:
	docker push omidmr/gitversion

cargo_publish:
	cargo publish

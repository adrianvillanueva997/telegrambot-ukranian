docker-lint:
	hadolint --ignore DL3008 Dockerfile

lint:  docker-lint

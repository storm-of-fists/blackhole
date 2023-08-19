SHELL := /bin/bash

# Build containers with bazel, loads them locally.
build:
	cd ./minecraft_backup_worker; make build
	cd ./rest_api; make build

# Push built containers to the remote repo.
push:
	cd ./minecraft_backup_worker; make push
	cd ./rest_api; make push

up: build
	docker compose -f docker-compose.yml up -d --build

up_dev: build
	docker compose -f docker-compose.yml --verbose up --build

stop:
	docker stop $$(docker ps -q)

clean:
	docker system prune
	docker rm -vf $(docker ps -a -q)

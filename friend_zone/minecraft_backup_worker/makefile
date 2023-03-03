SHELL := /bin/bash

build:
	# kinda weird, but --norun keeps bazel from running the container.
	# this will create a tagged image, but not push it or anything.
	bazel run :minecraft_backup_worker -- --norun

up: build
	docker compose -f docker-compose.yml up -d

up_dev: build
	docker compose -f docker-compose.yml --verbose up

stop:
	docker stop $$(docker ps -q)

clean:
	docker system prune
	docker rm -vf $(docker ps -a -q)

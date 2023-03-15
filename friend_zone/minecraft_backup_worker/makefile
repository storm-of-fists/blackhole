SHELL := /bin/bash

build:
	# kinda weird, but --norun keeps bazel from running the container.
	# this will create a tagged image, but not push it or anything.
	bazel run :minecraft_backup_worker -- --norun
	docker tag bazel/friend_zone/minecraft_backup_worker:minecraft_backup_worker contron44/minecraft_backup_worker:latest

push: build
	docker push contron44/minecraft_backup_worker:latest

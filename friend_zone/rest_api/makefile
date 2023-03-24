SHELL := /bin/bash

build:
	# kinda weird, but --norun keeps bazel from running the container.
	# this will create a tagged image, but not push it or anything.
	bazel run :rest_api.image -- --norun
	docker tag bazel/friend_zone/rest_api:rest_api.image contron44/friend_zone_rest_api:latest

push: build
	docker push contron44/friend_zone_rest_api:latest

SHELL := /bin/bash

up:
	docker compose -f docker-compose.yml up -d --build

up_dev:
	docker compose -f docker-compose.yml --verbose up --build

stop:
	docker stop $$(docker ps -q)

clean:
	docker system prune
	docker rm -vf $(docker ps -a -q)

PYTHON = python3
PLANET_DATA = https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de440.bsp
TOP := $(dir $(firstword $(MAKEFILE_LIST)))
SHELL := /bin/bash

help:
	@echo "------------HELP------------"
	@echo "You don't need my help lol."
	@echo "----------------------------"

venv:
	test -d venv || ${PYTHON} -m venv venv

install: venv
	source venv/bin/activate && \
	pip install -r requirements.txt

reqs:
	source venv/bin/activate && \
	touch requirements.txt && \
	pip freeze > requirements.txt

test:
	@echo "lol who tests their code?"

run:
	$(PYTHON) --version

data:
	@cd "$(TOP)src/instantiation_data/" && \
	if [ ! -f "de440.bsp" ]; then \
		wget $(PLANET_DATA); \
	else \
		echo "Data file has already been downloaded."; \
	fi;\

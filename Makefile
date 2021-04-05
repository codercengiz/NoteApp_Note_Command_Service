SHELL := /bin/bash

.PHONY: generate-all generate-eventstore generate-kafka delete-all

generate-all:generate-network generate-eventstore generate-kafka

delete-all: delete-kafka delete-eventstore delete-networks

generate-eventstore:
	@docker-compose -f docker-compose-for-eventstore.yaml up -d

generate-kafka:
	@docker-compose -f docker-compose-for-kafka.yaml up -d

generate-network:
	@docker network create note-app || true

delete-eventstore:
	@docker-compose -f docker-compose-for-eventstore.yaml down

delete-kafka:
	@docker-compose -f docker-compose-for-kafka.yaml down

delete-networks:
	@docker network ls --filter name=note-app -q | xargs docker network rm

run:
	@cargo run
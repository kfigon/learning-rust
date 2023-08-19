start_db:
	docker-compose up -d

stop_db:
	docker-compose down

migrate:
	cat ./schema.sql | docker exec -i todo_app_database psql -U postgres -d postgres

connect_to_db:
	docker exec -it todo_app_database psql -U postgres -d postgres
# \dt to list tables

test:
	cargo t
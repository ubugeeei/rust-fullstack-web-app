MIGRATE_ARG = create_todos

build:
	cargo build

start:
	cargo run &

stop:
	lsof -i:8080 -t | xargs kill

orm_setup:
	mkdir -p db && diesel setup --database-url ./db/sample.db

orm_generate:
	mkdir -p db && diesel migration generate ${MIGRATE_ARG} --database-url ./db/sample.db

orm_migrate:
	mkdir -p db && diesel migration run --database-url ./db/sample.db

orm_revert:
	mkdir -p db && diesel migration revert --database-url ./db/sample.db

orm_remigrate:
	make orm_revert
	make orm_migrate

test:
	curl -X POST -H "Content-Type: application/json" -d '{"query": "mutation {createTodo(title: \"testtodo\", description: \"\")}"}' http://127.0.0.1:4000 | jq
	curl -X POST -H "Content-Type: application/json" -d '{"query": "query  {getTodos {id\ntitle\ndescription\nisDone}}"}' http://127.0.0.1:4000 | jq
start:
	trunk serve --open &

stop:
	lsof -i:8080 -t | xargs kill

codegen:
	graphql-client generate --schema-path src/graphql/schema/schema.graphql src/graphql/schema/get_todos.graphql

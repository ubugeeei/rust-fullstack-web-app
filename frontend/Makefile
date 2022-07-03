start:
	trunk serve --open &

stop:
	lsof -i:8080 -t | xargs kill

SCHEMA = get_todo
codegen:
	graphql-client generate --schema-path src/graphql/schema/schema.graphql src/graphql/schema/$(SCHEMA).graphql

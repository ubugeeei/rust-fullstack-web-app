start:
	trunk serve --open &

stop:
	lsof -i:8080 -t | xargs kill

SCHEMA = hoge
codegen:
	graphql-client generate --schema-path src/graphql/schema.graphql src/graphql/$(SCHEMA).graphql

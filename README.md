### local env deps

- cargo
- diesel cli
- trunk cli
- sqlite
- graphql-client cli (dev)

### usage

```sh
# initial start
$ make initial_start

# From the second time onwards
$ make start_app
# or
$ make stop_app
```

### stacks

- frontend

  - [yew](https://yew.rs/)
  - [graphql_client](https://docs.rs/graphql_client/latest/graphql_client/)
  - [reqwest](https://docs.rs/reqwest/latest/reqwest/)

- backend
  - [diesel](https://diesel.rs/)
  - [actix-web](https://actix.rs/)
  - [async-graphql](https://async-graphql.github.io/async-graphql/en/index.html)

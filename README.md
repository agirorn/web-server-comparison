# The web server tests

The results are in [RESULTS.md](./RESULTS.md)

# The web servers

Each webserver responds to to endpoints.
- `/hello-world': That returns the "hello world" text
- `/save': That saves a small event in a postgres database

# the load test

```bash
cassowary run -u http://localhost:3000/hello-world -c 100 -n 100000 --histogram --boxplot
cassowary run -u http://localhost:3000/save -c 100 -n 100000 -H "Content-Type: application/json" --postfile post.json --histogram --boxplot
```

```bash
curl -X GET http://localhost:3000/hello-world
curl -H "Content-Type: application/json" -d "@post.json" -X POST http://localhost:3000/save
```

## k6 load test

```shell
k6 run k6-hello-world.js
k6 run k6-save.js
```

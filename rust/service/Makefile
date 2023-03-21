
dev-docker:
	yardman \
		Makefile \
		Dockerfile \
		package.json \
		src \
		'clear && make build'


build:
	docker build -t nodejs-service .
	docker run -it nodejs-service ls -R

shell:
	docker run -it nodejs-service sh

start:
	docker compose --project-name rust up

clean:
	docker system prune --volumes --force


hellow-world:
	curl -X GET http://localhost:3000/hello-world

save:
	curl -H "Content-Type: application/json" -d "@../../post.json" -X POST http://localhost:3000/save

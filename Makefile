run:
	docker start rust-http

stop:
	docker stop rust-http

exec:
	docker exec -it rust-http /bin/bash
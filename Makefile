initial_start:
	cd backend && make orm_setup && make orm_migrate
	make start_app

start_app:
	cd backend && make start
	sleep 5s
	cd frontend && make start

stop_app:
	cd backend && make stop
	cd frontend && make stop

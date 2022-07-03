start_app:
	cd backend && make start
	cd frontend && make start

stop_app:
	cd backend && make stop
	cd frontend && make stop

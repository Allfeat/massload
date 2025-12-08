# MassLoad - Build helpers
# Usage: make <target>

.PHONY: help docker-backend docker-frontend docker-all compose-up compose-down clean

help:
	@echo "MassLoad Docker Build Commands"
	@echo ""
	@echo "  make docker-backend   Build backend Docker image"
	@echo "  make docker-frontend  Build frontend Docker image"
	@echo "  make docker-all       Build all Docker images"
	@echo "  make compose-up       Start with docker-compose"
	@echo "  make compose-down     Stop docker-compose"
	@echo ""

docker-backend:
	docker build -f backend/Dockerfile -t massload-backend .

docker-frontend:
	docker build -f frontend/Dockerfile -t massload-frontend .

docker-all: docker-backend docker-frontend

compose-up:
	docker-compose up --build

compose-down:
	docker-compose down

clean:
	docker-compose down -v --rmi local 2>/dev/null || true


default:
    @echo "Available commands:"
    @echo "  build"
    @echo "  run"
    @echo "  test"
    @echo "  docker-build"
    @echo "  docker-up"
    @echo "  docker-down"

build:
    @echo "Building services..."
    @(cd app-service && cargo build)
    @(cd auth-service && cargo build)

run:
    @echo "Running services..."
    @echo "Starting auth-service..."
    @(cd auth-service && cargo watch -q -c -w src/ -w assets/ -x run &)
    @echo "Starting app-service..."
    @(cd app-service && cargo watch -q -c -w src/ -w assets/ -w templates/ -x run)

test:
    @echo "Testing services..."
    @(cd app-service && cargo test)
    @(cd auth-service && cargo test)

docker-build:
    @echo "Building Docker images..."
    @docker compose build

docker-up:
    @echo "Starting Docker containers..."
    @docker compose up

docker-down:
    @echo "Stopping Docker containers..."
    @docker compose down
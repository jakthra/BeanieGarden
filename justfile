default:
    just --list

[working-directory('core')]
db-seed:
    echo "Seeding database with data..."
    cargo run -p db_setup

docker-postgres:
    podman run -d --replace \
          --name postgres \
          --restart unless-stopped \
          -e POSTGRES_PASSWORD=postgres \
          -e POSTGRES_USER=postgres \
          -e POSTGRES_DB=beaniegenie \
          -v ~/postgres-data:/var/lib/postgresql/data \
          -p 5432:5432 \
          docker.io/postgres:18-alpine

[working-directory('frontend')]
db-migrate:
    echo "Running database migrations..."
    bun run db:migrate

[working-directory('frontend')]
db-generate:
    echo "Generating database schema..."
    bun run db:generate

[working-directory('core')]
db-api-entities:
    echo "Generating database entities..."
    sea-orm-cli generate entity --with-serde both -o entity/src

[working-directory('frontend')]
dev:
    echo "Starting development server..."
    bun run dev

[working-directory('frontend')]
add-admin:
    echo "Add admin user..."
    bun run db:add-admin admin@beaniegenie.com beaniegenie

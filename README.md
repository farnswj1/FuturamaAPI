# Futurama API
This is an API that provides data on Futurama, the TV series.

## Setup
The project uses the following:
- Rust
- Axum
- PostgreSQL
- Docker
- Docker Compose

For additional information on project specifications, see the ```Cargo.toml``` file in ```app```.

### Setting up PostgreSQL
In the ```postgres/``` directory, create a ```.env``` file
that contains the following environment variables:
```
POSTGRES_DB=futuramaapi
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
```

### Setting up the API
In the ```app/``` directory, create a ```.env``` file
that contains the following environment variables:
```
CORS_ALLOWED_ORIGINS="http://localhost"
DATABASE_URL="postgresql://postgres:password@postgres:5432/futuramaapi"
```

## Building
The project uses Docker. Ensure Docker and Docker Compose are installed before continuing.

To build, run ```docker compose build```

## Running
To run the web API, run ```docker compose up -d```, then go to http://localhost using your web browser.

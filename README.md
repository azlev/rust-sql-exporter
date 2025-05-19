# Rust SQL Exporter

This is a prometheus exporter that does queries in a PostgreSQL or SQL Server databases and expose as metrics.

Based on PostgreSQL 17.

## Why another exporter?

An exporter is a short project and I found it a good way to learn rust.

Also it mix 2 different behaviours:

* Synchronous queries: suitable for most monitoring queries. They are executed during the request
* Interval-based queries: suitable for long queries, usually business queries. They are executed in
  a separated context and merged with the synchronous queries

## Building

```
cargo build                        # just postgres support
```
or
```
cargo build --features=mssql  # both postgres and SQL Server support
```

## How to use it

```
export RSE_CONNINFO="host=127.0.0.1 user=postgres password=postgres"
export RSE_CONFIG=queries_postgres.yaml
export RSE_ADDRESS=0.0.0.0:3000
./rust-sql-exporter
```
or
```
export RSE_CONNINFO="server=tcp:localhost,1433;IntegratedSecurity=false;User ID=sa;Password=A@1mssql TrustServerCertificate=true"
export RSE_CONFIG=queries_mssql.yaml
export RSE_ADDRESS=0.0.0.0:3000
./rust-sql-exporter
```

The aim is to be simplistic, just two (RSE_ADDRESS is optional) environment variables and that's it.

## Customizing

To remove / add more metrics, edit the `queries` directory, adding more queries or removing existents.

The `cargo build` will concatenate all files inside `queries` directory in a file called `queries.yaml`.

you can also use `cat queries/*.yaml > queries.yaml` to achieve the same result. Just be careful to insert a newline on the last line.

The directory is organized in this way:

```
queries
   |-postgres
   |   |-global.yaml (metrics that query pg_global objects(*))
   |   |-tables.yaml (metrics from pg_stat_user_tables)
   |   |-indexes.yaml (metrics from pg_stat_user_indexes)
   |   `-interval.yaml (template to insert interval-based queries)
   `-mssql
       `-global.yaml (metrics for databases like connections)


(*): pg_global: SELECT relname
                  FROM pg_class
                 WHERE reltablespace = (SELECT oid FROM pg_tablespace WHERE spcname = 'pg_global')
```

## Building the image

At work, we ditched Docker to use podman, so it's part of the experiment to use podman:

`podman build . -f podman/Containerfile -t rust-sql-exporter:latest`

## Running locally

```
cargo build
podman kube play --replace podman/manifest.yaml
```

`cargo build` will use the `kubectl` command to create a configmap based on `queries.yaml`.
After that you can run locally with podman.

# Rust SQL Exporter

This is a prometheus exporter that does queries in a PostgreSQL or SQL Server databases and expose as metrics.

Based on PostgreSQL 17.

(Maybe one day I'll really do the SQL Server part).

## Why another exporter?

An exporter is a short project and I found it a good way to learn rust.

## How to use it

```
export RSE_CONNINFO="host=127.0.0.1 user=postgres password=postgres"
export RSE_CONFIG=queries.yaml
./rust-sql-exporter
```

The aim is to be simplistic, just two environment variables and that's it.

## Customizing

To remove / add more metrics, edit the `queries` directory, adding more queries or removing existents.

The `cargo build` will concatenate all files inside `queries` directory in a file called `queries.yaml`.

you can also use `cat queries/*.yaml > queries.yaml` to achieve the same result. Just be careful to insert a newline on the last line.

The directory is organized in this way:

```
queries
   |-global.yaml (metrics that query pg_global objects*)
   |-tables.yaml (metrics from pg_stat_user_tables)
   `-indexes.yaml (metrics from pg_stat_user_indexes)

*: pg_global: SELECT relname
              FROM pg_class
              WHERE reltablespace = (SELECT oid FROM pg_tablespace WHERE spcname = 'pg_global')
```

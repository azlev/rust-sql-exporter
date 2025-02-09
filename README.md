# Rust SQL Exporter

This is a prometheus exporter that does queries in a PostgreSQL or SQL Server databases and expose as metrics.

## Why another exporter?

An exporter is a short project and I found it a good way to learn rust.

## How to use it

```
export RSE_CONNINFO="host=127.0.0.1 user=postgres password=postgres"
export RSE_CONFIG=sql.yaml
./rust-sql-exporter
```

The aim is to be simplistic, just two environment variables and that's it.

## Gathering more or less metrics

TBD

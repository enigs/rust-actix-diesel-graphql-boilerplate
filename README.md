# Sample Actix-Web + Async-Graphql + Diesel Server

- [x] Actix-Web
- [x] Async-Graphql
- [X] SSE enabled
- [x] Diesel
- [x] Postgres
    - [x] Postgis Plugin

This is my simple boilerplate setup for actix-web + async-graphql + diesel + postgres (w/ Postgis plugin). I'm using this as a base for my future projects.

**Note:** Make sure to check how I generate random keys for master via `backend > library > ciphers`. I would probably create a cli generator for controller's key generation and bearer token pairs.
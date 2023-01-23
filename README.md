# Graphical interface to Animal Database.

Project is divided into 2 main parts.
- Backend contained inside single Docker Container
- Frontend as wasm package served by backend.

Backend uses Postgres Sql to store it's state.
Backend aims to serve 
  - REST API to fetch and update Backend Data 
  - Files required for serving frontend website properly. 



Tech Stack
  - Backend
    - Docker
    - Postgres SQL
    - Rust
      - Rocket web server
      - Sqlx sql connection framework
      - plotters graphic library
  - Frontend 
    - Rust
      - Yew frontend framework
      - Reqwest http framework
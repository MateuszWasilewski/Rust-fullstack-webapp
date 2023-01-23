# Graphical interface to Animal Database.

### Structure
Project is divided into 2 main parts.
- Backend contained inside single Docker Container
- Frontend as wasm package served by backend.

Backend uses Postgres Sql to store it's state.
Backend aims to serve 
  - REST API to fetch and update Backend Data 
  - Files required for serving frontend website properly. 
Data is exchanged using HTTP with Json between frontend and Backend 


### Tech Stack
  - Backend
    - Docker
    - Postgres SQL
    - Rust
      - Rocket web server
      - SQLx sql connection framework
      - plotters graphic library
  - Frontend 
    - Rust
      - Yew frontend framework
      - Reqwest http framework

### DB connection

SQLx framework
  - provides simple way to create connection pool to postgresDB.
  - applies any new migrations during app startup.
  - sanitizes all queries to conform to DB schema during app build.
  - prevents SQLX injection (in depth testing is required).

All DB operations are divided whether they 
  - SELECT
  - DELETE
  - UPDATE
Data in DB.

### Functionality
Currently any user on page can
  - Add animal
  - Add Litter
  - Fetch data about all animals
    - fetches only simplified data in vector
  - Fetch all litter data with genotypes from DB.
  - Fetch all data about specified animal
    - Show more animal data
    - Fetch all photos for specified animal
    - Generate and fetch image depicting animal ancestry up to some depth.  
      Could be refactored.
  - Fetch list of litters.
  - Fetch list of animals in litter.
  - Search bar
    - Currently does a regex search on all animals trying to match
      - animal id
      - phenotype
    - Could be implemented better but as of right now it will do.


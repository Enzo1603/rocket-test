# Notes 

## Docker & Postgresql

connect to postgresql shell: docker exec -it 9a327dd3a07f psql -U username postgr_db;

connect to database: \c;

list the relations: \d;

SELECT * FROM ...;

set DATABASE_URL=postgres://username:password@localhost:5432/postgr_db;

(UUID: c33fc4df-1ac0-4b7c-9577-bd9750055d0d, user123, user123@email.com)

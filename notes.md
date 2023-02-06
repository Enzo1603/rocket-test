# Notes 

## Docker & Postgresql

connect to postgresql shell: docker exec -it fe00a0231a23 psql -U username postgr_db;

connect to database: \c;

list the relations: \d;

SELECT * FROM ...;

set DATABASE_URL=postgres://username:password@localhost:5432/postgr_db;

(3e3dd4ae-3c37-40c6-aa64-7061f284ce28, John Doe, 18, 1, true)

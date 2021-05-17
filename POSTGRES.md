# event_poly

# implement a local db for consuming events
# source: https://onexlab-io.medium.com/docker-compose-mariadb-5eb7a37426a2
# create this docker compose file for a polymath instance
version: '3'

services:
  postgres:
    image: postgres:13.1
    healthcheck:
      test: [ "CMD", "pg_isready", "-q", "-d", "postgres", "-U", "root" ]
      timeout: 45s
      interval: 10s
      retries: 10
    restart: always
    environment:
      - POSTGRES_USER=root
      - POSTGRES_PASSWORD=password
      - APP_DB_USER=docker
      - APP_DB_PASS=docker
      - APP_DB_NAME=docker
    volumes:
      - ./db:/docker-entrypoint-initdb.d/
    ports:
      - 5432:5432

    adminer:
      image: adminer
      restart: always
      ports:
        - 8080:8080

# ####################################################

version: '3.1'

services:

  db:
    image: mariadb
    restart: always
    environment:
      MYSQL_ROOT_PASSWORD: polyadmin
      MYSQL_DATABASE: polydb
      MYSQL_USER: polyuser
      MYSQL_PASSWORD: polypass
    
    volumes: 
        - ./init:/docker-entrypoint-initdb.d

  adminer:
    image: adminer
    restart: always
    ports:
      - 8080:8080

# docker-compose commands
docker-compose up	
docker-compose up # from file directory
 Version: '10.5.10-MariaDB-1:10.5.10+maria~focal'  socket: '/run/mysqld/mysqld.sock'  port: 3306  mariadb.org binary distribution

# open adminer
localhost:8080

# SP-1 Backend

To run the backend perform the steps in "Setup, install and run django"

## Download and setup postgres

Download postgres app from https://postgresapp.com/downloads.html and run the command (MacOS):

    sudo mkdir -p /etc/paths.d && echo /Applications/Postgres.app/Contents/Versions/latest/bin | sudo tee /etc/paths.d/postgresapp

- Restart the terminal

- Open postgres.app
- Click initialize

In the backend/settings directory create a file called .env. The .env file should be in the same directory as base.py and dev.py

Add the data below to the .env file

    DB_NAME=your_system_username
    DB_USER=your_system_username
    DB_PASSWORD=
    DB_HOST=localhost  
    DB_PORT=5432

Exchange your_system_username with your mac username.

Make sure to also run:

    export PATH=$PATH:/Applications/Postgres.app/Contents/Versions/{your version}/bin

Then continue with the django setup.

## Setup, install and run django

#### Create a virtual environment

    make venv

#### Install the requirements

    make install

#### Then apply the migrations to the database

    make migrate

#### Run the server

    make run

## Show endpoints using NinjaAPI (OpenAPI)

    localhost:8000/api/docs#/

#### Show Django admin page

    localhost:8000/admin

#### To access admin page you have to create a superuser _Email address may be blank_

    make superuser

## Seeding database //TODO

This will populate the database with some dummy data

    make seed-db

## Black formatter (We should use this)

Black formatter should automatically be installed after,

    make venv and make install

To format the backend project run,

    make fmt

Note: migration files should not be formatted (and make fmt ensures of that)

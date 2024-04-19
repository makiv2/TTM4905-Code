-- Your SQL goes here
CREATE TABLE "proofs"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"proof" TEXT NOT NULL,
	"stdout_buffer_data" BYTEA NOT NULL
);

CREATE TABLE "users"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"password" VARCHAR NOT NULL,
	"message" TEXT NOT NULL
);


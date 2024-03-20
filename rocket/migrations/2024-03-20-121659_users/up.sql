-- Your SQL goes here
DROP TABLE IF EXISTS "posts";
CREATE TABLE "users"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"password" VARCHAR NOT NULL,
	"message" TEXT NOT NULL
);


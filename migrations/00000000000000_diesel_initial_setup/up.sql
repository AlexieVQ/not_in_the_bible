CREATE TABLE history (
    id VARCHAR PRIMARY KEY
);

CREATE TABLE requests (
    id VARCHAR PRIMARY KEY,
    "user" VARCHAR NOT NULL,
    "date" TIMESTAMP NOT NULL,
    op_id VARCHAR NOT NULL,
    op_author VARCHAR NOT NULL,
    "text" TEXT NOT NULL
);

CREATE TABLE responses (
    id VARCHAR PRIMARY KEY,
    body TEXT NOT NULL,
    "user" VARCHAR NOT NULL,
    "date" TIMESTAMP NOT NULL,
    op_id VARCHAR NOT NULL,
    op_author VARCHAR NOT NULL
);
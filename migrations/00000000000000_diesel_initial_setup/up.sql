CREATE TABLE history (
    id VARCHAR PRIMARY KEY
);

CREATE TABLE requests (
    id VARCHAR PRIMARY KEY,
    "user" VARCHAR NOT NULL,
    "date" TIMESTAMP NOT NULL,
    op_id VARCHAR UNIQUE NOT NULL,
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

CREATE FUNCTION already_computed(VARCHAR) RETURNS boolean AS $$
    SELECT COUNT(id) > 0 FROM (
        SELECT id FROM history WHERE id = $1 UNION ALL
        SELECT op_id AS id FROM responses WHERE op_id = $1) AS id
$$ LANGUAGE SQL;

ALTER TABLE requests
ADD CONSTRAINT op_already_computed
CHECK (already_computed(op_id) = FALSE);
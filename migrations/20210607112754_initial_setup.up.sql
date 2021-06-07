-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE ballots (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    cast_time TIMESTAMP WITH TIME ZONE
);

CREATE TABLE voorzitters (
    candidate VARCHAR PRIMARY KEY
);

CREATE TABLE ondervoorzitters (
    candidate VARCHAR PRIMARY KEY
);

CREATE TABLE penning_meesters (
    candidate VARCHAR PRIMARY KEY
);

CREATE TABLE secretarissen (
    candidate VARCHAR PRIMARY KEY
);

CREATE TABLE votes (
    ballot_id uuid REFERENCES ballots(id) UNIQUE NOT NULL,
    voorzitter VARCHAR REFERENCES voorzitters(candidate) NOT NULL,
    ondervoorzitter VARCHAR REFERENCES ondervoorzitters(candidate) NOT NULL,
    penning_meester VARCHAR REFERENCES penning_meesters(candidate) NOT NULL,
    secretaris VARCHAR REFERENCES secretarissen(candidate) NOT NULL
);

-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE ballots (
    id uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    cast_time TIMESTAMP WITH TIME ZONE
);

CREATE TABLE voorzitters (
    name VARCHAR PRIMARY KEY
);

CREATE TABLE ondervoorzitters (
    name VARCHAR PRIMARY KEY
);

CREATE TABLE penning_meesters (
    name VARCHAR PRIMARY KEY
);

CREATE TABLE secretarissen (
    name VARCHAR PRIMARY KEY
);

CREATE TABLE votes (
    ballot_id uuid REFERENCES ballots(id) UNIQUE NOT NULL,
    voorzitter VARCHAR REFERENCES voorzitters(name) NOT NULL,
    ondervoorzitter VARCHAR REFERENCES ondervoorzitters(name) NOT NULL,
    penning_meester VARCHAR REFERENCES penning_meesters(name) NOT NULL,
    secretaris VARCHAR REFERENCES secretarissen(name) NOT NULL
);

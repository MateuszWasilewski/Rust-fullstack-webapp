-- Add up migration script here

CREATE TABLE ANIMAL (
    id TEXT PRIMARY KEY,
    gender_male BOOLEAN NOT NULL,
    status VARCHAR(20),
    eye_color VARCHAR(20),
    hair VARCHAR(20)
);

CREATE TABLE LITTER (
    id TEXT PRIMARY KEY,
    mother TEXT NOT NULL REFERENCES ANIMAL,
    father TEXT NOT NULL REFERENCES ANIMAL
);

ALTER TABLE ANIMAL
ADD COLUMN litter VARCHAR(10) REFERENCES LITTER;
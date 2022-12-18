-- Add up migration script here

CREATE TABLE PHENOTYPE (
    id TEXT PRIMARY KEY,
    phenotype_variant TEXT NOT NULL
);

ALTER TABLE ANIMAL
ADD COLUMN phenotype TEXT REFERENCES PHENOTYPE;
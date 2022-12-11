-- Add down migration script here

ALTER TABLE ANIMAL
DROP COLUMN phenotype;

DROP TABLE PHENOTYPE;
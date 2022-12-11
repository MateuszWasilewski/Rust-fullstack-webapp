-- Add down migration script here

ALTER TABLE ANIMAL
DROP COLUMN litter;

DROP TABLE LITTER;

DROP TABLE ANIMAL;

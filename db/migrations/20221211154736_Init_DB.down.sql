-- Add down migration script here

DROP TABLE LITTER_PHOTO;
DROP TABLE ANIMAL_PHOTO;
DROP TABLE PHOTO;

ALTER TABLE ANIMAL
DROP COLUMN litter;

DROP TABLE LITTER;

DROP TABLE ANIMAL;

DROP TABLE GENOTYPE;

DROP TABLE PHENOTYPE;

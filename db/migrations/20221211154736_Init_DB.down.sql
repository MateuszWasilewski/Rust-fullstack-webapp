-- Add down migration script here

SET session_replication_role = 'replica';

DROP TABLE LITTER_PHOTO;
DROP TABLE ANIMAL_PHOTO;
DROP TABLE PHOTO;

ALTER TABLE ANIMAL
DROP COLUMN litter;

DROP TABLE LITTER;

DROP TABLE ANIMAL;

DROP TABLE GENOTYPE;

DROP TABLE PHENOTYPE;

SET session_replication_role = 'origin';
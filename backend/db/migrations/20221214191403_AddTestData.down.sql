-- Add down migration script here

SET session_replication_role = 'replica';

DELETE FROM ANIMAL_PHOTO;

DELETE FROM LITTER_PHOTO;

DELETE FROM PHOTO;

DELETE FROM ANIMAL;

DELETE FROM LITTER;

DELETE FROM GENOTYPE;

DELETE FROM PHENOTYPE;

SET session_replication_role = 'origin';
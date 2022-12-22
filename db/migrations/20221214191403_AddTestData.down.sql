-- Add down migration script here

SET session_replication_role = 'replica';

DELETE FROM ANIMAL;

DELETE FROM LITTER;

DELETE FROM PHENOTYPE;

SET session_replication_role = 'origin';
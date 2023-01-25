-- Add up migration script here

ALTER TABLE ANIMAL
ALTER COLUMN phenotype
DROP NOT NULL;

UPDATE ANIMAL 
SET phenotype = NULL 
WHERE phenotype = 'nieznany';

DELETE FROM PHENOTYPE
WHERE name = 'nieznany';
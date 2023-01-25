-- Add down migration script here

INSERT INTO PHENOTYPE (name, variant)
VALUES ('nieznany', 'nieznany');

UPDATE ANIMAL 
SET phenotype = 'nieznany' 
WHERE phenotype IS NULL;

ALTER TABLE ANIMAL
ALTER COLUMN phenotype
SET NOT NULL;

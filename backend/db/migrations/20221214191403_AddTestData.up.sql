-- Add up migration script here

INSERT INTO phenotype (name, variant)
  VALUES ('nieznany', 'nieznany');

--INSERT INTO ANIMAL (id, gender_male, status, phenotype)
--VALUES ('28.M3', True, 'adopted', 'nieznany'),
--       ('24.F4', False, 'alive', 'nieznany');
--
--INSERT INTO LITTER (id, mother, father)
--VALUES ('1', '24.F4', '28.M3'),
--       ('65', '24.F4', '28.M3');
----
--INSERT INTO ANIMAL (id, gender_male, status, phenotype, litter)
--VALUES ('1.M1', True, 'alive', 'nieznany', '1'),
--       ('1.M2', True, 'alive', 'nieznany', '1'),
--       ('1.F3', False, 'alive', 'nieznany', '1'),
--       ('1.F4', False, 'alive', 'nieznany', '1'),
--       ('65.F5', False, 'alive', 'nieznany', '65');
--
--INSERT INTO PHOTO (path)
--VALUES ('65.F5.jpg');
--
--INSERT INTO ANIMAL_PHOTO (animal, photo)
--VALUES ('65.F5', '65.F5.jpg');
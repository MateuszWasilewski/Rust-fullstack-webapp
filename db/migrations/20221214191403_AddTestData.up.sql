-- Add up migration script here

INSERT INTO phenotype (name, variant)
VALUES ('agouti', 'ticked'),
       ('dove', 'self'),
       ('chocolate fox', 'self'),
       ('american brindle', 'marked');

INSERT INTO ANIMAL (id, gender_male, status, phenotype)
VALUES ('28.M3', True, 'adopted', 'agouti'),
       ('24.F4', False, 'alive', 'dove');

INSERT INTO LITTER (id, mother, father)
VALUES ('1', '24.F4', '28.M3');

INSERT INTO ANIMAL (id, gender_male, status, phenotype, litter)
VALUES ('1.M1', True, 'alive', 'agouti', '1'),
       ('1.M2', True, 'alive', 'american brindle', '1'),
       ('1.F3', False, 'alive', 'dove', '1'),
       ('1.F4', False, 'alive', 'agouti', '1');
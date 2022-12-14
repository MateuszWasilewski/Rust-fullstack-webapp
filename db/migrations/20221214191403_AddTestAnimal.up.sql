-- Add up migration script here

INSERT INTO phenotype (id, phenotype_name, phenotype_variant)
VALUES ('agouti splashed', '', ''),
       ('dark splashed LH', '', ''),
       ('burmese LM', '', '');

INSERT INTO ANIMAL (id, gender_male, status, phenotype)
VALUES ('28.M3', True, 'adopted', 'dark splashed LH'),
       ('24.F4', False, 'alive', 'burmese LM');
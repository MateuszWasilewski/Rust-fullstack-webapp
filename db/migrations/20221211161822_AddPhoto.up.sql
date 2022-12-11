-- Add up migration script here

CREATE TABLE PHOTO (
    id SERIAL PRIMARY KEY,
    path TEXT NOT NULL,
    taken_date DATE
);

CREATE TABLE ANIMAL_PHOTO (
    animal VARCHAR(10) NOT NULL REFERENCES ANIMAL,
    photo SERIAL NOT NULL REFERENCES PHOTO
);

CREATE TABLE LITTER_PHOTO (
    litter VARCHAR(10) NOT NULL REFERENCES LITTER,
    photo SERIAL NOT NULL REFERENCES PHOTO
);

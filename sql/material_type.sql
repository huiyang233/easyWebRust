create table material_type
(
    id     bigint       not null
        primary key,
    name   varchar(255) not null,
    is_del tinyint(1)   not null
);

INSERT INTO material_type (id, name, is_del) VALUES (9207732047501688833, 'PLA', 0);
INSERT INTO material_type (id, name, is_del) VALUES (9207732049105780736, 'PEGT', 0);

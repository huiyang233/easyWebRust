create table material_color
(
    id     bigint      not null
        primary key,
    name   varchar(40) not null,
    color  varchar(10) not null comment '颜色 #xxx',
    is_del tinyint(1)  not null
)
    comment '材料颜色';

INSERT INTO material_color (id, name, color, is_del) VALUES (9207734891484872705, '红色', '#ff0000', 0);

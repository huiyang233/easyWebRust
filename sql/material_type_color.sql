create table material_type_color
(
    type_id  bigint     not null,
    color_id int        not null,
    price    decimal(2) not null comment '每克价钱',
    balance  float      not null comment '剩余多少克'
);

create index material_type_color_color_id_index
    on material_type_color (color_id);

create index material_type_color_type_id_index
    on material_type_color (type_id);


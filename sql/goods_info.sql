create table goods_info
(
    id          bigint       not null
        primary key,
    is_del      bit          not null,
    create_time datetime     not null,
    update_time datetime     not null,
    create_by   varchar(100) not null,
    update_by   varchar(100) not null,
    name        varchar(200) not null comment '商品名字',
    category_id bigint       null comment '类别id',
    main_image  varchar(300) null comment '主图',
    description varchar(500) null,
    status      int          not null comment '0=编辑 1=上架 2=下架'
)
    comment '商品表';

INSERT INTO goods_info (id, is_del, create_time, update_time, create_by, update_by, name, category_id, main_image, description, status) VALUES (1, false, '2024-09-05 16:21:28', '2024-09-05 16:21:30', 'admin', 'admin', '扫地机器人', null, null, '这是一个简短的简介', 0);

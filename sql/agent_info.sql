create table if not exists agent_info
(
    id          bigint       not null
    primary key,
    name        varchar(100) not null comment '代理商名字',
    city_id     bigint       not null comment '代理城市id',
    owner       bigint       null comment '代理的拥有者 存用户id',
    id_del      bit          null,
    create_by   varchar(20)  not null,
    update_by   varchar(20)  not null,
    create_time datetime     not null,
    update_time datetime     not null
    )
    comment '代理表';


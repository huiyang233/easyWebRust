create table sys_role
(
    id          bigint       not null
        primary key,
    create_by   varchar(32)  not null,
    create_time datetime(6)  not null,
    is_del      bit          not null,
    update_by   varchar(32)  not null,
    update_time datetime(6)  not null,
    name        varchar(255) not null
);

INSERT INTO sys_role (id, create_by, create_time, is_del, update_by, update_time, name) VALUES (1, 'admin', '2024-09-03 10:48:02.000000', false, 'admin', '2024-09-03 10:48:07.000000', '管理员');

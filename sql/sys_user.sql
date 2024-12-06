create table sys_user
(
    id             bigint       not null
        primary key,
    name           varchar(30)  null,
    user_name      varchar(20)  null,
    phone_number   varchar(20)  null,
    password       varchar(64)  null,
    enable         bit          null comment '是否启用',
    gender         int          null comment '1=男 0=女',
    is_del         bit          not null,
    is_super_admin tinyint(1)   not null comment '是否超级管理员',
    create_time    datetime     not null,
    update_time    datetime     not null,
    create_by      varchar(30)  not null,
    update_by      varchar(30)  not null,
    avatar         varchar(500) null
);

INSERT INTO sys_user (id, name, user_name, phone_number, password, enable, gender, is_del, is_super_admin, create_time, update_time, create_by, update_by, avatar) VALUES (1, 'admin', 'admin', '13402152243', '25d55ad283aa400af464c76d713c07ad', true, 1, false, 1, '2024-03-26 16:33:47', '2024-07-17 14:35:41', 'System', 'admin', 'https://localhost:5800/file/image/2024-03-28/0dbd756b-355b-4093-85be-302154cec2d6.png');

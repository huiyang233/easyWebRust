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

INSERT INTO sys_user (id, name, user_name, phone_number, password, enable, gender, is_del, is_super_admin, create_time, update_time, create_by, update_by, avatar) VALUES (1, 'admin', 'admin', '13402152243', 'e807f1fcf82d132f9bb018ca6738a19f', true, 1, false, 1, '2024-03-26 16:33:47', '2024-07-17 14:35:41', 'System', 'admin', 'https://localhost:5800/file/image/2024-03-28/0dbd756b-355b-4093-85be-302154cec2d6.png');
INSERT INTO sys_user (id, name, user_name, phone_number, password, enable, gender, is_del, is_super_admin, create_time, update_time, create_by, update_by, avatar) VALUES (12, '杨辉', '2409931477', '13402152243', '25f9e794323b453885f5181f1b624d0b', true, 1, false, 0, '2024-03-26 16:33:47', '2024-11-14 17:52:06', 'System', '2409931477', 'https://localhost:5800/file/image/2024-03-28/52205704-8d0d-4214-ba4b-740478321dca.jpeg');
INSERT INTO sys_user (id, name, user_name, phone_number, password, enable, gender, is_del, is_super_admin, create_time, update_time, create_by, update_by, avatar) VALUES (9207717861182439425, '杨辉', '123', '123', 'e807f1fcf82d132f9bb018ca6738a19f', true, null, false, 0, '2024-07-18 17:42:06', '2024-11-14 17:43:58', 'admin', 'admin', null);

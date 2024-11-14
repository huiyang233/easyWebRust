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
INSERT INTO sys_role (id, create_by, create_time, is_del, update_by, update_time, name) VALUES (1773595923142168578, 'hui.yang', '2024-03-29 14:19:54.553000', true, 'hui.yang', '2024-03-29 14:19:54.553000', '测试');
INSERT INTO sys_role (id, create_by, create_time, is_del, update_by, update_time, name) VALUES (9207491266222260225, 'hui.yang', '2024-04-29 16:49:57.089398', true, 'hui.yang', '2024-04-29 16:49:57.089409', '测试1');
INSERT INTO sys_role (id, create_by, create_time, is_del, update_by, update_time, name) VALUES (9207655554843410433, 'admin', '2024-06-26 17:31:27.920507', true, 'admin', '2024-07-19 16:54:11.969098', 'hello');
INSERT INTO sys_role (id, create_by, create_time, is_del, update_by, update_time, name) VALUES (9207850113608777729, 'admin', '2024-09-03 10:49:10.242266', true, 'admin', '2024-09-03 11:05:47.552012', '测试');
INSERT INTO sys_role (id, create_by, create_time, is_del, update_by, update_time, name) VALUES (9207850114040528896, 'admin', '2024-09-03 10:49:23.418269', true, 'admin', '2024-09-03 10:49:23.418273', '测试1');
INSERT INTO sys_role (id, create_by, create_time, is_del, update_by, update_time, name) VALUES (9208054787691479041, '2409931477', '2024-11-14 17:51:47.551832', false, '2409931477', '2024-11-14 17:51:54.901022', 'Test');

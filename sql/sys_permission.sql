create table sys_permission
(
    id          bigint       not null
        primary key,
    create_by   varchar(32)  not null,
    create_time datetime(6)  not null,
    update_by   varchar(32)  not null,
    update_time datetime(6)  not null,
    name        varchar(255) not null,
    value       varchar(255) not null,
    p_id        bigint       not null
);

INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (1, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '用户', 'user', 0);
INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (2, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '添加用户', 'user:add', 1);
INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (3, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '删除用户', 'user:del', 1);
INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (4, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '更新用户', 'user:update', 1);
INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (5, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '角色', 'role', 0);
INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (6, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '增加角色', 'role:add', 5);
INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (7, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '删除角色', 'role:del', 5);
INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (8, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '更新角色', 'role:update', 5);
INSERT INTO sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (9, 'system', '2024-01-26 09:35:26.000000', 'system', '2024-01-26 09:35:23.000000', '权限', 'permission', 0);

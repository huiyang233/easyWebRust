create table sys_permission
(
    id          bigint,
    create_by   varchar(32),
    create_time timestamp with time zone,
    update_by   varchar(32),
    update_time timestamp with time zone,
    name        varchar(255),
    value       varchar(255),
    p_id        bigint
);

alter table sys_permission
    owner to "2409931477";

INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (1, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '用户', 'user', 0);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (2, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '添加用户', 'user:add', 1);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (3, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '删除用户', 'user:del', 1);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (4, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '更新用户', 'user:update', 1);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (5, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '角色', 'role', 0);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (6, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '增加角色', 'role:add', 5);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (7, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '删除角色', 'role:del', 5);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (8, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '更新角色', 'role:update', 5);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (9, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '权限', 'permission', 0);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (10, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '日志', 'log', 0);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (11, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '黑名单', 'black_list', 0);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (12, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '增加黑名单', 'black_list:add', 11);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (13, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '删除黑名单', 'black_list:del', 11);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (14, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '更新黑名单', 'black_list:update', 11);
INSERT INTO public.sys_permission (id, create_by, create_time, update_by, update_time, name, value, p_id) VALUES (15, 'system', '2024-01-26 01:35:26.000000 +00:00', 'system', '2024-01-26 01:35:23.000000 +00:00', '更新黑名单配置', 'black_list_config:update', 11);


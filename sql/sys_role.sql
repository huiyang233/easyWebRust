create table sys_role
(
    id          bigint,
    create_by   varchar(32),
    create_time timestamp with time zone,
    is_del      boolean,
    update_by   varchar(32),
    update_time timestamp with time zone,
    name        varchar(255)
);

INSERT INTO public.sys_role (id, create_by, create_time, is_del, update_by, update_time, name) VALUES (1, 'admin', '2024-09-03 02:48:02.000000 +00:00', false, 'admin', '2024-09-03 02:48:07.000000 +00:00', '管理员');
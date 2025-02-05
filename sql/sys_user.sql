create table sys_user
(
    id             bigint,
    name           varchar(32),
    user_name      varchar(20),
    phone_number   varchar(20),
    password       varchar(64),
    enable         boolean,
    gender         integer,
    is_del         boolean,
    is_super_admin boolean,
    create_time    timestamp with time zone,
    update_time    timestamp with time zone,
    create_by      varchar(30),
    update_by      varchar(30),
    avatar         varchar(500),
    wx_open_id     varchar(100)
);


INSERT INTO public.sys_user (id, name, user_name, phone_number, password, enable, gender, is_del, is_super_admin, create_time, update_time, create_by, update_by, avatar, wx_open_id) VALUES (9208147484928704512, '123', '2409931477', '123', '25d55ad283aa400af464c76d713c07ad', true, null, false, false, '2024-12-16 19:40:02.731000 +00:00', '2025-01-06 06:30:38.045982 +00:00', 'admin', 'admin', null, null);
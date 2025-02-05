create table sys_log
(
    id          bigint,
    name        varchar(200),
    log_type    integer,
    description text,
    user_name   varchar(20),
    ip          varchar(64),
    create_time timestamp with time zone
);

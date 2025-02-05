create table black_list
(
    id          bigint,
    ip          varchar(100),
    create_time timestamp with time zone,
    is_del      boolean,
    reason      varchar(100),
    ban_time    timestamp with time zone
);



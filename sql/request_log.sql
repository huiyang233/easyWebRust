create table request_log
(
    id          bigint,
    uri         varchar(300),
    method      varchar(200),
    duration    bigint,
    ip          varchar(50),
    user_id     bigint,
    headers     text,
    query       text,
    create_time timestamp with time zone
);


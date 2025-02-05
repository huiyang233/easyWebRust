create table request_log
(
    id          bigint       not null
        primary key,
    uri         varchar(300) not null comment '访问路径',
    method      varchar(200) null,
    duration    bigint       null comment '毫秒',
    ip          varchar(50)  null,
    user_id     bigint       null comment '用户id',
    headers     text         null,
    query       text         null comment 'URL的参数',
    create_time datetime     null comment '访问的时间'
);

create index request_log_user_id_index
    on request_log (user_id);


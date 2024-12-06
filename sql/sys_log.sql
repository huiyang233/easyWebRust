create table sys_log
(
    id          bigint       not null
        primary key,
    name        varchar(200) not null comment '事件名字',
    log_type    int          not null comment '日志的类型 1=登录日志 2=删除日志 3=修改日志 4=其他日志',
    description text         not null,
    user_name   varchar(20)  not null comment '操作人的账号',
    ip          varchar(64)  not null,
    create_time datetime     not null comment '发生时间'
);

create index sys_log_create_time_index
    on sys_log (create_time);

create index sys_log_log_type_index
    on sys_log (log_type);

create index sys_log_user_name_index
    on sys_log (user_name);


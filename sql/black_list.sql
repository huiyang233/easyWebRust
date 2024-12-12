create table easy.black_list
(
    id          bigint       not null
        primary key,
    ip          varchar(100) not null comment 'ip地址，可能是ipv6所以得长点',
    create_time datetime     not null,
    is_del      bit          not null,
    reason      varchar(100) not null,
    ban_time    datetime     null
);


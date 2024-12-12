create table black_list_config
(
    id          bigint       not null
        primary key,
    ban_time    int unsigned not null comment '封禁时间 0=永久 >0=xx秒',
    `interval`  int unsigned not null comment '间隔 秒，每隔多少秒访问多少次',
    visit_count int unsigned not null comment '次数 每隔多少秒访问多少次'
);

INSERT INTO black_list_config (id, ban_time, `interval`, visit_count) VALUES (1, 0, 60, 1000);

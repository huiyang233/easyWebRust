create table merchant_info
(
    id           bigint       not null
        primary key,
    name         varchar(100) null comment '商家的名字',
    audit_status int          null comment '审核状态 1=待审核 2=已通过 3=Ban',
    owner        bigint       null comment '老板的用户id',
    agent_id     bigint       null comment '代理商的id',
    city_id      bigint       null comment '城市id',
    is_del       bit          null,
    create_by    varchar(20)  null,
    update_by    varchar(20)  null,
    create_time  datetime     null,
    update_time  datetime     null
)
    comment '商家表';


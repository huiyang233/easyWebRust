create table order_info
(
    id             bigint       not null
        primary key,
    type_id        bigint       not null,
    color_id       bigint       not null,
    weight         float        not null comment '重量有多少克',
    total_price    decimal(2)   not null,
    pay_type       int          null comment '支付方式 1=微信 2=支付宝',
    transaction_id varchar(128) null comment '交易id',
    paid_time      datetime     null comment '支付时间',
    create_time    datetime     not null comment '创建时间',
    status         int          not null comment '1=下单成功 2=等待排产 3=已生产完成 4=等待发货 5=已发货 6=已完成 7=退货中 8=已退货 9=已取消',
    model_file_id  bigint       not null comment '模型id'
);


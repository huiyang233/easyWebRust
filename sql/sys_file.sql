create table sys_file
(
    id          bigint,
    create_by   varchar(32),
    create_time timestamp with time zone,
    update_by   varchar(32),
    update_time timestamp with time zone,
    file_path   varchar(500),
    name        varchar(255),
    size        varchar(100),
    suffix      varchar(100),
    save_type   integer,
    url_path    varchar(500),
    md5         varchar(32),
    is_del      boolean
);

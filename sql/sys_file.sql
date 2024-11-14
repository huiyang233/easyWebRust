create table sys_file
(
    id          bigint       not null
        primary key,
    create_by   varchar(32)  null,
    create_time datetime(6)  null,
    is_del      bit          null,
    update_by   varchar(32)  null,
    update_time datetime(6)  null,
    file_path   varchar(500) null,
    name        varchar(255) null,
    size        varchar(100) null,
    suffix      varchar(100) null,
    type        varchar(200) null,
    url_path    varchar(500) null,
    md5         varchar(32)  null
);

INSERT INTO sys_file (id, create_by, create_time, is_del, update_by, update_time, file_path, name, size, suffix, type, url_path, md5) VALUES (1773019830752415746, 'hui.yang', '2024-03-28 00:10:43.423000', false, 'hui.yang', '2024-03-28 00:10:43.423000', '/Users/yanghui/Desktop/disk/image/2024-03-28/52205704-8d0d-4214-ba4b-740478321dca.jpeg', '07akioni.jpeg', '20042', 'jpeg', 'image/jpeg', 'http://localhost:8083/file/image/2024-03-28/52205704-8d0d-4214-ba4b-740478321dca.jpeg', 'a23b6ba2697baa86a0eb579485736a6a');
INSERT INTO sys_file (id, create_by, create_time, is_del, update_by, update_time, file_path, name, size, suffix, type, url_path, md5) VALUES (1773249480195878913, 'hui.yang', '2024-03-28 15:23:16.114000', false, 'hui.yang', '2024-03-28 15:23:16.114000', '/Users/yanghui/Desktop/disk/image/2024-03-28/18fc8a97-877a-45e5-a5b7-2a344cd68d53.jpg', '20240123173549_测试.jpg', '134826', 'jpg', 'image/jpeg', 'http://localhost:8083/file/image/2024-03-28/18fc8a97-877a-45e5-a5b7-2a344cd68d53.jpg', 'b205eed1ca745734404db7113addb988');
INSERT INTO sys_file (id, create_by, create_time, is_del, update_by, update_time, file_path, name, size, suffix, type, url_path, md5) VALUES (1773249643203309569, 'hui.yang', '2024-03-28 15:23:54.981000', false, 'hui.yang', '2024-03-28 15:23:54.981000', '/Users/yanghui/Desktop/disk/image/2024-03-28/0dbd756b-355b-4093-85be-302154cec2d6.png', 'efc61ce9d95702dfafcfc9f13e02e0d5.png', '417838', 'png', 'image/png', 'http://localhost:8083/file/image/2024-03-28/0dbd756b-355b-4093-85be-302154cec2d6.png', 'efc61ce9d95702dfafcfc9f13e02e0d5');
INSERT INTO sys_file (id, create_by, create_time, is_del, update_by, update_time, file_path, name, size, suffix, type, url_path, md5) VALUES (9207542016010616833, 'admin', '2024-05-17 15:02:37.720220', false, 'admin', '2024-05-17 15:02:37.720227', '/Users/yanghui/Desktop/disk/image/2024-05-17/ad45de51-4de0-4cf8-88eb-94f7cc1ea092.jar', 'memory-1.0.0.jar', '33947', 'jar', 'application/java-archive', 'http://localhost:8083/file/image/2024-05-17/ad45de51-4de0-4cf8-88eb-94f7cc1ea092.jar', '0882c54723b55b698412d124969e18e7');
INSERT INTO sys_file (id, create_by, create_time, is_del, update_by, update_time, file_path, name, size, suffix, type, url_path, md5) VALUES (9207542017709408257, 'admin', '2024-05-17 15:03:29.563067', false, 'admin', '2024-05-17 15:03:29.563080', '/Users/yanghui/Desktop/disk/image/2024-05-17/81b4d43d-1ad9-485f-a349-8842f8fdcfb8.jar', 'fttrAdapter.jar', '177646', 'jar', 'application/java-archive', 'http://localhost:8083/file/image/2024-05-17/81b4d43d-1ad9-485f-a349-8842f8fdcfb8.jar', 'dbbc6c32c212372f2c61c171a07d8100');
INSERT INTO sys_file (id, create_by, create_time, is_del, update_by, update_time, file_path, name, size, suffix, type, url_path, md5) VALUES (9207542021600903168, 'admin', '2024-05-17 15:05:28.322602', false, 'admin', '2024-05-17 15:05:28.322612', '/Users/yanghui/Desktop/disk/image/2024-05-17/18f85353-3970-4345-a21d-2e6327b12e8f.jpg', '1715219555571.jpg', '77150', 'jpg', 'image/jpeg', 'http://localhost:8083/file/image/2024-05-17/18f85353-3970-4345-a21d-2e6327b12e8f.jpg', '582db9b3494f7d625e4cf6db97505c86');
INSERT INTO sys_file (id, create_by, create_time, is_del, update_by, update_time, file_path, name, size, suffix, type, url_path, md5) VALUES (9207542118022512641, 'admin', '2024-05-17 15:54:30.876583', false, 'admin', '2024-05-17 15:54:30.876596', '/Users/yanghui/Desktop/disk/image/2024-05-17/6d7536c0-5404-4128-b49c-37a691388ff3.jpg', 'WechatIMG169 (1).jpg', '281810', 'jpg', 'image/jpeg', 'http://localhost:8083/file/image/2024-05-17/6d7536c0-5404-4128-b49c-37a691388ff3.jpg', 'da43d0d4d9f49f2cbbc5a8a2256c843d');
INSERT INTO sys_file (id, create_by, create_time, is_del, update_by, update_time, file_path, name, size, suffix, type, url_path, md5) VALUES (9207542119233191936, 'admin', '2024-05-17 15:55:07.823736', false, 'admin', '2024-05-17 15:55:07.823745', '/Users/yanghui/Desktop/disk/image/2024-05-17/3b332287-fcd5-4371-a6c7-5c0cc55fc696.jpg', 'WechatIMG169.jpg', '282777', 'jpg', 'image/jpeg', 'http://localhost:8083/file/image/2024-05-17/3b332287-fcd5-4371-a6c7-5c0cc55fc696.jpg', '42c3f73451ba6ae1b6cc942eb525160a');

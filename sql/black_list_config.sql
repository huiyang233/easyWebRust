create table black_list_config
(
    id          bigint,
    ban_time    integer,
    interval    integer,
    visit_count integer
);


INSERT INTO public.black_list_config (id, ban_time, interval, visit_count) VALUES (1, 0, 60, 1000);

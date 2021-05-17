-- This file should undo anything in `up.sql`
create table todos_dg_tmp
(
    id      varchar
        primary key not null,
    title   varchar,
    content varchar
);

insert into todos_dg_tmp(id, title, content)
select id, title, content
from todos;

drop table todos;

alter table todos_dg_tmp
    rename to todos;

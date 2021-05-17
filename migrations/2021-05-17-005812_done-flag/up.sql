-- Your SQL goes here
alter table todos
    add done smallint default 0 not null;

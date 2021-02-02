-- Your SQL goes here
create table todo (
    id serial primary,
    title varchar(255) not null,
    checked boolean not null default false
)
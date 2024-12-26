-- Your SQL goes here

create table characters (
    name text not null,
    owner bigint not null check (owner > 0),
    primary key (name, owner)
)
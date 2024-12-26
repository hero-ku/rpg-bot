-- Your SQL goes here

create table users (
    id bigint not null primary key,
    active_char text not null,
    foreign key (active_char, id) references characters(name, owner)
)
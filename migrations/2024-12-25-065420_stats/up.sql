-- Your SQL goes here

create table stats (
    char_name text not null,
    char_owner bigint not null,
    foreign key (char_name, char_owner) references characters(name, owner),

    name text not null,
    value integer not null,
    primary key (char_name, char_owner, name)
)
create table IF NOT EXISTS todos (
    id serial,
    user_id int not null,
    title varchar not null,
    description varchar not null,
    datecreated varchar not null,
    completed boolean,

    PRIMARY KEY(id),
    constraint fk_todo_user
        foreign key (user_id)
        references users(id)
);

create table IF NOT EXISTS users (
    id serial,
    name varchar not null,

    PRIMARY KEY(id)
);
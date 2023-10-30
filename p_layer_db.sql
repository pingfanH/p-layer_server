create table music_list
(
    user   varchar(50) not null,
    name   varchar(50) not null,
    date   varchar(50) not null,
    public tinyint(1)  not null
);

create table user_data
(
    user_token        varchar(50) not null,
    user_id           varchar(50) not null,
    user_account      varchar(50) not null,
    user_password     varchar(50) not null,
    user_name         varchar(50) not null,
    user_gender       varchar(50) not null,
    user_age          varchar(50) not null,
    user_info         varchar(50) not null,
    user_sign_date    varchar(50) not null,
    user_music_number int         not null,
    constraint user_account
        unique (user_account),
    constraint user_id
        unique (user_id),
    constraint user_name
        unique (user_name),
    constraint user_token
        unique (user_token)
);

create table user_play_list
(
    user   varchar(50) not null,
    name   varchar(50) not null,
    date   varchar(50) null,
    public tinyint(1)  not null
);


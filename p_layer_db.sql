
CREATE TABLE user_data (
   user_token VARCHAR(50) UNIQUE NOT NULL,
   user_id VARCHAR(50) UNIQUE NOT NULL,
   user_account VARCHAR(50) UNIQUE NOT NULL,
   user_password VARCHAR(50) NOT NULL,
   user_name VARCHAR(50) UNIQUE NOT NULL,
   user_gender VARCHAR(50) NOT NULL,
   user_age VARCHAR(50) NOT NULL,
   user_info VARCHAR(50) NOT NULL,
   user_sign_date VARCHAR(50) NOT NULL,
   user_music_number INT NOT NULL
);
CREATE TABLE music_list (
    user VARCHAR(50) NOT NULL,
    name VARCHAR(50) NOT NULL,
    date VARCHAR(50) NOT NULL,
    public BOOLEAN NOT NULLp_layer_db.sql
);


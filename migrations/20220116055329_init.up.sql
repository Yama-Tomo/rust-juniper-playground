-- Add up migration script here
CREATE TABLE `users`
(
    `id`         int(11)                NOT NULL AUTO_INCREMENT,
    `name`       varchar(255)           NOT NULL,
    `created_at` datetime DEFAULT NOW() NOT NULL,
    `updated_at` datetime               NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8;

CREATE TABLE `posts`
(
    `id`         int(11)                NOT NULL AUTO_INCREMENT,
    `user_id`    int(11)                NOT NULL,
    `title`      varchar(255)           NOT NULL,
    `created_at` datetime DEFAULT NOW() NOT NULL,
    `updated_at` datetime               NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8;

INSERT INTO users (name, updated_at)
VALUES ('Aron', NOW());
SET @user_id = last_insert_id();
INSERT INTO posts (user_id, title, updated_at)
VALUES (@user_id, 'Aron - post#1', NOW()),
       (@user_id, 'Aron - post#2', NOW());

INSERT INTO users (name, updated_at)
VALUES ('Bea', NOW());
SET @user_id = last_insert_id();
INSERT INTO posts (user_id, title, updated_at)
VALUES (@user_id, 'Bea - post#1', NOW());

INSERT INTO users (name, updated_at)
VALUES ('carl', NOW());

INSERT INTO users (name, updated_at)
VALUES ('Dora', NOW());
SET @user_id = last_insert_id();
INSERT INTO posts (user_id, title, updated_at)
VALUES (@user_id, 'Dora - post#1', NOW()),
       (@user_id, 'Dora - post#2', NOW()),
       (@user_id, 'Dora - post#3', NOW());

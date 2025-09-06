-- Your SQL goes here
CREATE TABLE `sets`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`left_player_id` INTEGER NOT NULL,
	`right_player_id` INTEGER NOT NULL,
	`left_score` INTEGER NOT NULL,
	`right_score` INTEGER NOT NULL,
	`date_time` DATE NOT NULL,
	`raw_input_text` TEXT NOT NULL
);

CREATE TABLE `players`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`tier_id` INTEGER NOT NULL
);

CREATE TABLE `player_aliases`(
	`player_id` INTEGER NOT NULL PRIMARY KEY,
	`alias` TEXT NOT NULL
);

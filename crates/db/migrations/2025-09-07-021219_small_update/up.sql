-- Your SQL goes here

ALTER TABLE `players` ADD COLUMN `discord_id` BIGINT NOT NULL;

ALTER TABLE `sets` DROP COLUMN `date_time`;
ALTER TABLE `sets` ADD COLUMN `date_time` TIMESTAMP NOT NULL;


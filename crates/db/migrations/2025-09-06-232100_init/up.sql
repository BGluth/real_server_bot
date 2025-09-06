-- Your SQL goes here


ALTER TABLE `sets` DROP COLUMN `date_time`;
ALTER TABLE `sets` ADD COLUMN `date_time` TIMESTAMP NOT NULL;


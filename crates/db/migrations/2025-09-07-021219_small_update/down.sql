-- This file should undo anything in `up.sql`

ALTER TABLE `players` DROP COLUMN `discord_id`;

ALTER TABLE `sets` DROP COLUMN `date_time`;
ALTER TABLE `sets` ADD COLUMN `date_time` DATE NOT NULL;


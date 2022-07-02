CREATE TABLE `todos` (
  `id` int(10) NOT NULL PRIMARY KEY,
  `title` varchar(128) NOT NULL,
  `description` varchar(256) NOT NULL,
  `is_done` BOOLEAN NOT NULL DEFAULT 0
)
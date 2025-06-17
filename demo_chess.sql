-- MariaDB dump 10.19  Distrib 10.4.32-MariaDB, for Win64 (AMD64)
--
-- Host: localhost    Database: demo_chess
-- ------------------------------------------------------
-- Server version	10.4.32-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `games`
--

DROP TABLE IF EXISTS `games`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `games` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `white_player_id` int(11) NOT NULL,
  `black_player_id` int(11) NOT NULL,
  `game_type` enum('normal','blitz') NOT NULL,
  `result` enum('white_win','black_win','draw') DEFAULT NULL,
  `status` enum('in_progress','finished') NOT NULL DEFAULT 'in_progress',
  `started_at` timestamp NOT NULL DEFAULT current_timestamp(),
  `ended_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=8 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `games`
--

LOCK TABLES `games` WRITE;
/*!40000 ALTER TABLE `games` DISABLE KEYS */;
INSERT INTO `games` VALUES (1,61,62,'normal','white_win','finished','2025-06-16 23:25:56','2025-06-16 23:26:13'),(2,62,61,'normal','white_win','finished','2025-06-16 23:37:34','2025-06-16 23:37:45'),(3,61,62,'normal','black_win','finished','2025-06-16 23:37:48','2025-06-16 23:38:09'),(4,61,62,'normal','white_win','finished','2025-06-16 23:53:21','2025-06-16 23:53:40'),(5,61,62,'normal','white_win','finished','2025-06-17 02:05:15','2025-06-17 02:05:28'),(6,61,62,'normal','white_win','finished','2025-06-17 02:19:23','2025-06-17 02:19:33'),(7,61,62,'normal','white_win','finished','2025-06-17 02:26:40','2025-06-17 02:27:10');
/*!40000 ALTER TABLE `games` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `moves`
--

DROP TABLE IF EXISTS `moves`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `moves` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `game_id` int(11) NOT NULL,
  `move_number` int(11) NOT NULL,
  `player_color` enum('black','white') NOT NULL,
  `pgn_move` varchar(16) NOT NULL,
  `created_at` timestamp NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=53 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `moves`
--

LOCK TABLES `moves` WRITE;
/*!40000 ALTER TABLE `moves` DISABLE KEYS */;
INSERT INTO `moves` VALUES (1,1,1,'white','e4','2025-06-16 23:25:58'),(2,1,1,'black','h6','2025-06-16 23:25:59'),(3,1,2,'white','Bc4','2025-06-16 23:26:01'),(4,1,2,'black','h5','2025-06-16 23:26:02'),(5,1,3,'white','Qxh5','2025-06-16 23:26:04'),(6,1,3,'black','g6','2025-06-16 23:26:06'),(7,1,4,'white','Qf3','2025-06-16 23:26:10'),(8,1,4,'black','g5','2025-06-16 23:26:12'),(9,1,5,'white','Qxf7#','2025-06-16 23:26:13'),(10,2,1,'white','e4','2025-06-16 23:37:36'),(11,2,1,'black','e5','2025-06-16 23:37:38'),(12,2,2,'white','Bc4','2025-06-16 23:37:39'),(13,2,2,'black','Na6','2025-06-16 23:37:40'),(14,2,3,'white','Qh5','2025-06-16 23:37:42'),(15,2,3,'black','Nb8','2025-06-16 23:37:44'),(16,2,4,'white','Qxf7#','2025-06-16 23:37:45'),(17,3,1,'white','e4','2025-06-16 23:37:51'),(18,3,1,'black','e5','2025-06-16 23:37:52'),(19,3,2,'white','Bc4','2025-06-16 23:37:56'),(20,3,2,'black','Bc5','2025-06-16 23:37:59'),(21,3,3,'white','Bf1','2025-06-16 23:38:00'),(22,3,3,'black','Qh4','2025-06-16 23:38:02'),(23,3,4,'white','a3','2025-06-16 23:38:07'),(24,3,4,'black','Qxf2#','2025-06-16 23:38:09'),(25,4,1,'white','e4','2025-06-16 23:53:23'),(26,4,1,'black','e5','2025-06-16 23:53:25'),(27,4,2,'white','Bc4','2025-06-16 23:53:26'),(28,4,2,'black','Na6','2025-06-16 23:53:34'),(29,4,3,'white','Qh5','2025-06-16 23:53:36'),(30,4,3,'black','Nb8','2025-06-16 23:53:37'),(31,4,4,'white','Qxf7#','2025-06-16 23:53:40'),(32,5,1,'white','e4','2025-06-17 02:05:17'),(33,5,1,'black','e5','2025-06-17 02:05:18'),(34,5,2,'white','Bc4','2025-06-17 02:05:20'),(35,5,2,'black','Na6','2025-06-17 02:05:21'),(36,5,3,'white','Qh5','2025-06-17 02:05:23'),(37,5,3,'black','Nb8','2025-06-17 02:05:24'),(38,5,4,'white','Qxf7#','2025-06-17 02:05:28'),(39,6,1,'white','e4','2025-06-17 02:19:24'),(40,6,1,'black','e5','2025-06-17 02:19:25'),(41,6,2,'white','Bc4','2025-06-17 02:19:27'),(42,6,2,'black','Na6','2025-06-17 02:19:28'),(43,6,3,'white','Qh5','2025-06-17 02:19:30'),(44,6,3,'black','Nb8','2025-06-17 02:19:32'),(45,6,4,'white','Qxf7#','2025-06-17 02:19:33'),(46,7,1,'white','e4','2025-06-17 02:26:42'),(47,7,1,'black','e5','2025-06-17 02:26:44'),(48,7,2,'white','Bc4','2025-06-17 02:26:46'),(49,7,2,'black','Na6','2025-06-17 02:27:04'),(50,7,3,'white','Qh5','2025-06-17 02:27:06'),(51,7,3,'black','Nb8','2025-06-17 02:27:08'),(52,7,4,'white','Qxf7#','2025-06-17 02:27:10');
/*!40000 ALTER TABLE `moves` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `statistics`
--

DROP TABLE IF EXISTS `statistics`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `statistics` (
  `user_id` int(11) NOT NULL DEFAULT 0,
  `games_played` int(11) NOT NULL DEFAULT 0,
  `games_won` int(11) NOT NULL DEFAULT 0,
  `games_lost` int(11) NOT NULL DEFAULT 0,
  `games_drawn` int(11) NOT NULL DEFAULT 0,
  `current_win_streak` int(11) NOT NULL DEFAULT 0,
  `max_win_streak` int(11) NOT NULL DEFAULT 0,
  `elo` int(11) NOT NULL DEFAULT 600,
  `max_elo` int(11) NOT NULL DEFAULT 600,
  `last_game_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`user_id`),
  CONSTRAINT `statistics_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `statistics`
--

LOCK TABLES `statistics` WRITE;
/*!40000 ALTER TABLE `statistics` DISABLE KEYS */;
INSERT INTO `statistics` VALUES (61,1,1,0,0,0,0,600,600,'2025-06-17 02:27:10'),(62,1,0,1,0,0,0,600,600,'2025-06-17 02:27:10');
/*!40000 ALTER TABLE `statistics` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(32) NOT NULL,
  `email` varchar(124) NOT NULL,
  `password_hash` varchar(256) NOT NULL,
  `role` enum('player','admin','moderator') NOT NULL DEFAULT 'player',
  `status` enum('active','banned','suspended','deleted') NOT NULL DEFAULT 'active',
  `created_at` timestamp NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=63 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES (61,'macius2010','1@x.com','$argon2id$v=19$m=19456,t=2,p=1$9dSHLRbypAwNgX4wwsJjwg$jEJJVZ0MiQXVxcyAw/2zWrofGW9+14kBe3lmzYJb2no','player','active','2025-06-16 20:17:22'),(62,'rafalek2012','2@x.com','$argon2id$v=19$m=19456,t=2,p=1$jchnHyZW6aD3gS6KBUM+oQ$3pp5Kwbu5sTuwbY7CJud3i2suq0QMJbLkXdgmfytX8Y','player','active','2025-06-16 20:18:11');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-06-17  4:31:49

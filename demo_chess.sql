-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Maj 31, 2025 at 07:49 PM
-- Wersja serwera: 10.4.32-MariaDB
-- Wersja PHP: 8.2.12

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `demo_chess`
--

-- --------------------------------------------------------

--
-- Struktura tabeli dla tabeli `statistics`
--

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
  `last_game_at` timestamp NULL DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `statistics`
--

INSERT INTO `statistics` (`user_id`, `games_played`, `games_won`, `games_lost`, `games_drawn`, `current_win_streak`, `max_win_streak`, `elo`, `max_elo`, `last_game_at`) VALUES
(1, 0, 0, 0, 0, 0, 0, 1000, 1000, NULL),
(53, 0, 0, 0, 0, 0, 0, 600, 600, NULL),
(54, 0, 0, 0, 0, 0, 0, 800, 600, NULL),
(55, 0, 0, 0, 0, 0, 0, 1000, 600, NULL),
(56, 0, 0, 0, 0, 0, 0, 1000, 1000, NULL),
(57, 0, 0, 0, 0, 0, 0, 800, 800, NULL),
(58, 0, 0, 0, 0, 0, 0, 600, 600, NULL);

-- --------------------------------------------------------

--
-- Struktura tabeli dla tabeli `users`
--

CREATE TABLE `users` (
  `id` int(11) NOT NULL,
  `username` varchar(32) NOT NULL,
  `email` varchar(124) NOT NULL,
  `password_hash` varchar(256) NOT NULL,
  `role` enum('player','admin','moderator') NOT NULL DEFAULT 'player',
  `status` enum('active','banned','suspended','deleted') NOT NULL DEFAULT 'active',
  `created_at` timestamp NOT NULL DEFAULT current_timestamp()
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `users`
--

INSERT INTO `users` (`id`, `username`, `email`, `password_hash`, `role`, `status`, `created_at`) VALUES
(1, 'abc', 'abc@xxx.com', 'dsfhksdjhrjkl23hrjkhfjk4h34jkhfsdjkhfjk234h5jkr43hjk5h4jkrfherjkfhhkjhjkhjkhkjh', 'player', 'active', '2025-05-24 13:38:05'),
(2, 'ksaljdklas', 'kacperkuiras@gdfs.com', 'Haslo/12', 'player', 'active', '2025-05-24 16:44:12'),
(3, 'fhsdkjfh', 'hjghjg@gfd.sds', 'sjfhsdkjhf', 'player', 'active', '2025-05-24 16:54:24'),
(4, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 16:55:01'),
(5, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 16:56:48'),
(6, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 16:58:35'),
(7, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 16:58:59'),
(8, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 16:59:36'),
(9, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:00:02'),
(10, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:00:04'),
(11, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:00:04'),
(12, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:00:05'),
(13, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:00:05'),
(14, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:00:05'),
(15, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:00:06'),
(16, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:00:06'),
(17, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:01:16'),
(18, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:01:40'),
(19, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:01:45'),
(20, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:01:53'),
(21, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:02:35'),
(22, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:03:00'),
(23, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:03:21'),
(24, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:03:30'),
(25, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:03:46'),
(26, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 'player', 'active', '2025-05-24 17:03:47'),
(27, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:04:02'),
(28, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:04:10'),
(29, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:05:52'),
(30, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:06:11'),
(31, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:08:43'),
(32, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:13:20'),
(33, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:14:01'),
(34, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:14:21'),
(35, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:15:03'),
(36, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:15:40'),
(37, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 'player', 'active', '2025-05-24 17:16:29'),
(38, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dfgdfgdfg', 'player', 'active', '2025-05-24 17:16:43'),
(39, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dfgdfgdfg', 'player', 'active', '2025-05-24 17:16:59'),
(40, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dgfdfgd', 'player', 'active', '2025-05-24 17:17:14'),
(41, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dgfdfgd', 'player', 'active', '2025-05-24 17:17:48'),
(42, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dgfdfgd', 'player', 'active', '2025-05-24 17:18:07'),
(43, 'dsklfjklsddfgdfgd', 'jfklsdj@jkl.kk', 'fdfgdfgd', 'player', 'active', '2025-05-24 17:19:02'),
(44, 'dsklfjklsddfgdfgd', 'jfklsdj@jkl.kk', 'fdfgdfgd', 'player', 'active', '2025-05-24 17:20:03'),
(45, 'kapi982', 'kacperkuras558@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$KcdEAtFp9QNf40/+bQKK8Q$RTkd+jxAf2pNjQocWr9tRytfNBf3Il5ZyoVrirXBkZE', 'player', 'active', '2025-05-24 17:54:15'),
(46, 'fsdhfjksd', 'kacper1434@GFDG.GDF', '$argon2id$v=19$m=19456,t=2,p=1$/42JOWBKJ0s3ZDhtZq+gGw$10392LwYi8l3NQVBr5qH7iuZc7BUq6hSjTSVMQjpB9Y', 'player', 'active', '2025-05-24 22:27:45'),
(47, 'kjkljkljkljkljkl', 'kamilkuras19@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$tMxpPAaDEvplBFVrLb1/6g$0yLoZvsAb3PZbQCsA7JghXGHjI8c47Ca6wOmvWcD1J0', 'player', 'active', '2025-05-24 22:30:55'),
(48, 'kapie9823', 'kasdksd@fd.csa', '$argon2id$v=19$m=19456,t=2,p=1$Q5yW76ujyCKYT9ooXcntMQ$SV9iVk6nGTvuMK5y1RMKJrruQSKH9t2hIec5nBQtymQ', 'player', 'active', '2025-05-24 23:31:36'),
(49, 'fjsdklfjsdklfjwkfw', 'fhgjkfdhgjk@jfdhgjk.fsd', '$argon2id$v=19$m=19456,t=2,p=1$3qK70V/8dxV9Z9x4cKsvgw$rWLjjL+0JttPidC17+10joDjzIR/XC8rY1jtF4zZ/NU', 'player', 'active', '2025-05-24 23:36:15'),
(50, 'fjsklf', 'fhgfdhgjk@jfdjk.fsd', '$argon2id$v=19$m=19456,t=2,p=1$FoTw3VyjAunYQ+7ZLu3WPA$AsgGuzhrYZoNhCu5r4ibZ4EE4cJ9WNtqmtWUwnZI9ck', 'player', 'active', '2025-05-24 23:36:38'),
(51, 'fjghjkwccg1234', 'jkjklj@gdfgf.vdffd', '$argon2id$v=19$m=19456,t=2,p=1$J2hXcQIC/ZJ77t8zKAF9IQ$pnm+N05DNvHPiywehl2WPEwgK5E2iCTLD+GNnRlm3/w', 'player', 'active', '2025-05-24 23:37:12'),
(52, 'aaaaaa', 'aaa@a.com', '$argon2id$v=19$m=19456,t=2,p=1$vso6o9YJ9RgYx7tu4nSoCQ$FCFUp/HDSmWUvp33IBbo/HGRqnJu+94IEnupIcrsUc8', 'player', 'active', '2025-05-29 17:37:47'),
(53, 'test1', 'test1@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$lmZJTxuECF3NLjn99ZWgFQ$AdvCfzrdeCVJEZdNcA/etbZ3G8CRHHKIbGNcn3fJv8M', 'player', 'active', '2025-05-30 19:19:43'),
(54, 'test2', 'test2@gmial.com', '$argon2id$v=19$m=19456,t=2,p=1$weif4kO5n+8OA8YnvHNrgA$H7xf7B19r2FFJu+HBswQ5Xgg2VNlTRaIkpwnAYyBbG8', 'player', 'active', '2025-05-30 19:21:05'),
(55, 'test3', 'test3@gmial.com', '$argon2id$v=19$m=19456,t=2,p=1$EQ2HMg7Hr7gkK1vYxCkDDA$r1guPO80gnRL/UmtRKOJ8nP9KZfD2W3VhMY9DEf29VA', 'player', 'active', '2025-05-30 19:21:20'),
(56, 'test4', 'test4@gmial.com', '$argon2id$v=19$m=19456,t=2,p=1$fSXH2+suuAmxF21hKzsB3w$gv+j5omzS+EAmqiefdbx5x9WkchELIXUlDk1q9MyYY8', 'player', 'active', '2025-05-30 19:22:46'),
(57, 'test5', 'test5@gmial.com', '$argon2id$v=19$m=19456,t=2,p=1$er6ABatVc5Vz29qzyMkPAg$6ijyCPuUOsJm2dN2KsIYUygHJaFuxWlt68/seb/+dws', 'player', 'active', '2025-05-30 19:22:57'),
(58, 'test6', 'test6@gmial.com', '$argon2id$v=19$m=19456,t=2,p=1$QWV7Qzf8iGI1MSNH0AMuJQ$DAANUCNDB2FoQtzuz5xd5U8/XN1iLa+8Bi035GF+ljE', 'player', 'active', '2025-05-30 19:23:10');

--
-- Indeksy dla zrzutów tabel
--

--
-- Indeksy dla tabeli `statistics`
--
ALTER TABLE `statistics`
  ADD PRIMARY KEY (`user_id`);

--
-- Indeksy dla tabeli `users`
--
ALTER TABLE `users`
  ADD PRIMARY KEY (`id`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `users`
--
ALTER TABLE `users`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=59;

--
-- Constraints for dumped tables
--

--
-- Constraints for table `statistics`
--
ALTER TABLE `statistics`
  ADD CONSTRAINT `statistics_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;

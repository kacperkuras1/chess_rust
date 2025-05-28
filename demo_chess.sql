-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Maj 28, 2025 at 09:03 PM
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
-- Struktura tabeli dla tabeli `users`
--

CREATE TABLE `users` (
  `id` int(11) NOT NULL,
  `username` varchar(32) NOT NULL,
  `email` varchar(124) NOT NULL,
  `password_hash` varchar(256) NOT NULL,
  `elo` int(11) NOT NULL,
  `role` enum('player','admin','moderator') NOT NULL DEFAULT 'player',
  `status` enum('active','banned','suspended','deleted') NOT NULL DEFAULT 'active',
  `created_at` timestamp NOT NULL DEFAULT current_timestamp()
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumping data for table `users`
--

INSERT INTO `users` (`id`, `username`, `email`, `password_hash`, `elo`, `role`, `status`, `created_at`) VALUES
(1, 'abc', 'abc@xxx.com', 'dsfhksdjhrjkl23hrjkhfjk4h34jkhfsdjkhfjk234h5jkr43hjk5h4jkrfherjkfhhkjhjkhjkhkjh', 600, 'player', 'active', '2025-05-24 13:38:05'),
(2, 'ksaljdklas', 'kacperkuiras@gdfs.com', 'Haslo/12', 800, 'player', 'active', '2025-05-24 16:44:12'),
(3, 'fhsdkjfh', 'hjghjg@gfd.sds', 'sjfhsdkjhf', 600, 'player', 'active', '2025-05-24 16:54:24'),
(4, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 16:55:01'),
(5, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 16:56:48'),
(6, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 16:58:35'),
(7, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 16:58:59'),
(8, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 16:59:36'),
(9, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:00:02'),
(10, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:00:04'),
(11, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:00:04'),
(12, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:00:05'),
(13, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:00:05'),
(14, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:00:05'),
(15, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:00:06'),
(16, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:00:06'),
(17, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:01:16'),
(18, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:01:40'),
(19, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:01:45'),
(20, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:01:53'),
(21, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:02:35'),
(22, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:03:00'),
(23, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:03:21'),
(24, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:03:30'),
(25, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:03:46'),
(26, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dsfjfklsd', 800, 'player', 'active', '2025-05-24 17:03:47'),
(27, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:04:02'),
(28, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:04:10'),
(29, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:05:52'),
(30, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:06:11'),
(31, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:08:43'),
(32, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:13:20'),
(33, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:14:01'),
(34, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:14:21'),
(35, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:15:03'),
(36, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:15:40'),
(37, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'ghghjg', 800, 'player', 'active', '2025-05-24 17:16:29'),
(38, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dfgdfgdfg', 800, 'player', 'active', '2025-05-24 17:16:43'),
(39, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dfgdfgdfg', 800, 'player', 'active', '2025-05-24 17:16:59'),
(40, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dgfdfgd', 800, 'player', 'active', '2025-05-24 17:17:14'),
(41, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dgfdfgd', 800, 'player', 'active', '2025-05-24 17:17:48'),
(42, 'dsklfjklsd', 'jfklsdj@jkl.kk', 'dgfdfgd', 800, 'player', 'active', '2025-05-24 17:18:07'),
(43, 'dsklfjklsddfgdfgd', 'jfklsdj@jkl.kk', 'fdfgdfgd', 800, 'player', 'active', '2025-05-24 17:19:02'),
(44, 'dsklfjklsddfgdfgd', 'jfklsdj@jkl.kk', 'fdfgdfgd', 800, 'player', 'active', '2025-05-24 17:20:03'),
(45, 'kapi982', 'kacperkuras558@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$KcdEAtFp9QNf40/+bQKK8Q$RTkd+jxAf2pNjQocWr9tRytfNBf3Il5ZyoVrirXBkZE', 600, 'player', 'active', '2025-05-24 17:54:15'),
(46, 'fsdhfjksd', 'kacper1434@GFDG.GDF', '$argon2id$v=19$m=19456,t=2,p=1$/42JOWBKJ0s3ZDhtZq+gGw$10392LwYi8l3NQVBr5qH7iuZc7BUq6hSjTSVMQjpB9Y', 600, 'player', 'active', '2025-05-24 22:27:45'),
(47, 'kjkljkljkljkljkl', 'kamilkuras19@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$tMxpPAaDEvplBFVrLb1/6g$0yLoZvsAb3PZbQCsA7JghXGHjI8c47Ca6wOmvWcD1J0', 600, 'player', 'active', '2025-05-24 22:30:55'),
(48, 'kapie9823', 'kasdksd@fd.csa', '$argon2id$v=19$m=19456,t=2,p=1$Q5yW76ujyCKYT9ooXcntMQ$SV9iVk6nGTvuMK5y1RMKJrruQSKH9t2hIec5nBQtymQ', 600, 'player', 'active', '2025-05-24 23:31:36'),
(49, 'fjsdklfjsdklfjwkfw', 'fhgjkfdhgjk@jfdhgjk.fsd', '$argon2id$v=19$m=19456,t=2,p=1$3qK70V/8dxV9Z9x4cKsvgw$rWLjjL+0JttPidC17+10joDjzIR/XC8rY1jtF4zZ/NU', 600, 'player', 'active', '2025-05-24 23:36:15'),
(50, 'fjsklf', 'fhgfdhgjk@jfdjk.fsd', '$argon2id$v=19$m=19456,t=2,p=1$FoTw3VyjAunYQ+7ZLu3WPA$AsgGuzhrYZoNhCu5r4ibZ4EE4cJ9WNtqmtWUwnZI9ck', 600, 'player', 'active', '2025-05-24 23:36:38'),
(51, 'fjghjkwccg1234', 'jkjklj@gdfgf.vdffd', '$argon2id$v=19$m=19456,t=2,p=1$J2hXcQIC/ZJ77t8zKAF9IQ$pnm+N05DNvHPiywehl2WPEwgK5E2iCTLD+GNnRlm3/w', 600, 'player', 'active', '2025-05-24 23:37:12');

--
-- Indeksy dla zrzutów tabel
--

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
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=52;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;

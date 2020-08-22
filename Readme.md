## Introduction
Small program written in Rust to sends emails that are queued in a database table.
Only supports SMTP protocol.

## Featured Packages

- `diesel`: ORM that Operates on Several Databases
- `dotenv`: Configuration Loader (.env)
- `envy`: Deserializes Environment Variables into a Config Struct
- `lettre` : email client
- `lettre_email` : email composer
- `r2d2`: Database Connection Pooling
- `native-tls` : TLS connection handling
- `serde` : Serialize/Deserialize structs for diesel
- `serde_derive` : serde Serialize, Deserialize macros

## Usage

- Create SQL table that stores the emails in queue. (See structure below)
- Fill out the .env file.
- Set up the program to run as cron/automatic job.

#### Env file description

- `MAIL_SERVER` = mail server address and port
- `DATABASE` = database type to use (mysql, postgres or sqllite)
- `DATABASE_URL` = database connection url
- `INFO_EMAIL` = from email address for type: INFO. Also used as username for SMTP
- `INFO_NAME` = from email name for type: INFO
- `INFO_PASS` = password used in SMTP connection.
- `LOG_TABLE` = Table name where emails are queued.
- `SEND_DURATION` = Time in seconds to try sending failed/unsent emails.

#### Table structure for email queue
Currently the program excepts the email queue table to be in fixed structure.

Create statement:
```
CREATE TABLE `email_queue` (
  `_id` int(11) NOT NULL AUTO_INCREMENT,
  `to_address` varchar(256) NOT NULL,
  `subject` varchar(256) NOT NULL,
  `mail_body` text DEFAULT NULL,
  `from` varchar(88) DEFAULT NULL,
  `status` tinyint(4) DEFAULT 0,
  `_timestamp` datetime NOT NULL DEFAULT current_timestamp(),
  `send_time` datetime NOT NULL DEFAULT current_timestamp(),
  `sent_time` datetime DEFAULT NULL,
  `send_count` int(11) DEFAULT 0,
  `last_error` text DEFAULT NULL,
  PRIMARY KEY (`_id`),
  UNIQUE KEY `email_queue_status_time` (`status`,`send_time`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4;
```

Following fields should be populated to add email to a queue:
- to_address - addres to send the email to
- subject - email subject
- mail_body - html body for the email
- from - address from where to send the email. If it is set to INFO or NOTICE, then it will use the .env file config. Otherwise it will try to connect without pass.
- send_time - time when the email should be sent.

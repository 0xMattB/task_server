# Task Server

## Introduction

"Task Server" is a simple server program written in Rust. It allows users to post and retrieve tasks using a REST API, with JSON encoded data. Each task is given a due date and an optional reminder time period in days. A reminder email can be sent to the user a number of days ahead of the due date if the optional reminder field is set (if the reminder email function is enabled), and an expired email is sent on the due date (if the expired email function is enabled).

## Requirements

In addition to the source code, a PostgreSQL database must be used to store the program's data (the program uses Diesel ORM to interface with the database). If a dedicated machine is not available to run the database, a container (e.g. Docker) can be used.

The rest of these instructions assume Docker will be used.

## Set-Up

* If necessary, download and install the `PostgreSQL` database software.
* If necessary, download and install `Docker` and `docker-compose` software.
* If necessary, download and install `Diesel`:
    * `cargo install diesel_cli --no-default-features --features postgres`
    * Note: If the program periodically crashes with an error code 3, try placing [libintl-9.dll](https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857) directly into the source code home directory.
    * Note: If Diesel can’t find certain DLLs, add the PostgreSQL lib files to the [system path](https://github.com/diesel-rs/diesel/issues/2470#issuecomment-665548811) (using the current version number).
    * Note: If Diesel has an [error compiling](https://github.com/diesel-rs/diesel/issues/2519#issuecomment-1301801751), add to or create the following at `$HOME/.cargo/config.toml`:

```
[v1]
"diesel_cli 2.0.1 (registry+https://github.com/rust-lang/crates.io-index)" = ["diesel.exe"]

[target.x86_64-pc-windows-msvc.pq]
rustc-link-search = ["C:\\Program Files\\PostgreSQL\\16\\lib"]
rustc-link-lib = ["libpq"]
```

* Download the "task_server" source code.
* The default database port is `5432`. If this value needs to be different, change this value in `postgres.yaml` and `.env` (both files located in the root directory of the source code).
* Edit the configuration file for the desired system parameters (see "Configuration File" section below).
* Run Docker.
* Compile and run the the program.
* Once the program is running for the first time, an account with the username "admin" must be created. This user account has advanced privileges over the database.

## Entries

**Endpoint(s):**
`/entries`
`/entries/{id}`

Task entries are referred to as "Entries", and contain the following fields and access:

```
Field      GET   POST   PUT   PATCH   Search Parameters
-----      ---   ----   ---   -----   -----------------
id          *     -      -     -       -
username    *     -      -     -       -
year        *     *      *    (*)     (*)
month       *     *      *    (*)     (*)
day         *     *      *    (*)     (*)
task        *     *      *    (*)      -
reminder    *    (*)    (*)   (*)     (*)
expired     *     -      -     -      (*)
created     *     -      -     -       -
updated     *     -      -     -       -
user_id     *     -      -     -       -

 *  = Required field
(*) = Optional Field
 -  = Inaccessible field
```

* "id", "expired", "created", and "updated" are automatically assigned by the software.
* "username" is filled in based on the user who created the task.
* "Search Parameters" following the standard REST API nomenclature: `?parameter=value`.
* When a date (year, month, day) is created or modified, it is checked for validity - that the date is valid, and the date has not yet passed.
* When a reminder is created or modified, it is checked for validity (that the resulting date has not yet passed).
* GET returns all entries for that user. GET<ID> only returns the indicated entry if that task is assigned to that user.
* The "admin" user has the same access to every entry as if they were that user.
* A user may only use DELETE on an "Entry" that they own (the "admin" user can delete any "Entry").
* It is up to the user to delete any completed/expired tasks - no tasks are deleted automatically by the software.
* The "user_id" field is linked to the "User" table, for ease of look-up when sending emails.

## Users

**Endpoint(s):**
`/users`
`/users/{id}`

In order for a user to interface with the server program, they must create a user account. The user account consists of the following fields:

```
  Field        GET   POST   PUT   PATCH   Search Parameters
  -----        ---   ----   ---   -----   -----------------
> id            -     -      -     -       -
> username      -     *      -     -       -
> password      -     *      -    (*)      -
> email         -     *      -     -       -
> utc_offset    -    (*)     -    (*)      -

*   = Required field
(*) = Optional Field
-   = Inaccessible field
```

* POST is used to create a new user account. To be considered valid, the "username" and "email" fields must not already exist in the user database.
* "id" is automatically assigned by the software.
* The "password" field only performs a rudimentary check for a minimum number of characters.
* The "admin" user has full read access to the user database (including GET<ID>), and write access to the "password" and "utc_offset" fields.
* Only the "admin" user has the ability to DELETE a user.

Note: The "admin" user can delete all entries in the database by targeting the `/entries/all` endpoint with a DELETE command.

## HTTP Headers

Each API command must include an HTTP header, with the following information:

* `Content-Type: application/json`
* `authorization: basic {value}`
    * The "authorization" field is required for all commands except "User POST" (i.e. creating a new account).
    * The "authorization" field {value} is a base64 string derived from the string "{username}:{password}".
    * No additional authentication or safety measures are incorporated to obscure the "authorization" field, as this is intended to be a simple program.

## Email Timing

* Reminder emails (if applicable and enabled) are sent at midnight (UTC) at the end of the calculated date.
* Expired emails (if enabled) are sent at midnight (UTC) at the end of the due date.
* The optional "utc_offset" field in the "user" database allows each user to adjust for midnight by an indicated number of hours from UTC.

## Configuration File

* A configuration file ("config.txt") is provided that allows certain options and parameters to be set.
* If the configuration file is not valid, an error message will be displayed, and default configuration values will be used.
* If the configuration file is not present, a warning message will be displayed, a default configuration file will be generated, and default configuration values will be used.

(Note: If the configuration file becomes broken, just delete it and run the program to generate a fresh one with default values.)

Configuration fields:
* `sender_email_address`: The email address that the program uses to send reminder/expired emails.
    * Must be updated by the user if "enable_reminder_emails" and/or "enable_expired_emails" is TRUE.
* `sender_email_password`: The password for the given email address.
    * Must be updated by the user if "enable_reminder_emails" and/or "enable_expired_emails" is TRUE.
* `sender_email_smtp`: The SMTP link for the given email address.
    * Must be updated by the user if "enable_reminder_emails" and/or "enable_expired_emails" is TRUE.
* `enable_reminder_emails`: TRUE=enabled, FALSE=DISABLED
* `enable_expired_emails`: TRUE=enabled, FALSE=DISABLED
* `server_ip`: The IP address that the server is to be run on
* `server_port`: The port that the server is to be run on

## Health Check

The program contains a "health check" endpoint, that can query the status of the software:
`/health`

## API Examples (using cURL), Entries

*[auth] indicates the particular authorization string*
*[uuid] indicates the particular ID (UUID) string*
*cURL commands are formatted for Windows Powershell*

GET (All):
`curl -H "Content-Type: application/json" -H "authorization: basic [auth]" -s http://localhost:8085/api/entries`

GET (by ID):
`curl -H "Content-Type: application/json" -H "authorization: basic [auth]" -s http://localhost:8085/api/entries/[uuid]`

POST:
`curl -X POST -H "Content-Type: application/json" -H "authorization: basic [auth]" -d "{\"year\": \"2024\", \"month\": \"6\", \"day\": \"15\", \"task\": \"This is the task.\", \"reminder\": \"1\"}" http://localhost:8085/api/entries`

PUT:
`curl -s -X PUT -H "Content-Type: application/json" -H "authorization: basic [auth]" -d "{\"year\": \"2024\", \"month\": \"6\", \"day\": \"15\", \"task\": \"This is the task.\", \"reminder\": \"1\"}" http://localhost:8085/api/entries/[uuid]`

PATCH:
`curl -s -X PUT -H "Content-Type: application/json" -H "authorization: basic [auth]" -d "{\"year\": \"2025\", \"task\": \"This is an updated task.\"}" http://localhost:8085/api/entries/[uuid]`

DELETE:
`curl -H "Content-Type: application/json" -H "authorization: basic [auth]" -s -X DELETE http://localhost:8085/api/entries/[uuid]`

## API Examples (using cURL), Users

*[auth] indicates the particular authorization string*
*[uuid] indicates the particular ID (UUID) string*
*cURL commands are formatted for Windows Powershell*

GET (All):
`curl -H "Content-Type: application/json" -H "authorization: basic [auth]" -s http://localhost:8085/api/users`

GET (by ID):
`curl -H "Content-Type: application/json" -H "authorization: basic [auth]" -s http://localhost:8085/api/users/[uuid]`

POST:
`curl -X POST -H "Content-Type: application/json" -d "{\"username\": \"my_username\", \"email\": \"my_email@domain.com\", \"password\": \"my_password\", \"utc_offset\": \"5\"}" http://localhost:8085/api/users`

PATCH:
`curl -s -X PUT -H "Content-Type: application/json" -H "authorization: basic [auth]" -d "{\"password\": \"my_new_password\", \"utc_offset\": \"3\"}" http://localhost:8085/api/users/[uuid]`

DELETE:
`curl -H "Content-Type: application/json" -H "authorization: basic [auth]" -s -X DELETE http://localhost:8085/api/users/[uuid]`

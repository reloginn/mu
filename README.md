> It's an experimental program. It's not fully completed.

# What is it? #
- It's a program that runs **Tarantool** itself and configures it to work with sessions.

# How do I use it?
- First, you need to clone this repository:
```shell
git clone https://github.com/reloginn/mu.git
```
- Go to the folder and compile the program:
```shell
cd mu
cargo build --release
mkdir ~/tarantool
mv ./target/release/mu ~/tarantool
```
- All snapshots and the `http` library will be stored in the directory from where you run the program.
- The server will be available on port 5432. You can send requests to it.

# Available requests
- There are currently two requests available: `PUT 0.0.0.0.0:5432` and `GET 0.0.0.0.0:5432`.

**PUT 0.0.0.0.0:5432/**
```json
{
    "session": "value",
    "uuid": "value",
    "user_agent": "value"
}
```
- In response, you'll get an empty body, or JSON of the form:
```json
{
    "error": "value"
}
```

- To get a user by session, send a request like this:

**GET 0.0.0.0.0:5432/**
```json
{
    "session": "value"
}
```
- If everything was successful, you will receive JSON of the following form in response:
```json
{
    "session": "value",
    "uuid": "value",
    "user_agent": "value"
}
```
- Or an error:
```json
{
    "error": "value"
}
```

- You can also get all the user's sessions by sending a similar request:

**GET 0.0.0.0.0:5432/**
```json
{
    "uuid": "value"
}
```
- In response you will get an array of `session`, `uuid` and `user_agent` fields or an error.

# Requirements
- **Tarantool** (you can install it using your distribution's package manager)
- **cargo**

# Why is it not supported Windows?
- **Tarantool** does not support Windows, you can install WSL (Windows Subsystem for Linux) and work from there.

# TODO
- **[-]** Add a session expiration time 

> It's an experimental program. It's not fully completed.

# What is it? #
- It's a program that runs **Tarantool** itself and configures it to work with sessions.

# How do you use it?
- First, you need to clone this repository:
```shell
git clone https://github.com/reloginn/mu.git
```
- Go to the folder and compile the program:
```shell
cd mu
cargo run --release
mkdir ~/tarantool
mv ./target/release/mu ~/tarantool
```
- All snapshots and the `http` library will be stored in the directory from where you run the program.

# Requirements
- **Tarantool** (you can install it using your distribution's package manager)
- **cargo**

# Why is it not supported by Windows?
- **Tarantool** does not support Windows, you can install WSL (Windows Subsystem for Linux) and work from there.

# TODO
- **[-]** Add a session expiration time 
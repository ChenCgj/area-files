# Configure, Opearation and Maintenance

## Build

The client-core and server is programming in Rust, user can compile these two component easily using cargo.

The client and be the `nc` or the Android/Desktop program, the client is programmed in Kotlin, use the Idea to compile the software.


## Configure

The client-core is cross-platform, it can run in common OS such as windows, linux and so on.

The behavior of the software can be configured by the `config/config.json`, there some options to control the parameters in the running.

configure for client/host

```json
{
    "download_path": string,
    "shared_path": string,
    "token_path": string,
    "server_ip": string,
    "server_port": int,
    "host_ip": string,
    "host_port": int,
    "broadcast_ip": string,
    "client_listen_ip": string,
    "client_listen_port": int,
    "localhost_name": string
}
```

configure for server

```json
{
    "server_ip": string,
    "server_port": int,
    "file_save_path": string,
    "mysql_ip": string,
    "mysql_port": int,
    "redis_ip": string,
    "redis_port": int
}
```

## Operation

The user should execute the client-core first, and then use the client to connect the core to send the command.

There some commands here:

`CMD list my`
`CMD download ip filepath`

## Maintenance

Because the software has the server components, the server owner should maintain the MySQL database and the Redis with the server together.

The owner should watch the server status and clean the unused data to avoid out of memory.

# System Safety Analysis for Area Files

## Introdution

Area Files is writing in Rust, providing file transmission service.

There are some risk in this software and this report will describe these risks.

## System Safety and Security

### System Safety

1. Memory safety: Area Files is programming in Rust which makes the program more safe in memory.

2. Concurrency safety: A lot of concurrency errors are caused by memory errors, but the features of Rust reduce these errors. Threads share only a little data and use mutex to protect the data.

3. It is a pity that it has a lot of possible errors unhandled in the software because of the limited time.

### Security

1. The Server use the MySQL to save the user data, it's possible to be attacked by the SQL injection.
2. Use the Redis to save the hot data, it's possible to be attacked by others.
3. The data between different hosts and server is not be encryted, the data in network may be stolen.


# Requirements Analysis For Area Files

## System Summary

`Area Files` is a file sharing system. Users can use this system to exchange any file between different hosts. This software hides the details of different devices, so it's seem that all data are in only one seperate devices and any change on the filesystem will be seen in other devices. The system also provides browsing files on server, sharing files between different LAN.

`Area Files` is designed to make transmitting files between two hostes easier and make person can share their files to any one who is interested in these files.

## Software Scope

1. User interface of the system
2. Transit server
3. Database

## Business processes

There are five main businesses:

1. Manage users
2. Get information of files from the LAN and the server
3. Transmit a file from a host to another host
4. Manage the share files in host and server
5. Manage the access permission of files

## Functional Requirement

1. Users can get the information of shared files in the LAN
2. Users can download files in the LAN
3. Users can share some files in the local area network
4. Users can register on the server so that they can access the server resource
5. Users can get the information of files in the transit server
6. Users can download files from the transit server
7. Users can upload some files to the transit server
8. Users can manage the files they shares
9. Users can decide who can access their shared files by creating/applying tokens for files

## Non-functional Requirement

1. **High performance**

   The client need to transmit a lot of files to a lot of host.

   The server also need to be a ntermediary agent to transmit files between two host in different LAN.

2. **Integrity**

   The file a host received should be integrity, otherwist should report a error.

3. **Security**

   If not have the rights, user should not access the files. The data transmited in the network should not be stolen by others.

4. **Stability**
   
   The system should execute throughout the day.

5. **Robustness**

   The software can deal with some mistakes of users

6. **Scalability**

   Auto to add a new host when the host connets to the LAN.

   Easy to add new server to transmit and save files.

## Glossary

- ***file***: Data which user want to **share** is file. It contains but not only `*.mp4`, `*.mp3`, `*.png`, `*.jpg` even some text.

- ***file information***: The meta data of a file, contains filename, size, update time, user right token.

- ***server***: The transit server transmits files between hosts which are in differenct LAN. The server also save some files user sent and share them to other users. The server also can only save the meta data of files instead of files, and when need a files, the server will acquire the file from the source user.

- ***user right token***: The user right token is a right flag of files. If a file has a user right token, a user have to get the token so that he can access the file.

- ***LAN***: Local Area Network

- ***WAN***: Wide Area Network


## Functional Specification

### *About User*

---

**Register new user**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |


### *About File*

---

**Get file information**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |

**List files**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |

**Download file**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |


**Upload file**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |


**Upload file information**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |


**Make file shared**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |


**Make file unshared**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |


### *About User Right Token*

---

**Create user right token**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |

**Destroy user right token**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |


**Apply user right token to file**

| item          | detail  |
| ------------- | ------- |
| function      |         |
| description   |         |
| input         |         |
| source        |         |
| output        |         |
| target        |         |
| action        |         |
| condition     |         |
| precondition  |         |
| postcondition |         |
| side effect   |         |


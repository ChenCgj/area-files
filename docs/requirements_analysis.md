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

- **host and client**: These two words are similar in many content, but client is used for the server and the host is used in the LAN for a host in the same LAN.

- **server request**: Server request is means that the server require some source which is located in client. The client will periodically query the server to get the server requests, so that the client can send the resource to server.

- ***LAN***: Local Area Network

- ***WAN***: Wide Area Network


## Functional Specification

### *About User*

---

**Register new user**

| item          | detail                                                                                            |
| ------------- | ------------------------------------------------------------------------------------------------- |
| function      | register a new user in the server                                                                 |
| description   | register a new user with user name and password, and generate an uid                              |
| input         | name, password                                                                                    |
| source        | socket connected the client                                                                       |
| output        | message about whether the registration is successful, and an uid if success                       |
| target        | client                                                                                            |
| action        | the server try to register (name, password) in the database                                       |
| condition     | the name has not be occupied                                                                      |
| precondition  | the socket to the client, the socket to the database are connetced and the server is good running |
| postcondition | the (uid, name, password) is recorded in the database if successed                                |
| side effect   | none                                                                                              |


### *About File*

---

**Get file information**

| item          | detail                                          |
| ------------- | ----------------------------------------------- |
| function      | get the file information                        |
| description   | get file information from other host and server |
| input         | none                                            |
| source        | none                                            |
| output        | a request of getting file information           |
| target        | other host in the LAN and the server            |
| action        | send request to other host and server           |
| condition     | user try to get the file information            |
| precondition  | the client is connected to LAN                  |
| postcondition | the request was sent                            |
| side effect   | none                                            |

**Send file information**

| item          | detail                                              |
| ------------- | --------------------------------------------------- |
| function      | send the file information                           |
| description   | send file information to a host                     |
| input         | a request of getting file information               |
| source        | client, other hosts in LAN                          |
| output        | a response of file information                      |
| target        | cilent for server or other host in the LAN for host |
| action        | send reponse to other host and server               |
| condition     | the request is getting file information             |
| precondition  | the client is connected to LAN or server            |
| postcondition | the response was sent                               |
| side effect   | none                                                |

**List files**

| item          | detail                                                |
| ------------- | ----------------------------------------------------- |
| function      | list files with information                           |
| description   | list files infomation got from the LAN and the server |
| input         | none                                                  |
| source        | none                                                  |
| output        | the files information list                            |
| target        | the interface of user                                 |
| action        | list all files information                            |
| condition     | files information was gotten                          |
| precondition  | the client is good running                            |
| postcondition | the file lists is send to the user interface          |
| side effect   | none                                                  |

**Download file**

| item          | detail                                                               |
| ------------- | -------------------------------------------------------------------- |
| function      | download file                                                        |
| description   | download files from other host in LAN or server                      |
| input         | the filename                                                         |
| source        | the user interface                                                   |
| output        | a request of downloading files                                       |
| target        | other hosts in LAN or server                                         |
| action        | send the request of downloading file to other hosts in LAN or server |
| condition     | the filename is in the files information list                        |
| precondition  | the client is connected to the LAN or server                         |
| postcondition | the request is sent                                                  |
| side effect   | none                                                                 |

**Send file**

| item          | detail                                                              |
| ------------- | ------------------------------------------------------------------- |
| function      | send file                                                           |
| description   | send files to other host in LAN or client for server                |
| input         | a request of downloading file                                       |
| source        | other hosts in the LAN or a client for server                       |
| output        | a response with file                                                |
| target        | other hosts in LAN or client for server                             |
| action        | send the reponse of file to other hosts in LAN or client for server |
| condition     | the file is in the host or the server                               |
| precondition  | the client is connected to the LAN or server                        |
| postcondition | the response of file was sent                                       |
| side effect   | if the file in in the server, then the file content will be cached  |

**Upload file**

| item          | detail                                                                                  |
| ------------- | --------------------------------------------------------------------------------------- |
| function      | upload file                                                                             |
| description   | upload files to server                                                                  |
| input         | filename                                                                                |
| source        | user interface                                                                          |
| output        | sending result                                                                          |
| target        | the user interface                                                                      |
| action        | sending files to the server                                                             |
| condition     | the file is in the shareFiles directory                                                 |
| precondition  | the user has the account on the server                                                  |
| postcondition | the sending result is showed in the user interface                                      |
| side effect   | the server will generate the file information for the server and caches the information |

**Upload file information**

| item          | detail                                             |
| ------------- | -------------------------------------------------- |
| function      | upload file information to server                  |
| description   | upload files information to server                 |
| input         | file information                                   |
| source        | user interface                                     |
| output        | sending result                                     |
| target        | the user interface                                 |
| action        | sending file information to the server             |
| condition     | the file is in the shareFiles directory            |
| precondition  | the user has an account on the server              |
| postcondition | the sending result is showed in the user interface |
| side effect   | the server will save the information               |

**Make file shared**

| item          | detail                                                |
| ------------- | ----------------------------------------------------- |
| function      | make file shared                                      |
| description   | make file on disk to be shared                        |
| input         | filename                                              |
| source        | the user interface                                    |
| output        | make file shared result                               |
| target        | the user interface                                    |
| action        | move/copy the file to the sharedFiles directory       |
| condition     | the file in on the local diskes                       |
| precondition  | the system can access the file                        |
| postcondition | the file is moved/copied to the sharedFiles directory |
| side effect   | none                                                  |

**Make file unshared**

| item          | detail                                                   |
| ------------- | -------------------------------------------------------- |
| function      | make file unshared                                       |
| description   | make file on disk to be unshared                         |
| input         | filename                                                 |
| source        | the user interface                                       |
| output        | make file unshared result                                |
| target        | the user interface                                       |
| action        | move/delete the file to the sharedFiles directory        |
| condition     | the file in in the sharedFiles directory                 |
| precondition  | the system can access the file                           |
| postcondition | the file is moved/deleted from the sharedFiles directory |
| side effect   | none                                                     |

### *About User Right Token*

---

**Create user right token**

| item          | detail                                        |
| ------------- | --------------------------------------------- |
| function      | create user right token                       |
| description   | create a user right token                     |
| input         | token identifier                              |
| source        | user interface                                |
| output        | whether the token is generated                |
| target        | the user interface                            |
| action        | send create token request to the server       |
| condition     | the token identifier should not be duplicated |
| precondition  | the client is connected to the server         |
| postcondition | the token will be saved in the server         |
| side effect   | none                                          |

**Destroy user right token**

| item          | detail                                    |
| ------------- | ----------------------------------------- |
| function      | destroy user right token                  |
| description   | destroy a user right token                |
| input         | token identifier                          |
| source        | user interface                            |
| output        | whether the token is destroyed            |
| target        | the user interface                        |
| action        | send destroy token request to the server  |
| condition     | the token identifier should be existed    |
| precondition  | the client is connected to the server     |
| postcondition | the token will be clean out in the server |
| side effect   | none                                      |

**Apply user right token to file**

| item          | detail                                                                                           |
| ------------- | ------------------------------------------------------------------------------------------------ |
| function      | apply user right token to file                                                                   |
| description   | apply a user right token to file to protect the file from being acquired                         |
| input         | the filename, token identifier                                                                   |
| source        | user interface                                                                                   |
| output        | the result of action                                                                             |
| target        | the user interface                                                                               |
| action        | add token to the file                                                                            |
| condition     | the file is in the sharedFiles directory and the token is not applied to the file                |
| precondition  | the client is conneted to the server and the token has existed                                   |
| postcondition | the token is applied to the file                                                                 |
| side effect   | if the file or information has been uploaded to the server, update the information on the server |

**Remove user right token to file**

| item          | detail                                                                                           |
| ------------- | ------------------------------------------------------------------------------------------------ |
| function      | remove user right token from file                                                                |
| description   | remove a user right token from a file                                                            |
| input         | the filename, token identifier                                                                   |
| source        | user interface                                                                                   |
| output        | the result of action                                                                             |
| target        | the user interface                                                                               |
| action        | remove token from the file                                                                       |
| condition     | the file is in the sharedFiles directory and the token is applied on the file                    |
| precondition  | the client is conneted to the server and the token has existed                                   |
| postcondition | the token is removed from the file                                                               |
| side effect   | if the file or information has been uploaded to the server, update the information on the server |

### *About Server Request*

**Request Query**

| item          | detail                                                  |
| ------------- | ------------------------------------------------------- |
| function      | Request Query                                           |
| description   | query the server if there is any request for the client |
| input         | none                                                    |
| source        | none                                                    |
| output        | the reponse of the request                              |
| target        | the client                                              |
| action        | receive the query response and deal with the request    |
| condition     | out of the time from last query                         |
| precondition  | the client has login to the server                      |
| postcondition | the request from the server has been processed          |
| side effect   | none                                                    |

**Query Response**

| item          | detail                                                 |
| ------------- | ------------------------------------------------------ |
| function      | query response                                         |
| description   | reply to the request query from the client             |
| input         | the request query from the client                      |
| source        | the client                                             |
| output        | server request response                                |
| target        | the client                                             |
| action        | send the request reponse to the client                 |
| condition     | the request is the query request                       |
| precondition  | the client user has login the server                   |
| postcondition | the reponse with server request was sent to the client |
| side effect   | none                                                   |

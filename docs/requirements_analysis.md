# Requirements Analysis for Area Files

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

   The file a host received should be integrity, otherwise should report a error.

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

- ***server token***: user right token used on server, get file from the server need this kind of token

- ***LAN token***: user right token used on LAN, get file from other host in LAN need this kind of token

- **host and client**: These two words are similar in many content, but client is used for the server and the host is used in the LAN for a host in the same LAN.

- **server request**: Server request is means that the server require some source which is located in client. The client will periodically query the server to get the server requests, so that the client can send the resource to server.

- ***LAN***: Local Area Network

- ***WAN***: Wide Area Network


## Use Case

**usecase1: register and login**

![register_login](image/requirements_report/use_case1_register_login.svg)

![register_process](image/requirements_report/use_case1_register_process.svg)

![login_process](image/requirements_report/use_case1_login_process.svg)

*description*:
- register:
  - description: register a new user in the server
  - actor: user, server
  - precondition: the user can access the server
  - steps:
    1. input the name
    2. input the password
    3. input the password again
    4. submit form
    5. server check whether the name is existed
    6. server generates a uid and saves the account in database
    7. register successfully
  - exceptions:
    - 3a: the second password is not as same as first, register failed
    - 4a: network error, register failed
    - 5a: the name has been occupied, register failed
  - postcondition: user interface show register successfully and show the login interface with name/uid
- login:
  - description: login to the server
  - actor: user, server
  - precondition: the user can access the server
  - steps:
    1. input the name/uid
    2. input the password
    3. submit form
    4. server check the password
    5. login successfully
  - exception:
    - 3a: network error, login failed
    - 4a: password error, login failed
  - postcondition: user interface show login successfully and show the user name/uid on the main interface

**usecase2: get file information and send file information**

![get_send_file_info](image/requirements_report/use_case2_get_send_file_info.svg)

![get_send_file_info_process](image/requirements_report/use_case2_get_send_file_info_process.svg)

*description*:
- get file information:
  - description: update the shared file information list by requesting file information from other hosts and server
  - actor: user
  - precondition: for LAN file information, need to connect to the LAN, for server, need access the server
  - steps:
    1. user click/choose update file list
    2. user wait the file information response
    3. update the file list on the interface
  - exception:
    - 1a: network error, update failed
    - 2a: not file information was received, update failed
  - postcondition: update the file list
- send file information:
  - description: send file information to reply the file information request received before
  - actor: user, server
  - precondition: receive a file information request
  - steps:
    - for user:
      1. get all information of files located in sharedFiles directory and packs them to json
      2. send the json str to the sender
    - for server:
      1. get all information of files which received from other user, the meta data of files and packs them to json
      2. send the json str to the sender
  - exception:
    - 1a: get information failed, send failed
    - 2a: network error, send failed
  - postcondition: send the json string of file information to sender

**usecase3: list files**

![list_files](image/requirements_report/use_case3_list_files.svg)

![list_files](image/requirements_report/use_case3_list_file_process.svg)

*description*:
- list files information:
  - description: list files information gotten from the server and hosts
  - actor: user
  - precondition: none
  - steps:
    1. user click/choose show the file list or user on the file list interface
    2. list all files information
  - exception: none
  - postcondition: files information are on the surface

**usecase4: download and send file**

![download_send_file](image/requirements_report/use_case4_download_send_file.svg)

![download_send_file_process](image/requirements_report/use_case4_download_send_file_process.svg)

*description*:
- download file:
  - description: download file from host or server
  - actor: user
  - precondition: for file on host, connnection to LAN is required, for file on server, being able to access server is required
  - steps:
    1. user input the filename
    2. check whether the filename exists in file information
    3. check whether the user has the user right token asked by the file
    4. ask the source to download the file
    5. receive the file
    6. show download finish message on user interface
  - exception:
    - 2a: the filename does not exist in file information, downlaod failed
    - 3a: the user does not have the user right token asked by the file, download failed
    - 4a: the source does not connect to the network, download failed
    - 4b: network error, download failed
    - 4c: souce check the user right token failed, download failed
    - 5a: network error, download failed
  - postcondiction: file required was downloaded in downloads directory
- send file:
  - description: send file to destination
  - actor: user, server
  - precondition: receive downlaod file request
  - steps:
    1. check if the filename exists
       - for server: check whether the file is on disk, or has the meta data of file, if not have the file, ask the source to upload the file and wait it if possible
       - for host: only check whether the file is on disk
    2. check the user right token asked by the file and the user right token in the request
    3. transmit the file to the destination
  - exception:
    - 1a: the file does not existed in the sharedFiles directory, send file failed, send error message
    - 2a: check the user right token failed, send file failed, send error message
    - 3a: network error, send file failed
  - postcondition: file was transmitted to the destination

**usecase5: upload file or file information**

![upload_file_information](image/requirements_report/use_case5_upload_file_information.svg)

![upload_file_process](image/requirements_report/use_case5_upload_file_process.svg)

*description*:
- upload file information:
  - description: upload file information to server
  - actor: user, server
  - precondition: login to the server
  - steps:
    1. check wheter the user logs in
    2. input the file(s) name
    3. query whether the user want to upload file content
    4. check whether the file exists in sharedFiles directory
    5. submit form
    6. server save the information and file content(if have)
  - exception:
    - 1a: ask user login first
    - 4a: a file does not exists, send file information failed
    - 5a: network error, send file information failed
    - 6a: server error, save file information failed
  - postcondition: file information was saved in server

**usecase6: switch file sharing statu**

![switch_file_sharing statu](image/requirements_report/use_case6_switch_shared.svg)

![switch_file_sharing_process](image/requirements_report/use_case6_switch_process.svg)

*description*:
- make file be shared:
  - description: make a file can be shared in LAN or server
  - actor: user, server
  - precondition: none
  - steps:
    1. input the filename
    2. check whether the file exists and locates in correct path
    3. switch the shared status
    4. if register, update the information on server
    5. show status of the file
  - exception:
    - 2a: file does not exist or does nost need to switch statu, switch failed
    - 4a: the user does not login or network error, show message and retry when connect to the netword 
  - postcondition: the sharing statu of file was changed

**usecase7: create token**

![create_token](image/requirements_report/use_case7_create_token.svg)

![create_token_process](image/requirements_report/use_case7_create_token_process.svg)

*description*:
- create token
  - description: create user right token to limit the access right of file
  - actor: user, server
  - precondition: for server token, need to login first
  - steps:
    1. input the token identifier
    2. check the identifier does not exist
    3. input the token password
    4. query whether the token is valid in LAN or in server
    5. if in server, server save the token
  - exception:
    - 2a: the identifier has existed, create failed
    - 5a: network failed, create failed
  - postcondition: create a new user right token

**usecase8: destroy token**

![destroy_token](image/requirements_report/use_case8_destroy_token.svg)

![destroy_token_process](image/requirements_report/use_case8_destroy_token_process.svg)

*description*:
- destroy token
  - description: destroy user right token
  - actor: user, server
  - precondition: for server token, need to login
  - steps:
    1. input the token identifier
    2. check whether the token identifier exists
    3. if the token is in server, update the token list in server
    4. clean the token in all files
    5. delete successfully
  - exception:
    - 2a: the token identifier does not exist, destroy failed
    - 3a: network error, destroy failed
  - postcondition: destroy the token

**usecase9: apply or remove token on file**

![apply_remove_token_on_file](image/requirements_report/use_case9_apply_remove_file_token.svg)

![apply_token_process](image/requirements_report/use_case9_apply_token_process.svg)

![remove_token_process](image/requirements_report/use_case9_remove_token_process.svg)

*description*:
- apply token
  - description: apply token on file, limit the file access right by the token
  - actor: user, server
  - precondition: none
  - steps:
    1. input the token identiifer
    2. input the filename
    3. check whether the token exists and the file exists
    4. check if the file has the token or not
    5. add token for this file
    6. if the file has been uploaded to the server, update the file information on server
    7. show message about adding successfully on the interface
  - exception:
    - 3a: the token of the file does not exist, apply failed
    - 4a: the token has been applied on the file, apply failed
  - postconditions: the token has been applied to the file
- remove token
  - description: remove a token from a file
  - actor: user, server
  - precondition: none
  - steps:
    1. input the token identiifer
    2. input the filename
    3. check whether the token exists and the file exists
    4. check if the file has the token or not
    5. remove token from this file
    6. if the file has been uploaded to the server, update the file information on server
    7. show message about removing successfully on the interface
  - exception:
    - 3a: the token of the file does not exist, remove failed
    - 4a: the token has not been applied on the file, remove failed
  - postconditions:

**usecase10: server request query and reply**

![server_request_query_and_reply](image/requirements_report/use_case10_query_reply_server_request.svg)

![server_request_query_reply_process](image/requirements_report/use_case10_server_request_process.svg)

*description*:
- server request query
  - descritption: ask the server that is there any server request for the client
  - actor: user, server
  - precondition: login to the server, out of time limit since last query
  - steps:
    1. send the query to the server
  - exceptions:
    - 1a: network error
  - postcondition: a query request was sent to the server
- server request reply
  - description: reply to the client's query with server request
  - actor: user, server
  - precondition: receive the server request requery
  - steps:
    1. search that whether there is some file request from other clients, if any, pack it in the reply
    2. send the reply to the client
  - exception:
    - 2a: network error, send failed
  - postconditions: a reply was sent to the client

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

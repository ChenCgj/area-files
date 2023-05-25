# Detail Design

## Sequence of Some Cases

**use case2**

![use_case2](image/detail_design/seq_case2.svg)

**use case4**

![use_case4](image/detail_design/seq_case4.svg)

## Protocol Design

### protocols between user interface and client core

Format: `Type Action [Args ...]\0` (case sentensive)

1. register user

   send:  `CMD register name password`

   reply: json string

   ```json
   {
       "statu": int,
       "uid": string,
       "name": string,
       "msg": string
   }
   ```

2. login

   send: `CMD login (name|uid) password`

   reply: json string

   ```json
   {
       "statu": int,
       "uid": string,
       "name": string,
       "msg": string
   }
   ```

3. update file list(get file information)

   send: `CMD update-file-info`

   reply: json string

   ```json
   {
       "statu": int,
       "info": array of FileInfo,
       "msg": string
   }
   ```

4. list files

   send: `CMD list (all | my | server | hosts)`

   reply: json string

   ```json
   {
       "statu": int,
       "info": array of FileInfo,
       "msg": string
   }
   ```

5. download file

   send: `CMD download filename` (filename should be the releative path shown in FileInfo)

   reply: json string

   ```json
   {
       "statu": int,
       "msg": string
   }
   ```

   - query download process

     send: `CMD download-query filename`

     reply: json string

     ```json
     {
        "statu": int,
        "process": int,
        "msg": string
     }
     ```

6. upload file or information

   send: `CMD upload [-info] filename`

   reply: json string

   ```json
   {
       "statu": int,
       "msg": string
   }
   ```

   - query upload process

     send: `CMD upload-query filename`

     reply: json string

     ```json
     {
        "statu": int,
        "process": int,
        "msg": string
     }
     ```

7. make file unshared

   send: `CMD unshare [-d] filename`

   reply: json string

   ```json
   {
       "statu": int,
       "msg": string
   }
   ```

8. make file shared

   send: `CMD share filename` (filename should be the absoulte path)

   reply: json string

   ```json
   {
       "statu": int,
       "msg": string
   }
   ```

9. create token
    
   send: `CMD token -c [-s [-u user]] identifier password` (-s for creating server token, and -u to specified the user)

   reply: json string

   ```json
   {
       "statu": int,
       "msg": string
   }
   ```

10. destroy token
    
    send: `CMD token -d identifier`

    reply: json string

    ```json
    {
        "statu": int,
        "msg": string
    }
    ```

11. apply token
    
    send: `CMD token -a token_identifier filename`

    reply: json string

    ```json
    {
        "statu": int,
        "msg": string
    }
    ```

12. remove token from file
    
    send: `CMD token -r token_identifier filename`

    ```json
    {
        "statu": int,
        "msg": string
    }
    ```

### protocols between different hosts

1. get file information (udp broadcasting)
   
   send: json string

   ```json
   {
       "type": "query_area",
       "stamp": int
   }
   ```

   reply: json string

   ```json
   {
       "type": "reply_area",
       "stamp": int,
       "restMsg": int,
       "username": string,
       "files": array of file info
   }
   ```

2. download file (tcp)
   
   send: json string

   ```json
   {
       "type": "request_area",
       "stamp": int,
       "filename": string,
       "tokens": array of token
   }
   ```

   reply: byte stream

   ```c
   struct MsgSendArea {
       char AFSMsgType m_msgType;
       char m_fileAttrib[];
       char m_filedata[];
   }
   ```

### protocols between server and host

1. register (tcp)
   
   send: json string

   ```json
   {
       "type": "register",
       "name": string,
       "password": string
   }
   ```

   reply: json string

   ```json
   {
       "type": "register_re",
       "statu": int,
       "msg": string,
       "uuid": string
   }
   ```

2. login (tcp)
   
   send: json string

   ```json
   {
       "type": "login",
       "name": string,
       "password": string
   }
   ```

   reply: json string

   ```json
   {
       "type": "login_re",
       "statu": int,
       "msg": string,
       "uuid": string
   }
   ```

3. server request query (udp)
   
   send: json string

   ```json
   {
       "type": "query_server",
       "stamp": int,
       "uuid": string
   }
   ```

   reply: json string

   ```json
   {
       "type": "reply_query",
       "stamp": int,
       "files": array of Fileinfo
   }
   ```

4. downlaod file from server (tcp)
   
   send: json string

   ```json
   {
       "type": "request_server",
       "uuid": string,
       "tokens": array of token,
       "filename": string
   }
   ```

   reply: byte stream

   ```c
   struct MsgSendServer {
       char AFSMsgType m_msgType;
       char m_fileAttrib[];
       char m_data[];
   };
   ```

5. upload file to server (tcp)
   
   send: byte stream

   ```c
   struct MsgUpload {
        char AFSMsgType m_msgType;
        char m_fileAttrib[];
        char m_data[];
   };
   ```

   reply: None

6. get file information (tcp)
   
   send: json string

   ```json
   {
       "type": "query_server",
       "uuid": string
   }
   ```

   reply: json string

   ```json
   {
       "type": "reply_server",
       "files": [
           {
               "username": string,
               "files": array of file info
           }
       ]
   }
   ```

7. create server token (tcp)
   
   send: json string

   ```json
   {
       "type": "create_token",
       "id": string,
       "password": password
   }
   ```

   reply: json string

   ```json
   {
       "type": "create_token_reply",
       "statu": int,
       "msg": string
   }
   ```

8. destroy server token (tcp)

   send: json string

   ```json
   {
       "type": "destroy token",
       "id": string
   }
   ```

   reply: json string

   ```json
   {
       "type": "destroy_token_reply",
       "statu": int,
       "msg": string
   }
   ```

9. apply server token
    
   send: json string

   ```json
   {
       "type": "apply_token",
       "id": string,
       "filename": string
   }
   ```

   reply: json string

   ```json
   {
       "type": "apply_token_reply",
       "statu": int,
       "msg": string
   }
   ```

10. remove token from file
    
    ```json
    {
        "type": "remove_token_from_file",
        "id": string,
        "filename": string
    }
    ```

    reply: json string

    ```json
    {
        "type": "remove_token_reply",
        "statu": int,
        "msg": string
    }
    ```

## Data Structure Design

**Token**

A token contains two components: identifier and password. And there is a type field (server or LAN).

```c++
struct Token {
    enum TokenType m_type;
    std::string m_identifier;
    std::string m_password;
};
```

**FileInfo**

For FileInfo, the struct describes the file's relative and tokens, last update time and so on.

the last update time is FileInfo is different with the last modified time in file system, if you download a file from LAN, the last update time in FileInfo shouldn't be changed, but the last modified time of the file may be the time when you download the file.

```c++
struct FileInfo {
    std::string m_path;     // relative path of the shared_path in configure    
    std::string m_time;     // last update time
    std::string m_size;     // size of the file
    std::vector<Token> tokens;
    std::string user;
};
```

**User**

A user may only use the software in LAN, who will not register on server, we should distinguish different hosts in LAN, a possible method is to record the ip, or, we also can show the host name.

```c++
struct UserLAN {
    std::string m_hostName;
    uint32_t ip;
};
```

```c++
struct User {
    std::string uuid;
    std::string name;
};
```

## Directory Structure Design

- shared_path
  - file1
  - file2
  - dir1
    - file3
    - file4
  - dir2
  - ...
  - .info_dir
    - file1_info
    - file2_info
    - dir1
      - file3_info
      - file4_info
    - dir2
    - ...

- download_path
  - file1
  - file2
  - ...

## Configures

`Area Files` uses json to config the software, all possible option are bellow:

```json
{
    "download_path": string,
    "shared_path": string,
    "server_ip": string,
    "server_port": int,
    "localhost_name": string,
    "client_port": int
}
```

## UML

![UML](image/detail_design/uml.svg)

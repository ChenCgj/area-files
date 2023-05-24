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
        "process": int
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
        "process": int
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
    
   send: `CMD token -a token_idengifier filename`

   reply: json string

   ```json
   {
       "statu": int,
       "msg": string
   }
   ```   {
       "statu": int,
       "msg": string
   }

### protocols between different hosts

1. request file

### protocols between server and host

1. the server request query

## Data Structure Design

## Directory Structure Design

## Configure Design

## UML

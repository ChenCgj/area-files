# Area Files

## Introduction

The `Area Files` is a file sharing system. Users can use this system to exchange any file between different hosts.

Althought it is designed to work in local area network, it also can share files between two hosts in different local area network or wide area network by using the transit server.

## Functions

1. list all files you can get in network
2. share you files by moving your files to `shareFiles` directory
3. download the file you choose
4. user registers on the server to provide the information of user's sharing file on the server
5. set permission of your files to prevent some hosts or users to get your share files
6. get shared file on the server if user have the right to the file

## Configure

configure the ip and port of the client/server (not finish).

## Build

***This is a temp directory. It's not the final software***

```shell
cd experiment
make
```

## usage

### client

- start client

   1. create the share directory `shareFiles` and download directory `downloads`
   2. launch the client: `build/client/client`

- some client command

   1. `list-my`
   
      list all shared files of yourself
   
   2. `fresh`
   
      get information of shared files in network
   
   3. list-all
   
      list the information got by `fresh`
   
   4. download xxx
   
      download the file xxx

   5. exit

      exit the program


### server

not finish


# the fast self hosted Server Connie

A user-frendly server/client that makes it easy and cheap to transfer data between client and server world wide.

## Table of Contents
- [Installation](#installation)
- [Contributing](#contributing)
- [License](#license)





## Installation

# Prerequisites for server 
- a computer
- Linux(required for server side)
- internet access
- a postgres database(could be a cloud databse or you could host it)

# Prerequisites for client 
- a computer
- internet access


**there're 2 ways in witch you can install Connie**

# First(user-frendly)
1. download the latest release( for the client congrats you have installed connie for the server side will have to do more)
2. (the server side only works on linux file structures ) if you are on windows you will need a WSL you can refer to microsoft docs on how to install it on linux it's simple execute the create-dirs.sh file
3. place the cie or cie.exe file in /opt/Connie/bin/
4. create your server ident and vola

# Second
 git clone https://github.com/samdiron/connie && \n
 cd Connie &&\n
 chmod +x ./install.sh && chmod +x ./create-dirs.sh && \n
 ./install.sh # then answer the prompted questions

 now add /opt/Connie/bin to your path
 

before you run the server make sure to put your database connection url in file: /opt/Connie/conf/db_conn
if it's first time running Connie on that database you need to run cie -v 1 db -m true 

create your server ident by 
> cie server --new true --name localhost --net-space <NET/local>  --default-machine true

now to run the server 
> cie -v 1 bind -d true 

secure-connection, low-overhead, server-clusters, private-netwoking, secure-data-trasfer via checksums  


## Contributing

simple 

if you know how to program rust and would like to help with the Connie project 
you could email me on samertadro@gmail.com

or if you find an issue or a bug you can make a new Issue on github


## License

GNU GENERAL PUBLIC LICENSE
                       Version 3, 29 June 2007

 Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/>
 Everyone is permitted to copy and distribute verbatim copies
 of this license document, but changing it is not allowed.

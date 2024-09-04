# rust_cassandra

Simple cassandra example in rust with insert and select by configuring cassandra in docker using default cluster to connect.

**Note: For Mac users, Cargo build will not work until the c linker libraries are installed as cassandra-cpp crate would need all dependencies, So I compiled all required drivers in a docker file to build and run in a docker image**

**Run the application (via docker)**
~~~~
docker compose up
docker build rust_cassandra .
docker run rust_cassandra
~~~~


**Steps to install drivers manually and Build in Local - Cargo build (Mac users)**
1. brew install below libraries via homebrew,
~~~~
Openssl, cassandra-cpp-driver, cmake, libuv, pkg-config
~~~~
2. Build datastax driver as below,
~~~~
- git clone https://github.com/datastax/cpp-driver.git 
- cd cpp-driver 
- mkdir build && cd build 
- cmake .. 
- make
- make install
~~~~
3. Symlink the openssl by setting/export the below paths in terminal config
~~~~
export LIBRARY_PATH=$(brew --prefix openssl@3)/lib:/usr/local/lib:/usr/lib:$LIBRARY_PATH
export LIBRARY_PATH=/opt/homebrew/opt/libuv/lib:$LIBRARY_PATH
~~~~
4. source <your terminal config path> (Ex: source ~/.zshrc)
5. cargo build
if [ $1 = a ]
then
    cargo run -- -vvv --p2p 127.0.0.1:6000 --api 127.0.0.1:7000
elif [ $1 = b ]
then
    cargo run -- -vvv --p2p 127.0.0.1:6001 --api 127.0.0.1:7001 -c 127.0.0.1:6000
elif [ $1 = c ]
then
    cargo run -- -vvv --p2p 127.0.0.1:6002 --api 127.0.0.1:7002 -c 127.0.0.1:6001
fi
#!/bin/bash

source $1

DURATION=30s
THREADS=64
TIMEOUT=5

PREPARE
RUN

while [[ $THREADS -le 128 ]]; do
    sleep $TIMEOUT
    echo "   >>> Benchmarking PUT <<<"
    wrk -t 4 -d $DURATION -c $THREADS -s data/put.lua $SERVER/news
    sleep $TIMEOUT
    echo "   >>> Benchmarking GET <<<"
    wrk -t 4 -d $DURATION -c $THREADS $SERVER/news/1
    sleep $TIMEOUT
    echo "   >>> Benchmarking POST <<<"
    wrk -t 4 -d $DURATION -c $THREADS -s data/post.lua $SERVER/news

    THREADS=$((THREADS * 2))
done

KILL

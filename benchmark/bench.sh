#!/bin/bash

source $1 

COUNT=1000000
THREADS=16
TIMEOUT=10

while [[ $THREADS -le 128 ]]; do
    pushd ../rust-server/
    diesel migration run 
    popd

    ab -n $COUNT -c $THREADS -g tests/${1}_${THREADS}_put.tsv -u data/put.json $SERVER/news
    sleep $TIMEOUT
    ab -n $COUNT -c $THREADS -g tests/${1}_${THREADS}_get.tsv -r $SERVER/news/1
    sleep $TIMEOUT
    ab -n $COUNT -c $THREADS -g tests/${1}_${THREADS}_post.tsv -p data/post.json $SERVER/news
    sleep $TIMEOUT

    #cat template.plot | sed "s/THREADS/${THREADS}/g" | sed 's/METHOD/put/g' | gnuplot
    #cat template.plot | sed "s/THREADS/${THREADS}/g" | sed 's/METHOD/get/g' | gnuplot
    #cat template.plot | sed "s/THREADS/${THREADS}/g" | sed 's/METHOD/post/g' | gnuplot
    
    THREADS=$((THREADS * 2))
done
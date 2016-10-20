source $0 

COUNT=100000
THREADS=8

while [[ $THREADS < 512 ]]; do
    pushd ../rust_server/
    diesel database reset
    popd

    ab -n $COUNT -c $THREADS -g tests/put-$COUNT.tsv -u data/put.json $SERVER/news
    ab -n $COUNT -c $THREADS -g tests/get-$COUNT.tsv $SERVER/news/100
    ab -n $COUNT -c $THREADS -g tests/push-$COUNT.tsv -p data/post.json $SERVER/news

    $THREADS=$((THREADS * 2))
done
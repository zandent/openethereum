for ((n=0;n<100000;n++))
do
 echo $n
 curl -X POST --data '{"jsonrpc":"2.0","method":"eth_protocolVersion","params":[],"id":67}'  http://127.0.0.1:8545 -H "Content-Type: application/json"
 sleep 60
done
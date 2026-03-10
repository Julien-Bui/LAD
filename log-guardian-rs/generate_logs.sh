
echo "Génération de access.log en cours..."
rm -f access.log


for i in {1..1000}; do
    size=$(( RANDOM % 5000 + 500 ))
    echo "192.168.1.$(( RANDOM % 255 )) - - [10/Oct/2026:13:55:36 -0700] \"GET /index.html HTTP/1.0\" 200 $size" >> access.log
done


for i in {1..50}; do
    size=$(( RANDOM % 500000 + 100000 ))
    status=$(( RANDOM % 2 == 0 ? 404 : 500 ))
    echo "10.0.0.$(( RANDOM % 255 )) - - [10/Oct/2026:14:00:00 -0700] \"POST /api/admin HTTP/1.0\" $status $size" >> access.log
done

echo "access.log généré avec 1050 logs !"

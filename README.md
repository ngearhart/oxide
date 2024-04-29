# Oxide

This is my implementation of the Redis coding challenge. It is only complete halfway through step 4. GET and SET works great, but there is some bug that I do not have time to troubleshoot when there are > 50 requests (it might be multithread related but increasing worker threads did not change this).
```
> redis-benchmark -t SET,GET -q -n 50

SET: 25000.00 requests per second, p50=0.919 msec       
GET: 25000.00 requests per second, p50=1.047 msec

> redis-benchmark -t SET,GET -q -n 51

Error: Server closed the connectionc=nan (overall: nan)

> redis-benchmark -t SET,GET -q -n 2 -c 1

Error: Server closed the connectionc=nan (overall: nan)
```

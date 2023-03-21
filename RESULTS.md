# Results from saving small events in Prostgres

Saving small events in a PosgrSQL, this includes deserializing JSON data from the
request and then serializint data to a json json column in Postgres.

Ther are 2 app:
1. Nodejsing the express webserver and the pg database module writen in typescript.
2. rust using the axum webserverna and the sqlx::postgres databse crate.


There wew 2 tests performed.
1. Using k6 for 30 seconds wite 400 user, sending a post requst to 'http://0.0.0.0:3000/save' This saves the event in the database.
2. Using wrk to fetch the hello world string from memmory `wrk -t6 -c400 -d30s http://127.0.0.1:3000/hello-world`

# Test 1 results

| **name**    | **request** | **request/sec** |
|-------------|-------------|-----------------|
| node        | 27924       | 922.950308/s    |
| rust debug  | 41338       | 1369.241344/s   | 148.48%
| rust releas | 90114       | 2994.131152/s   | 324.72%


# Test 2 results
| **name**    | **request** | **request/sec** |
|-------------|-------------|-----------------|
| node        | 124351      | 694.70          |
| rust debug  | 225301      | 1.260           | 181.55%
| rust releas | 681023      | 3.800           | 547.55%


# WRK number

## Node

Running 30s test @ http://127.0.0.1:3000/hello-world
  6 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    59.56ms   14.43ms 294.97ms   85.88%
    Req/Sec   694.70    170.86     1.14k    65.05%
  124351 requests in 30.02s, 28.34MB read
  Socket errors: connect 149, read 114, write 0, timeout 0
Requests/sec:   4141.69
Transfer/sec:      0.94MB

## Rust debug

Running 30s test @ http://127.0.0.1:3000/hello-world
  6 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    34.70ms   34.61ms   1.13s    99.34%
    Req/Sec     1.26k   253.20     1.86k    54.11%
  225301 requests in 30.01s, 27.93MB read
  Socket errors: connect 149, read 97, write 0, timeout 0
Requests/sec:   7507.17
Transfer/sec:      0.93MB

## Rust production

Running 30s test @ http://127.0.0.1:3000/hello-world
  6 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    10.90ms    3.72ms 134.19ms   84.89%
    Req/Sec     3.80k   767.92     6.35k    68.39%
  681023 requests in 30.01s, 84.43MB read
  Socket errors: connect 149, read 122, write 0, timeout 0
Requests/sec:  22692.10
Transfer/sec:      2.81MB

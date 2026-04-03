
Upstream proxy for subdomains. 

Authenticates + authorizes with subodamain + cookie / bearer token.

Has a rate limiter. 

Enables same API with Cookie and Bearer token Authentication and TBD Authorization.

2 user roles: admin and user 

+ authentication 
+ authorization 
+ billing
+ log 
+ Rate limit 
+ remove malicious packets

# TBD
// TBD CERTAIN URLS (.git, .env) SEND STRAIGHT TO RATE LIMITER
// TBD BROADCAST LOG
// GET COOKIE
// GET BEARER TOKEN
// AUTHENTICATE
// AUTHORIZE
// SET HEADERS Access-Control-Allow-Origin
// PROXY:
// // ADD TO REQ HEADERS
// // MAKE REQUEST
// // AWAIT
// // RESPOND TO CLIENT
// BROADCAST LOG
//
// // DELETE SUBDOMAIN

## DB
user
subdomain
subdomain_user

# API

## NO AUTH:


GET /api/test

POST /api/user - register user 
GET /api/user

GET /login /login/
POST /login /login/

+ OPTIONS FOR ALL
+ GET /register /register/ - html

# POST /api/user
Creates user, add's cookie to the session.

# GET /api/user/generate/bearer-token
# GET /api/user/view/bearer-token
# DELETE /api/user/delete/bearer-token

## APP

GET /logout

GET /api/user - my user data
PUT /api/user - put user
DELETE /api/user - delete user

GET /api/test-subdomain - see if subdomain is available

GET /api/subdomain - get user subdomains

POST /api/subdomain - add subdomain

DELETE /api/subdomain - delete subdomain

Subdomain specific:
GET /api/users - users for subdomain
POST /api/users - add user to subdomain
DELETE /api/users/:id - delete user from subdomain

Subdomain specific admin:
PUT /api/make-user-admin - make user admin
PUT /api/remove-user-admin - remove user admin

GET /api/user/generate/air-token

# LOCALHOST

sudo nano /etc/hosts

127.0.0.1   subdomain.localhost.com
127.0.0.1   test.localhost.com

# Questions?
// app.
// // is it you or is it proxy?
// .well-known/apple-app-site-association
// favicon.ico
// Dashboard
// subdomain_meta
// user_meta
// /api/redirect/:subdomain
// robots.txt
// does not anser to just "domain.com".
// not private by default should also be possible for blog sites. Perhaps should be all the same type - private or public 

# Workspaces?
Proxy
App
Server

# Concepts
+ subdomains are always authenticated
+ + such solution should be build on doing these subdomains in DNS level 
+ Allows only one subdomain
+ + Other schemes will be answered as "Not authenticated".
+ Ansers "Not authenticated" to all invalid requests to avoid enumeration.

# CONFIG
active_server_ip

# Rate limiter

You can set in config Burst Size and Requests Per Second.
  
Burst size is the number of total requests allowed to pipeline in total per one IP.

Requests per second sets how many places in pipeline are freed in a second for 1 IP.

If limit is hit, 429 is returned.

# Test suite

In folder test_suite there is a function that adds a 1000000 users to your db.

# PLATFORM
+ SET .well-known/apple-app-site-association
+ PUT stuff to proxy HEADER
+ URL-id
+ add auth methods - SMART ID, log in link.
+ Companu registration extra keys
+ Central key storage?

# Server sample
Respond
Accept registrations

# CURL
curl http://localhost:8080
curl -H "Referer: http://localhost:8080" http://localhost:8080
curl -H "Referer: http://hello.hello.ee" http://localhost:8080
curl -H "Host:" http://localhost:8080
curl -H "Host: http://hello.wello.ee" http://localhost:8080
curl -H "Host: http://hello.wello.ee" http://localhost:8080 -H "Authorization: Bearer _token"
curl -b "centrale=_cookie" -H "Host: http://hello.wello.ee" http://localhost:8080


//
curl -X POST -H "Content-Type: application/json" -d '{"username": "a", "password": "b"}' http://localhost:8080/api/user -vvv

curl -b "centrale=Uun1VmI3TBu3EvW6lkaXlQ" -H "Host: http://hello.localhost.com" http://localhost:8080/api/user

curl -b "centrale=Uun1VmI3TBu3EvW6lkaXlQ" -H "Content-Type: application/json" -d '{"subdomain": "test"}'  http://localhost:8080/api/subdomain -vvv

--
curl -b "centrale=Uun1VmI3TBu3EvW6lkaXlQ" -H "Host: http://test.localhost.com" http://test.localhost.com:8080 -vvv

# Oha

oha http://localhost:8080/api/user

oha -n 1 --insecure -H "Cookie: centrale=Uun1VmI3TBu3EvW6lkaXlQ; Domain=subdomain.localhost.com; Max-Age=86400"  http://subdomain.localhost.com:8080/api/user

oha -n 1000000 --insecure -H "Cookie: centrale=Uun1VmI3TBu3EvW6lkaXlQ; Domain=subdomain.localhost.com; Max-Age=86400"  http://subdomain.localhost.com:8080/api/user

oha -n 1000000 --insecure -H "Cookie: centrale=Uun1VmI3TBu3EvW6lkaXlQ;"  http://subdomain.localhost.com:8080/api/user


oha -n 1000 --insecure -H "Cookie: centrale=Uun1VmI3TBu3EvW6lkaXlQ;"  -H "Host: http://test.localhost.com"  http://test.localhost.com:8080


---
oha -m POST -T "Content-Type: application/json" -d '{"username": "aaba", "password": "b"}' http://localhost:8080/api/user

# WEBSOCKET PROXY TEST:
curl -b "centrale=Uun1VmI3TBu3EvW6lkaXlQ" -H "Host: http://test.localhost.com" http://test.localhost.com:8080/api/user/air/token -vvv
// GET RESULT AND PASTE data.air_token AS air_token
RUST_LOG=info ws --header="Host: ws://test.localhost.com" "ws://test.localhost.com:8080/air?air_token=GmCNE1jehmk/7g9zSX5OOQ"

# WSS WEBSOCKET PROXY TEST:
curl -b "centrale=Uun1VmI3TBu3EvW6lkaXlQ" -H "Host: test.localhost.com" --cacert /Users/martin/yeah/centrale/proxy/test.localhost.com.pem  https://test.localhost.com/api/user/air/token -vvv

RUST_LOG=info ws --header="Host: wss://test.localhost.com" "wss://test.localhost.com/air?air_token=YY3i1zhlYuXubB/vZWSmYg" -k



# FILES

sqlite3 "/Users/martin/Library/Application Support/centrale/centrale.db"

sqlcipher /Users/martin/Library/Application\ Support/centrale/subdomains/test

cd /Users/martin/Library/Application\ Support/centrale/

# PROBLEMS:
+ How to get subdomain to sample server
+ basic HTML to login... should be in "app".
+ how does "app" work? How should it be called?


Subdomainer.com
prettylongdomain.com

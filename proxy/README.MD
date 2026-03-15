
Upstream proxy for subdomains. 

Authenticates + authorizes with subodamain + cookie / bearer token.
2 user roles: admin and user 


# TBD
// TBD RATE LIMIT BY IP
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

GET /login /login/
POST /login /login/


GET /register /register/
POST /api/user - register user 
GET /api/user

GET /logout

+ OPTIONS FOR ALL


# POST /api/user
Creates user, add's cookie to the session.

# GET /api/user/generate/bearer-token
# GET /api/user/view/bearer-token
# DELETE /api/user/delete/bearer-token

## APP
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

# LOCALHOST

sudo nano /etc/hosts

127.0.0.1   subdomain.localhost.com
127.0.0.1   another.subdomain.localhost.com


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

curl -X POST -H "Content-Type: application/json" -d '{"username": "a", "password": "b"}' http://localhost:8080/api/user -vvv
curl -b "centrale=9yu8eGtLVSafWpuMbR0sTw" -H "Host: http://hello.wello.ee" http://localhost:8080

curl -b "centrale=NO7sEugmOMzRoymVkrv3eA" -H "Content-Type: application/json" -d '{"subdomain": "test1"}'  http://localhost:8080/api/subdomain -vvv




# CURL

oha http://localhost:8080/api/user


oha -H "Cookie: centrale=NO7sEugmOMzRoymVkrv3eA" http://localhost:8080/api/user

oha -m POST -T "Content-Type: application/json" -d '{"username": "aaba", "password": "b"}' http://localhost:8080/api/user


# FILES

sqlite3 "/Users/martin/Library/Application Support/centrale/centrale.db"

cd /Users/martin/Library/Application\ Support/centrale/


Subdomainer.com

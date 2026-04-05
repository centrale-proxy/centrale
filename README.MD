
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
+ https cert

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

GET /api/user/air/token

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

curl -X POST -H "Content-Type: application/json" -d '{"username": "a1", "password": "b"}' https://test.myapp.local/api/user -vvv

curl -b "centrale=LFivww5YEw0XINIw9QGzpQ" -H "Referer: https://test.localhost.com" https://test.myapp.local/api/user

curl -b "centrale=IKboPW7U/954kWDVwHUl5w" -H "Content-Type: application/json" -d '{"subdomain": "test"}' https://test.myapp.local/api/subdomain -vvv

--
curl -b "centrale=LFivww5YEw0XINIw9QGzpQ" -H "Referer: http://test.localhost.com" https://test.localhost.com -vvv

# curl prod
//
curl -X POST -H "Content-Type: application/json" -d '{"username": "ba", "password": "b"}' https://test.proompt.eu/api/user -vvv

curl -b "centrale=nzAMcHw2k4R4zC0KXBug/A" https://test.proompt.eu/api/user

curl -b "centrale=nzAMcHw2k4R4zC0KXBug/A" -H "Content-Type: application/json" -d '{"subdomain": "test5"}'  https://test.proompt.eu/api/subdomain -vvv

curl -b "centrale=nzAMcHw2k4R4zC0KXBug/A" -H "Referer: https://test6.localhost.com" "https://test6.proompt.eu?air_token=f" -vvv


# Oha

oha http://localhost:8080/api/user

oha -n 1 --insecure -H "Cookie: centrale=Uun1VmI3TBu3EvW6lkaXlQ; Domain=subdomain.localhost.com; Max-Age=86400"  http://subdomain.localhost.com:8080/api/user

oha -n 1000000 --insecure -H "Cookie: centrale=Uun1VmI3TBu3EvW6lkaXlQ; Domain=subdomain.localhost.com; Max-Age=86400"  http://subdomain.localhost.com:8080/api/user

oha -n 1000000 --insecure -H "Cookie: centrale=Uun1VmI3TBu3EvW6lkaXlQ;"  http://subdomain.localhost.com:8080/api/user


oha -n 1000 --insecure -H "Cookie: centrale=Uun1VmI3TBu3EvW6lkaXlQ;"  -H "Host: http://test.localhost.com"  http://test.localhost.com:8080

//
oha -n 100 -H "Cookie: centrale=nzAMcHw2k4R4zC0KXBug/A;" -H "Referer: https://test6.localhost.com"  "https://test6.proompt.eu?air_token=f"

oha -n 100 -H "Cookie: centrale=GJR6D4B697qj67AjdJNRFw_;" https://test.proompt.eu/api/user

---
oha -m POST -T "Content-Type: application/json" -d '{"username": "aaba", "password": "b"}' http://localhost:8080/api/user

# WEBSOCKET PROXY TEST:
curl -b "centrale=Uun1VmI3TBu3EvW6lkaXlQ" -H "Host: http://test.localhost.com" http://test.localhost.com:8080/api/user/air/token -vvv
// GET RESULT AND PASTE data.air_token AS air_token
RUST_LOG=info ws --header="Host: ws://test.localhost.com" "ws://test.localhost.com:8080/air?air_token=GmCNE1jehmk/7g9zSX5OOQ"

# WSS WEBSOCKET PROXY TEST:
curl -b "centrale=Uun1VmI3TBu3EvW6lkaXlQ" -H "Host: test.localhost.com" --cacert /Users/martin/yeah/centrale/proxy/test.localhost.com.pem  https://test.localhost.com/api/user/air/token -vvv

RUST_LOG=info ws --header="Host: wss://test.localhost.com" "wss://test.localhost.com/air?air_token=YY3i1zhlYuXubB/vZWSmYg" -k


# WSS WEBSOCKET PROXY TEST PROD:
curl -b "centrale=nzAMcHw2k4R4zC0KXBug/A" -H "Host: test.proompt.eu" https://test.proompt.eu/api/user/air/token -vvv

RUST_LOG=info ws --header="Host: wss://test6.proompt.eu" "wss://test6.proompt.eu/air?air_token=kRkVKpRtcWgqATJw1+53fA" -k



# FILES

sqlite3 "/Users/martin/Library/Application Support/centrale/centrale.db"
sqlcipher /Users/martin/Library/Application Support/centrale/centrale.db 


sqlcipher /Users/martin/Library/Application\ Support/centrale/subdomains/test

sqlcipher /home/www/.local/share/centrale/centrale.db 

PRAGMA key="oNnglBgHldsPbvkOgQt";

cd /Users/martin/Library/Application\ Support/centrale/

# PROBLEMS:
+ How to get subdomain to sample server
+ basic HTML to login... should be in "app".
+ how does "app" work? How should it be called?


Subdomainer.com
prettylongdomain.com


Deploy:

sudo nano /etc/systemd/system/centrale.service

[Unit]
Description=Centrale  Proy
After=network.target

[Service]
User=www       
ExecStart=/home/www/centrale/target/release/proxy
Restart=on-failure

[Install]
WantedBy=multi-user.target


---
sudo systemctl enable centrale
sudo systemctl start centrale

sudo journalctl -u centrale.service
sudo journalctl -u centrale.service -f


sudo systemctl enable sample_server
sudo systemctl start sample_server


# CERT locally
mkcert *.localhost.com
mkcert "*.myapp.local"

# CERT prod
sudo certbot certonly --manual --preferred-challenges dns   -d "*.proompt.eu" -d "proompt.eu"

sudo nano /etc/systemd/system/centrale.socket

[Socket]
ListenStream=443

[Install]
WantedBy=sockets.target


# BRowser

fetch("https://test.myapp.local/api/user", {
  method: "POST",
  credentials: "include",         
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ username: "martin", password: "hello" }),

})
  .then(r => r.text().then(t => ({ status: r.status, headers: Object.fromEntries(r.headers), body: t })))
  .then(console.log)
  .catch(console.error);


fetch("https://test.myapp.local/api/subdomain", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ subdomain: "test" })
})
  .then(r => r.text().then(t => ({ status: r.status, headers: Object.fromEntries(r.headers), body: t })))
  .then(console.log)
  .catch(console.error);

# BRowser prod

fetch("https://test.proompt.eu/api/user", {
  method: "POST",
  credentials: "include",         
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ username: "martin", password: "hello" }),

})
  .then(r => r.text().then(t => ({ status: r.status, headers: Object.fromEntries(r.headers), body: t })))
  .then(console.log)
  .catch(console.error);


fetch("https://test.proompt.eu/api/subdomain", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ subdomain: "claya" })
})
  .then(r => r.text().then(t => ({ status: r.status, headers: Object.fromEntries(r.headers), body: t })))
  .then(console.log)
  .catch(console.error);

# Token auth
curl -H 'Authorization: Bearer uJ&$}r12A?CG:Ty*dP$jmoZY%^{3BAO[d<Y[jkd8Cv@BT?Ty7:*ftWvKiN;?9xD<' https://clay.myapp.local/api/user/bearer/token

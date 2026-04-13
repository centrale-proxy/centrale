
# Subdomain based SaaS engine.

Each subdonain gets their own forwarding address (and encrypted db in destination server).  

***

# Technically

Upstream proxy based on subdomain. 

Assumes DNS A *.domain.com being directed to proxy.  

Proxy:
+ Negotiates HTTPS
+ Authenticates (cookie / bearer)
+ Authorizes user access to subdomain,
+ Rate limits
+ proxies request (http + websocket) to destination server
+ Forwards request data to Writer (logging service)

***

## NO AUTH endoints:
```
GET /api/tester - (ensures website is up and db is working)
POST /api/user - register user (password can be 1-200 chars, username can be 1-100 chars)
POST /login - login
```
## AUTH endpoints
```
GET /api/user/bearer/token
(TBD) GET /api/user/view/bearer-token
(TBD) DELETE /api/user/delete/bearer-token

GET /api/user - my user data
GET /logout
(TBD) PUT /api/user - put user
(TBD) DELETE /api/user - delete user

Genereate air token for websockets
GET /api/user/air/token

Generate bearer token:
GET /api/user/bearer/token
(TBD) DELETE /api/user/bearer/:id

POST /api/subdomain - add subdomain (subdomain is max 20 chars, if over, will be tructuated)
(TBD) GET /api/subdomain - get user subdomains
(TBD) DELETE /api/subdomain - delete subdomain
(TBD) GET /api/test-subdomain - see if subdomain is available

Subdomain specific:
(TBD) GET /api/users - users for subdomain
(TBD) POST /api/users - add user to subdomain
(TBD) DELETE /api/users/:id - delete user from subdomain

Subdomain specific admin:
(TBD) PUT /api/make-user-admin - make user admin
(TBD) PUT /api/remove-user-admin - remove user admin
```

# SET UP ENVIRONEMENT VARIABLES
Either copy .env.sample to .env 
```
// used to encrypt local db
export CENTRALE_MASTER_PASSWORD="_my_master_password_that_is_very_long_"
// used with connection between proxy and destination server
export CENTRALE_MASTER_BEARER_TOKEN="_my_master_bearer_token_that_is_very_long_"

```
# SET UP LOCALHOST LOCALHOST
```

mkcert "*.mydomain.local"

sudo nano /etc/hosts

127.0.0.1   test.mydomain.local
```

# Concepts
+ subdomains are always authenticated.
+ Allows only one subdomain level (one.domain.com) not one.two.subdomain.com
+ + Other schemes will be answered as "Not authenticated".
+ Ansers "Not authenticated" to all invalid requests to avoid enumeration.

# Rate limiter
You can set in config Burst Size and Requests Per Second.  

Burst size is the number of total requests allowed to pipeline in total per one IP.  

Requests per second sets how many places in pipeline are freed in a second for 1 IP.
If limit is hit, 429 is returned.  

# Test suite
In folder test_suite there is a function that adds a 1000000 users to your local db.

# CURL
```
curl http://localhost:8080
curl -H "Referer: http://localhost:8080" http://localhost:8080
curl -H "Referer: http://hello.hello.ee" http://localhost:8080
curl -H "Host:" http://localhost:8080
curl -H "Host: http://hello.wello.ee" http://localhost:8080
curl -H "Host: http://hello.wello.ee" http://localhost:8080 -H "Authorization: Bearer _token"
curl -b "centrale=_cookie" -H "Host: http://hello.wello.ee" http://localhost:8080

//

curl -X POST -H "Content-Type: application/json" -d '{"username": "a1", "password": "b"}' https://test.domain.local/api/user -vvv

curl -b "centrale=LFivww5YEw0XINIw9QGzpQ" -H "Referer: https://test.domain.local" https://test.domain.local/api/user

curl -b "centrale=IKboPW7U/954kWDVwHUl5w" -H "Content-Type: application/json" -d '{"subdomain": "test"}' https://test.domain.local/api/subdomain -vvv

--
curl -b "centrale=LFivww5YEw0XINIw9QGzpQ" -H "Referer: http://test.domain.local" https://test.domain.local -vvv

```

# WEBSOCKET PROXY TEST:
```
curl -b "centrale=aaaaaa" -H "Host: http://test.localhost.com" http://test.localhost.com:8080/api/user/air/token -vvv
// GET RESULT AND PASTE data.air_token AS air_token
RUST_LOG=info ws --header="Host: ws://test.localhost.com" "ws://test.localhost.com:8080/air?air_token=GmCNE1jehmk/7g9zSX5OOQ"
```

# WSS WEBSOCKET PROXY TEST:
```
curl -b "centrale=aaaaaa" -H "Host: test.localhost.com" --cacert /Users/user/app/centrale/proxy/test.localhost.com.pem  https://test.localhost.com/api/user/air/token -vvv

RUST_LOG=info ws --header="Host: wss://test.localhost.com" "wss://test.localhost.com/air?air_token=YY3i1zhlYuXubB/vZWSmYg" -k

```

# Sqlite3 access

```
sqlcipher /home/username/.local/share/centrale/centrale.db 
PRAGMA key="aaaa";

```

# Deploy:
```

sudo nano /etc/systemd/system/centrale.service

[Unit]
Description=Centrale  Proy
After=network.target

[Service]
User=www       
ExecStart=/home/www/centrale/target/release/proxy
EnvironmentFile=/etc/environment
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

```

# CERT locally
```

mkcert "*.mydomain.local"
```

# CERT prod
```

sudo certbot certonly --manual --preferred-challenges dns   -d "*.domain.eu" -d "domain.eu"

sudo nano /etc/systemd/system/centrale.socket

[Socket]
ListenStream=443

[Install]
WantedBy=sockets.target

```

# Browser
```

fetch("https://test.mydomain.local/api/user", {
  method: "POST",
  credentials: "include",         
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ username: "test", password: "hello" }),

})
  .then(r => r.text().then(t => ({ status: r.status, headers: Object.fromEntries(r.headers), body: t })))
  .then(console.log)
  .catch(console.error);

fetch("https://test.domain.local/api/login", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ username: "test", password: "hello" }),

})
  .then(r => r.text().then(t => ({ status: r.status, headers: Object.fromEntries(r.headers), body: t })))
  .then(console.log)
  .catch(console.error);

fetch("https://test.mydomain.local/api/subdomain", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ subdomain: "test" })
})
  .then(r => r.text().then(t => ({ status: r.status, headers: Object.fromEntries(r.headers), body: t })))
  .then(console.log)
  .catch(console.error);

# Token auth
curl -H 'Authorization: Bearer test' https://test.domain.local/api/user/
```


# TBD
+ TBD CERTAIN URLS (.git, .env) SEND STRAIGHT TO RATE LIMITER
+ SET HEADERS Access-Control-Allow-Origin

# Questions?
+ app.
+ .well-known/apple-app-site-association
+ favicon.ico
+ Dashboard
+ subdomain_meta
+ user_meta
+ /api/redirect/:subdomain
+ robots.txt
+ not private by default should also be possible for blog sites. Perhaps should be all the same type - private or public 

# Workspaces?
+ Proxy
+ App
+ Server

# PLATFORM
+ SET .well-known/apple-app-site-association
+ add auth methods - SMART ID, log in link.
+ Companu registration extra keys
+ Central key storage?

# PROBLEMS:
+ basic HTML to login... should be in "app".
+ how does "app" work? How should it be called?

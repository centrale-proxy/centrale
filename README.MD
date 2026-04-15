
# Subdomain based SaaS engine.

Each subdonain gets their own forwarding address (and encrypted db in destination server).  
Idea is, that you can register new customers to new destination server, when one has enough tenants.

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
+ + add id, password and role to (destination server db) request headers
+ Forwards request data to Writer (logging service)
+ + If response status is not 200, error body is saved
+ + This helps customer support know the userfacing error as it happens

Destination server:
+ each subdomain gets own (sqlite + sqlcipher password) db
+ with id and password from request header opens db commection

***

# Oppinionated parts:
Q: If proxy is down, whole service is immediately down
A: It would be the same, if your auth server were down.

Q: Auth + proxy is done by the same part
A: I have no good solution for that
  
Q: Client errors body is logged to Writer
A: Previously customer support had to ask a programmer to find what the error was.

Q: So... an Actix server is connected straight to internet.
A: Yes - and I'm totally waiting for ideas what should be in front of it which is not Nginx.

Q: Why subdomains?
A: API looks better

Q: Why is backend doing subodmains not DNS?
A: in DNS, they would be enuremable.

A: logging system Writer uses UDP connection, which is unreliable compared to TCP.
A: it could be easily swapped to TCP, at the moment I like UDP more, as it puts a bit less work to Proxy.

Q: Encryption key is in the header of request.
A: My main idea here is that the destination server itself would not have the key(s), only the encrypted db. Totally waiting for ideas on how to transport the secret.

Q: Token and cookie are both allowed for same endpoints
A:: It seams double the work to make own endpoints for app and web,
Especcialy if you use CapacitorJS or similar and your webapp is your ios app,
This will also affected the TBD Access-Control-Allow-Origin part

## NO AUTH endoints:
```
GET /api/tester - ensures website is up and db is working
POST /api/user - register user (password can be 1-200 chars, username can be 1-100 chars, must be alphanumeric)
POST /login - login
```
## AUTH endpoints
```
GET /api/user/bearer/token - generate bearer token for user
(TBD) GET /api/user/bearer/view
(TBD) DELETE /api/user/bearer/:id

GET /api/user - my user data
GET /logout
(TBD) PUT /api/user - put user
(TBD) DELETE /api/user - delete user

GET /api/user/air/token - Genereate air token for websockets

POST /api/subdomain - add subdomain (subdomain is min 1, max 20 chars, if over, will be tructuated, must be alphanumeric)
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
copy .env.sample to .env   
Fill with your credentials

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

curl -b "centrale=ssssss" -H "Referer: https://test.domain.local" https://test.domain.local/api/user

curl -b "centrale=ssssss" -H "Content-Type: application/json" -d '{"subdomain": "test"}' https://test.domain.local/api/subdomain -vvv

--
curl -b "centrale=ssssss" -H "Referer: http://test.domain.local" https://test.domain.local -vvv

```

# WEBSOCKET PROXY TEST:
```
curl -b "centrale=aaaaaa" -H "Host: http://test.domain.local" http://test.domain.local:8080/api/user/air/token -vvv
// GET RESULT AND PASTE data.air_token AS air_token
RUST_LOG=info ws --header="Host: ws://test.domain.local" "ws://test.domain.local:8080/air?air_token=GmCNE1jehmk/7g9zSX5OOQ"
```

# WSS WEBSOCKET PROXY TEST:
```
curl -b "centrale=aaaaaa" -H "Host: test.domain.local" --cacert /Users/user/app/centrale/proxy/test.localhost.com.pem  https://test.domain.local/api/user/air/token -vvv

RUST_LOG=info ws --header="Host: wss://test.domain.local" "wss://domain.local/air?air_token=aaaaaa" -k

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
debugging:  
source /etc/env  
---
sudo systemctl enable centrale
sudo systemctl start centrale

sudo journalctl -u centrale.service
sudo journalctl -u centrale.service -f

sudo journalctl -u writer.service
sudo journalctl -u writer.service -f


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

fetch("https://test.mydomain.local/api/subdomain", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ subdomain: "test" })
})

fetch("https://test.domain.local/api/login", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ username: "test", password: "hello" }),

})


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

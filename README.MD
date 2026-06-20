
# Subdomain based SaaS engine.
  
Authenticates, authorizes, rate limits, logs.  
Proxies request to destination server.  
  
Rust + Actix + SQLite.  
  
## Centrale:
Assumes DNS A *.domain.com being directed to centrale.  
  
+ Negotiates HTTPS (or doesn't if .env sets differently)  
+ Authenticates (cookie / bearer)  
+ Authorizes user access to subdomain  
+ rate limitser(s) (IP based)  
+ proxies request (http + websocket) to destination server  
+ + adds id, password and role to request (to destination server db) headers  
+ Forwards request data to Writer (logging service)  
+ Enables user adding / user management, subdomain adding.  
+ Serves same front end for all subdomains (optionally)  
  
## Destination server:  
+ There can be multiple destination servers.    
+ each subdomain gets own enrypted (sqlite + sqlcipher password) db  
+ id, password (and role) from request header opens db connection  
  
## Writer (logging service)  
+ Logs request and response headers.  
+ If response status is not 200, error body is saved  
+ + This helps customer support know the userfacing error as it happens  
  
***
  
# Concepts
+ subdomains are always authenticated.  
+ Allows only one subdomain level (one.domain.com) not one.two.subdomain.com  
+ Answers "Not authenticated" to all invalid requests to avoid enumeration.  
+ Serves same front end for all subdomains (optionally)  
+ Same backend can be used for all subdomains  
  
# Oppinionated parts:
Q: Why subdomains?   
A: API looks better.   
  
Q: Why not everyone in same db as in "normal SaaS".  
All subdonains have their own databases (with indivudual user + pass login for each) removing the need for separate ACL logic which would be needed if all data would be in same db.   
  
Q: User registration gives an immediate cookie (instead of requiring log in).  
A: I've been in the position of asking for this feature "later" (when the auth was built already).  
    
Q: If client errors then the message body is logged to Writer.  
A: Previously customer support had to ask devops to find what customer error was.  
  
Q: Why is backend doing subodmains not DNS?  
A: in DNS all customer subdomains would be visible with a DNS query.  
  
A: logging system Writer uses UDP connection, which is unreliable compared to TCP.  
A: it could be easily swapped to TCP, at the moment I like UDP more, as it puts a bit less work to Centrale + unless a new thread is created for every message, could be blocking for Centrale.  
  
Q: Encryption key is in the header of request.    
A: My main idea here is that the destination server itself would not have the key(s), only the encrypted db. Totally waiting for ideas on how to transport the secret.    
    
Q: If centrale is down, whole service is immediately down    
A: It would be the same, if  auth server were down.    
    
Q: Auth + procy is done by the same service and they could interfere.    
A: I have no good solution for that yet but this question is on my mind.  
  
Q: Token and cookie are both allowed for same endpoints  
A: It seams double the work to make own endpoints for app and web,
Especcialy if you use CapacitorJS or similar and your webapp is your ios app,  
  
## NO AUTH endoints:
These endpoints have PUBLIC_RATE_LIMITER settings.
```
GET /api/tester - ensures website is up and db is working
POST /api/user_add - register user (password can be 1-200 chars, username can be 1-100 chars, must be alphanumeric)
POST /login - login
```
## AUTH endpoints
These endpoints have RATE_LIMITER settings.
```
GET /api/user/:user_id/bearer/generate - generate bearer token for user
GET /api/user/:user_id/bearer/view
(TBD) DELETE /api/user/bearer/:id

GET /api/user - my user data
(TBD) GET /logout
(TBD) PUT /api/user - put user
(TBD) DELETE /api/user - delete user

POST /api/subdomain - add subdomain (subdomain is min 1, max 30 chars, if over, will be tructuated, must be alphanumeric)
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
127.0.0.1   app.mydomain.local
127.0.0.1   one.mydomain.local
```

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

curl -X POST -H "Content-Type: application/json" -d '{"username": "a1", "password": "b"}' https://test.domain.local/api/user_add -vvv

curl -b "centrale=ssssss" -H "Referer: https://test.domain.local" https://test.domain.local/api/user_add

curl -b "centrale=ssssss" -H "Content-Type: application/json" -d '{"subdomain": "test"}' https://test.domain.local/api/subdomain -vvv

--
curl -b "centrale=ssssss" -H "Referer: http://test.domain.local" https://test.domain.local -vvv

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
ExecStart=/home/www/centrale/target/release/centrale
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


sudo systemctl enable destination_server
sudo systemctl start destination_server

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

fetch("https://test.mydomain.local/api/user_add", {
  method: "POST",
  credentials: "include",         
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ username: "test", password: "hello" }),
})

// ONLY ALLOWED FROM app.mydomain.local
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
+ not private by default should also be possible for blog sites. Perhaps should be all the same type - either private or public 

# PLATFORM
+ SET .well-known/apple-app-site-association
+ add auth methods - SMART ID, log in link, email.
+ Subdomain registration extra keys
+ Central key storage?

# PROBLEMS:
+ basic HTML to login... should be in "app".

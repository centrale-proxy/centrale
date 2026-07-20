
# Subdomain based SaaS engine.
  
Authenticates, authorizes, rate limits, logs.  
Proxies request to destination server.  
  
Rust + Pingora + Actix + SQLite.  

Assumes DNS A *.domain.com being directed to load_balancer.  

## Load balancer or pingora-instead-of-nginx:
+ Use Pingora at `0.0.0.0:80` and `0.0.0.0:443` to forward traffic to Centrale  
+ Negotiates HTTPS on `:443`
+ Redirects HTTP on `:80` to HTTPS by default 
+ Forwards HTTPS + websocket traffic to Centrale 

## Centrale:
+ Authenticates (cookie / bearer)  
+ Authorizes user access to subdomain  
+ IP based rate limiter(s) (actix-governor)   
+ proxies request (https + websocket) to destination server  
+ + adds user_id, password and role to request (to destination server db) headers  
+ Forwards request data to Writer (logging service)  
+ Enables user adding / user management, subdomain adding.  
+ Serves same front end for all subdomains (optionally)  
+ User management + user subdomain access management
  
## Destination server:  
+ There can be multiple destination servers.    
+ each subdomain gets own enrypted (sqlite + sqlcipher password) db  
+ id, password (and role) from request header opens db connection  
+ Active destination server is set with DESTINATION_SERVER_ADDRESS  
  
## Writer (logging service)  
+ Logs request header bytes in a separate db.
+ Parses bytes + stores url, query, method, browser, refferer, host, os, etc to db.  
+ If response status is > 400, error body is logged  
+ If /api/ping is used shows aproox how long visitor stays on website
```
pub struct ParsedCheckIn {
    pub url: Option<String>,
    pub query: Option<String>,
    pub method: Option<String>,
    pub referrer: Option<String>,
    pub host: Option<String>,
    pub os: Option<String>,
    pub ua: Option<String>, 
    pub browser: Option<String>,  // PARSED UA
    pub is_bot: bool,             // GOOGLE OR FB CRAWLER
    pub lead: Option<String>,     // GOOGLE BING OR FB
    pub campaign: Option<String>, // utm_campaign
    pub anon_name: String,
    pub subdomain: String,
}
```  
***
  
# Concepts
+ subdomains are always authenticated.  
+ Allows only one subdomain level (one.domain.com) not one.two.subdomain.com  
+ Answers "Not authenticated" to all invalid requests to avoid enumeration.  
+ Serves same front end for all subdomains (optionally)  
+ Same (or different) backend can be used for all subdomains  
  
# Oppinionated parts:
Q: User registration gives an immediate cookie (instead of requiring log in).  
A: I've been in the position of asking for this feature "later" (when the auth was built already).  

Q: If client errors then the message body is logged to Writer.  
A: Previously customer support had to ask devops to find what customer error was.  

Q: Why is backend doing subodmains not DNS?  
A: in DNS all customer subdomains would be visible with a DNS query.  

A: logging system Writer uses UDP connection, which is unreliable compared to TCP.  
A: it could be easily swapped to TCP, at the moment I like UDP more, as it puts a bit less work to Centrale. Wouldn't want the authentication to break because logging needs resources.
  
Q: Token and cookie are both allowed for same endpoints  
A: It's double the work to make and maintain own endpoints for own and and client API,
    
Q: If centrale is down, whole service is immediately down    
A: It would be the same, if  auth server were down.    
    
Q: Why subdomains?   
A: API looks better.   
   
Q: Why not everyone in same db as in "normal SaaS".  
All subdomains have their own databases (with indivudual user + pass login for each) removing the need for separate ACL logic which would be needed if all data would be in same db.   
    
Q: Auth + proxy is done by the same service and they could interfere.    
A: I have no good solution for that yet but this question is on my mind.  
    
## NO AUTH endoints:
These endpoints have PUBLIC_RATE_LIMITER settings.
```
GET /api/tester - ensures website is up and db is working
POST /api/user_add - register user (password can be 1-200 chars, username can be 1-100 chars, must be alphanumeric)
POST /login - login
POST /api/ping - Used for statistics - expexts { url: "/", counter: 30 }
```
## AUTH endpoints
These endpoints have RATE_LIMITER settings.
```
GET /api/user/:user_id/bearer/generate - generate new bearer token for user, delete old. You can only generate and view once.
(TBD) DELETE /api/user/bearer/:id

GET /api/user - my user data
(TBD) GET /logout
(TBD) PUT /api/user - put user
(TBD) DELETE /api/user - delete user

POST /api/subdomain - add subdomain (subdomain is min 1, max 30 chars, if over, will be tructuated, must be alphanumeric)
GET /api/subdomain - get user subdomains
PUT /api/subdomain/:subdomain - edit subdomain
(TBD) DELETE /api/subdomain - delete subdomain
(TBD) GET /api/test-subdomain - see if subdomain is available

Subdomain specific:
GET /api/subdomain_user - users for subdomain
POST /api/subdomain_user - add / edit user or role to subdomain
PUT /api/subdomain_user - add user or role to subdomain
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
127.0.0.1   admin_data.mydomain.local
```

# Install
```
// curl https://sh.rustup.rs -sSf | sh
// sudo apt install build-essential
// apt install libssl-dev
// apt install pkg-config
// sudo apt-get install libsqlite3-dev
// sudo apt install cmake
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
EnvironmentFile=/home/www/centrale/.env
Restart=on-failure

[Install]
WantedBy=multi-user.target

--

sudo nano /etc/systemd/system/load_balancer.service

[Unit]
Description=Centrale Pingora load balancer
After=network.target centrale.service

[Service]
User=www
ExecStart=/home/www/centrale/target/release/load_balancer
EnvironmentFile=/home/www/centrale/.env
Restart=on-failure

[Install]
WantedBy=multi-user.target

---
setcap 'cap_net_bind_service=+ep' /home/www/centrale/target/release/load_balancer

---

sudo nano /etc/systemd/system/writer.service

[Unit]
Description=Writer
After=network.target writer.service

[Service]
User=www
ExecStart=/home/www/centrale/target/release/writer
EnvironmentFile=/home/www/centrale/.env
Restart=on-failure

[Install]
WantedBy=multi-user.target

---

debugging:  
source /etc/env  
---
sudo systemctl enable centrale
sudo systemctl start centrale

sudo systemctl enable load_balancer
sudo systemctl start load_balancer

sudo journalctl -u centrale.service
sudo journalctl -u centrale.service -f

sudo journalctl -u load_balancer.service
sudo journalctl -u load_balancer.service -f

sudo journalctl -u writer.service
sudo journalctl -u writer.service -f

sudo systemctl enable destination_server
sudo systemctl start destination_server

sudo systemctl enable writer
sudo systemctl start writer

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

# Tests
Some tests require destination server to run.  
  
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

# TBD
+ SET .well-known/apple-app-site-association
+ add auth methods - SMART ID, log in link, email, gmail.
+ Central key storage?
+ basic HTML to login... should be in "app"?

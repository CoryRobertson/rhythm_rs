;; example of quick deployment to a web server off site.
trunk build --release
docker build -t rhythm_rs .
docker save -o IMAGELOCATION rhythm_rs:latest
scp LOCATION DESTINATION
ssh SERVER docker load -i DESTINATION ON SERVER
ssh SERVER DEPLOYMENT DOCKER COMMAND
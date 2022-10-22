;; example of quick deployment to a web server off site.
trunk build --release
docker build -t rhythm_rs .
docker save -o IMAGELOCATION rhythm_rs:latest
ssh SERVER "cd LOCATIONOFCONTAINER ; docker-compose down ; docker image rm rhythm_rs"
scp IMAGELOCATION SERVER:LOCATIONOFCONTAINER
ssh SERVER docker load -i LOCATIONOFCONTAINER
ssh SERVER "cd LOCATIONOFCONTAINER ; docker-compose up -d"
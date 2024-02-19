cargo build --release
docker build -t snek$1 .
docker run -d -p $2:$2 -e SNEK=0.0.0.0:$2 --name snek$1 snek$1
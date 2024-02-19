cargo build --release
docker build -t snek$1 .
docker run -d -p $2:$2 -e SNEK=0.0.0.0:$2 snek$1
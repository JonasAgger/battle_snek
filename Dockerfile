FROM rust:1.76-slim

COPY target/release/battle_mulle . 

EXPOSE 8080

ENTRYPOINT ["./battle_mulle"]
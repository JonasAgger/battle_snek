FROM rust:1.76-slim

COPY target/release/battle_mulle . 

ENTRYPOINT ["./battle_mulle"]
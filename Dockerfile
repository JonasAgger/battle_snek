FROM scratch

COPY target/release/battle_mulle . 

EXPOSE 8080

ENTRYPOINT ["battle_mulle"]
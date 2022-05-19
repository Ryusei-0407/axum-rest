FROM rust:1.61.0-bullseye as build

RUN apt update &&\
    apt install -y build-essential git clang cmake libstdc++-10-dev libssl-dev libxxhash-dev zlib1g-dev pkg-config &&\
    git clone https://github.com/rui314/mold.git && cd mold &&\
    git checkout v1.0.3 &&\
    make -j$(nproc) CXX=clang++ &&\
    make install

WORKDIR /app

COPY . .

RUN mold -run cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build /app/target/release/app /

ARG DATABASE_URL

CMD ["./app"]

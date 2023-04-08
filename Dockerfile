## Use Rust stable as the base image
#FROM rust:latest
#
## Create a new directory for the application
#RUN mkdir /app
#WORKDIR /app
#
## Copy the Rocket project files into the container
#COPY . .
#
## Install the necessary dependencies
#RUN apt-get update && apt-get install -y pkg-config libssl-dev
#
## Build the application
#RUN cargo build --release
#
## Expose the default Rocket port
#EXPOSE 8000
#
## Start the application
#CMD ["./target/release/my_rocket_app"]


#FROM liuchong/rustup

FROM rust:latest

WORKDIR /app
#RUN apt update &&\
#    rm -rf ~/.cache &&\
#    apt clean all &&\
#    apt install -y cmake &&\
#    apt install -y clang


# install libtorch=1.9.0
# https://pytorch.org/get-started/locally/
# RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-1.9.0%2Bcpu.zip -O libtorch.zip
#RUN wget https://download.pytorch.org/libtorch/nightly/cpu/libtorch-shared-with-deps-latest.zip -O libtorch.zip
#RUN unzip -o libtorch.zip

#ENV LIBTORCH /app/libtorch
#ENV LD_LIBRARY_PATH /app/libtorch/lib:$LD_LIBRARY_PATH

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8080

ADD . /app

WORKDIR /app

RUN rustup default nightly

RUN cargo build --release

CMD ["cargo", "run", "--release"]
FROM ubuntu:18.04

WORKDIR /home/

COPY . .

ARG DEBIAN_FRONTEND=noninteractive

RUN bash ./setup.sh

ENV PATH="/root/.cargo/bin:$PATH"

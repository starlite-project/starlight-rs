FROM ubuntu:20.04

WORKDIR /home/

COPY . .

ENV TZ=America/New_York

RUN bash ./setup.sh

ENV PATH="/root/.cargo/bin:$PATH"

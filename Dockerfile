FROM ubuntu:23.04

RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Update new packages
RUN apt-get update

FROM python:3.11
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1
#ENV PYTHONPATH "${PYTHONPATH}:/usr/local/bin/python"
ENV PATH="/usr/local/bin:$PATH"





WORKDIR code/

COPY config/requirements.txt .
RUN pip install -r requirements.txt
COPY . /code/


ENV PATH="/root/.cargo/bin:$PATH"

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable

#RUN cargo install cargo-watch
#RUN cd /code/actix_backend && cargo build


FROM rust:1.67

WORKDIR /usr/src/grib2_meteo_charts

# copy sources
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# build dependencies
RUN cargo build --release

#CMD ["myapp"]

# Factorio Data Exporter Container
# Uses factoriotools/factorio as base to get Factorio + Space Age DLC
# renovate: datasource=docker depName=factoriotools/factorio
ARG FACTORIO_VERSION=2.0.73
FROM factoriotools/factorio:${FACTORIO_VERSION}

USER root

# Install Rust toolchain
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /exporter

# Copy exporter code
COPY . .

# Pre-download dependencies (cache layer)
RUN cargo fetch

# Build the exporter
RUN cargo build --release

# Set Factorio base directory
ENV FACTORIO_BASE_DIR=/opt/factorio

# Default command: run the exporter
CMD ["cargo", "run", "--release"]

FROM alpine:3.22 AS build

ARG TARGETARCH
ARG RELEASE=latest

RUN apk add --no-cache ca-certificates curl unzip
WORKDIR /build

RUN set -eux; \
    case "$TARGETARCH" in \
      amd64)  ARCH_SUBSTR="x86_64-musl"  ;; \
      arm64)  ARCH_SUBSTR="aarch64-musl" ;; \
      *) echo "Unsupported TARGETARCH=$TARGETARCH" >&2; exit 1 ;; \
    esac; \
    if [ "$RELEASE" = "latest" ]; then \
      TAG="$(curl -fsSL https://api.github.com/repos/nebulafx/nebulafx/releases \
              | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4 | head -n 1)"; \
    else \
      TAG="$RELEASE"; \
    fi; \
    echo "Using tag: $TAG (arch pattern: $ARCH_SUBSTR)"; \
    # Find download URL in assets list for this tag that contains arch substring and ends with .zip
    URL="$(curl -fsSL "https://api.github.com/repos/nebulafx/nebulafx/releases/tags/$TAG" \
           | grep -o "\"browser_download_url\": \"[^\"]*${ARCH_SUBSTR}[^\"]*\\.zip\"" \
           | cut -d'"' -f4 | head -n 1)"; \
    if [ -z "$URL" ]; then echo "Failed to locate release asset for $ARCH_SUBSTR at tag $TAG" >&2; exit 1; fi; \
    echo "Downloading: $URL"; \
    curl -fL "$URL" -o nebulafx.zip; \
    unzip -q nebulafx.zip -d /build; \
    # If binary is not in root directory, try to locate and move from zip to /build/nebulafx
    if [ ! -x /build/nebulafx ]; then \
      BIN_PATH="$(unzip -Z -1 nebulafx.zip | grep -E '(^|/)nebulafx$' | head -n 1 || true)"; \
      if [ -n "$BIN_PATH" ]; then \
        mkdir -p /build/.tmp && unzip -q nebulafx.zip "$BIN_PATH" -d /build/.tmp && \
        mv "/build/.tmp/$BIN_PATH" /build/nebulafx; \
      fi; \
    fi; \
    [ -x /build/nebulafx ] || { echo "nebulafx binary not found in asset" >&2; exit 1; }; \
    chmod +x /build/nebulafx; \
    rm -rf nebulafx.zip /build/.tmp || true


FROM alpine:3.22

ARG RELEASE=latest
ARG BUILD_DATE
ARG VCS_REF

LABEL name="NebulaFX" \
      vendor="NebulaFX Team" \
      maintainer="NebulaFX Team <dev@nebulafx.com>" \
      version="v${RELEASE#v}" \
      release="${RELEASE}" \
      build-date="${BUILD_DATE}" \
      vcs-ref="${VCS_REF}" \
      summary="High-performance distributed object storage system compatible with S3 API" \
      description="NebulaFX is a distributed object storage system written in Rust, supporting erasure coding, multi-tenant management, and observability." \
      url="https://nebulafx.com" \
      license="Apache-2.0"

RUN apk add --no-cache ca-certificates coreutils curl

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build /build/nebulafx /usr/bin/nebulafx
COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /usr/bin/nebulafx /entrypoint.sh

RUN addgroup -g 1000 -S nebulafx && \
    adduser -u 1000 -G nebulafx -S nebulafx -D && \
    mkdir -p /data /logs && \
    chown -R nebulafx:nebulafx /data /logs && \
    chmod 0750 /data /logs

ENV NEUBULAFX_ADDRESS=":9000" \
    NEUBULAFX_ACCESS_KEY="nebulafxadmin" \
    NEUBULAFX_SECRET_KEY="nebulafxadmin" \
    NEUBULAFX_EXTERNAL_ADDRESS="" \
    NEUBULAFX_CORS_ALLOWED_ORIGINS="*" \
    NEUBULAFX_CONSOLE_CORS_ALLOWED_ORIGINS="*" \
    NEUBULAFX_VOLUMES="/data" \
    RUST_LOG="warn" \
    NEUBULAFX_OBS_LOG_DIRECTORY="/logs" 
    
EXPOSE 9000 9001

VOLUME ["/data", "/logs"]

USER nebulafx

ENTRYPOINT ["/entrypoint.sh"]

CMD ["nebulafx"]

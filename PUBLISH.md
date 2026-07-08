
##
```shell
podman login docker.io

# amd64 + arm64 를 한 번에 빌드하고 매니페스트(멀티아치 이미지) 생성
podman build --memory=16g --memory-swap=-1 \
    --platform linux/amd64,linux/arm64 \
    --manifest docker.io/sarolab/dat-cms:latest .

# push (publish)
podman manifest push --all docker.io/sarolab/dat-cms:latest docker.io/sarolab/dat-cms:latest
podman manifest push --all docker.io/sarolab/dat-cms:latest docker.io/sarolab/dat-cms:4.3.4
```


## Create manifest
## ARM64
``` shell
podman login docker.io

# manifast create
podman manifest rm sarolab-dat-cms-manifest || true
podman manifest create sarolab-dat-cms-manifest

# build --platform linux/arm64
podman build --memory=10g --memory-swap=-1 \
    --platform linux/arm64 \
    --manifest sarolab-dat-cms-manifest -t sarolab/dat-cms:latest .

# push (publish)
podman manifest push sarolab-dat-cms-manifest docker.io/sarolab/dat-cms:latest
```
## AMD64
```
podman login docker.io

# manifast create / integrate
podman manifest rm sarolab-dat-cms-manifest || true
podman manifest create sarolab-dat-cms-manifest
podman manifest add --all sarolab-dat-cms-manifest docker://docker.io/sarolab/dat-cms:latest

# build --platform linux/amd64
podman build --memory=32g --memory-swap=-1 \
    --platform linux/amd64 \
    --manifest sarolab-dat-cms-manifest -t sarolab/dat-cms:latest .

# push (publish)
podman manifest push sarolab-dat-cms-manifest docker.io/sarolab/dat-cms:latest
podman manifest push sarolab-dat-cms-manifest docker.io/sarolab/dat-cms:4.3.4
```

## remove
```
docker rmi -f $(docker images -q sarolab/dat-cms)
```
```shell
podman run -d -p 8088:80 --name dat-cms-container sarolab/dat-cms
curl localhost:8088
podman logs -f dat-cms-container

podman stop dat-cms-container
podman rm dat-cms-container

```

## Configuration
### Mac
```
# 맥은 podman이 리눅스 가상머신에서 돌아가기 때문에
# --memory 명령을 주기위해선 podman 자체의 메모리를 늘려야한다.
podman machine stop
podman machine set --memory 8000
podman machine start
```



## Configuration - build
```
# cross install
cargo install cross --git https://github.com/cross-rs/cross

# linux, mac
export CROSS_CONTAINER_ENGINE=podman

# windows ps
$env:CROSS_CONTAINER_ENGINE="podman"
```

```
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=global-registry \
    --mount=type=cache,target=/usr/local/cargo/git,id=global-git \
    --mount=type=cache,target=/work/target,id=saro-lab-dat-cms--${APP} \
    RUST_TARGET="$(uname -m)-unknown-linux-musl" && \
    RUSTFLAGS="-C target-feature=+crt-static" \
    cargo build --release --target "${RUST_TARGET}" && \
    cp "target/${RUST_TARGET}/release/dat-cms" /app/dat-cms-bin
```

```
# build
podman build --build-arg APP=dat-cms -t dat-cms:test .
# run
podman run -d -p 8088:80 --name dat-cms-container dat-cms:test
# ps
podman ps
# delete
podman rm -f dat-cms-container
```

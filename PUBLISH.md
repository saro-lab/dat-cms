## publish
```shell
podman login docker.io

# amd64 + arm64 를 한 번에 빌드하고 매니페스트(멀티아치 이미지) 생성
podman build --memory=16g --memory-swap=-1 \
    --platform linux/amd64,linux/arm64 \
    --manifest docker.io/sarolab/dat-cms:latest .

# push (publish)
podman manifest push --all docker.io/sarolab/dat-cms:latest docker.io/sarolab/dat-cms:latest
podman manifest push --all docker.io/sarolab/dat-cms:latest docker.io/sarolab/dat-cms:4.3.5
```

## manifest
```shell
# remove
podman manifest rm sarolab-dat-cms-manifest || true
# create
podman manifest create sarolab-dat-cms-manifest
# add
podman manifest add --all sarolab-dat-cms-manifest docker://docker.io/sarolab/dat-cms:latest
# push
podman manifest push sarolab-dat-cms-manifest docker.io/sarolab/dat-cms:latest
```

## cargo update
```shell
# cargo install cargo-edit
cargo upgrade
```
## publish
```shell
podman login docker.io
podman manifest rm docker.io/sarolab/dat-cms:latest 2>/dev/null || true
podman manifest rm sarolab-dat-cms-manifest || true

podman build --memory=16g --memory-swap=-1 \
    --platform linux/amd64,linux/arm64 \
    --manifest docker.io/sarolab/dat-cms:latest .

podman manifest push --all docker.io/sarolab/dat-cms:latest
podman manifest push --all docker.io/sarolab/dat-cms:latest docker.io/sarolab/dat-cms:4.3.5
```

## cargo update
```shell
# cargo install cargo-edit
cargo upgrade
```

# harbor_rs

[Harbor](https://goharbor.io/) SDK for Rust

```toml
[dependencies]
harbor_rs = "0.1.1"
```

## clean-harbor-images

根据时间间隔清理 [Harbor](https://goharbor.io/) 上的镜像 Tag

```yaml
harbor_base_api: "https://example.com/api"
harbor_username: "admin"
harbor_password": "admin"
clean_interval": 20 # 清理 20 天前构建的镜像
repos:
  - "test/image1"
  - "test/image2"
```

## LICENSE

[MIT](https://github.com/k8scat/harbor_rs/blob/main/LICENSE)

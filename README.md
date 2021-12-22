# harbor_rs

> Harbor API Version: v1.10.0, [v2.0](https://editor.swagger.io/?url=https://raw.githubusercontent.com/goharbor/harbor/master/api/v2.0/swagger.yaml) is coming soon.

[Harbor](https://goharbor.io/) API in Rust.

```toml
[dependencies]
harbor_rs = "0.1.2"
```

## clean-image-tags

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

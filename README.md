[![crates.io](https://img.shields.io/crates/v/bollard.svg)](https://crates.io/crates/harbor_rs)
[![license](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![circle-ci](https://circleci.com/gh/fussybeaver/bollard/tree/master.svg?style=svg)](https://circleci.com/gh/fussybeaver/bollard/tree/master)
[![appveyor](https://ci.appveyor.com/api/projects/status/n5khebyfae0u1sbv/branch/master?svg=true)](https://ci.appveyor.com/project/fussybeaver/boondock)
[![docs](https://docs.rs/bollard/badge.svg)](https://docs.rs/bollard/)

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

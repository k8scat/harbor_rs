version = 0.1.1
upgrade:
	sed -i "" 's/^version = "[0-9]*.[0-9]*.[0-9]*"/version = "${version}"/' Cargo.toml
	sed -i "" 's/harbor_rs = "[0-9]*.[0-9]*.[0-9]*"/harbor_rs = "${version}"/g' README.md
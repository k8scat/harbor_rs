version = 0.1.2
upgrade:
	sed -i "" 's/^version = "[0-9]*.[0-9]*.[0-9]*"/version = "${version}"/' Cargo.toml
	sed -i "" 's/harbor_rs = "[0-9]*.[0-9]*.[0-9]*"/harbor_rs = "${version}"/g' README.md

tests:
	# user tests ordered
	cargo test --color=always --package harbor_rs --lib harbor::user::tests::get_current_user
	cargo test --color=always --package harbor_rs --lib harbor::user::tests::get_user_profile
	cargo test --color=always --package harbor_rs --lib harbor::user::tests::create_user
	cargo test --color=always --package harbor_rs --lib harbor::user::tests::list_users
	cargo test --color=always --package harbor_rs --lib harbor::user::tests::search_users
	cargo test --color=always --package harbor_rs --lib harbor::user::tests::update_user_profile
	cargo test --color=always --package harbor_rs --lib harbor::user::tests::delete_user
	cargo test --color=always --package harbor_rs --lib harbor::user::tests::list_current_user_permissions -- --show-output

	# project tests ordered
	cargo test --color=always --package harbor_rs --lib harbor::project::tests::list_projects

build-clean-image-tags:
	cargo build --release --all-features --bin clean-image-tags

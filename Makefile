clippy:
	cargo clippy

minimal:
	cargo run --example minimal

mode:
	cargo run --example mode

VERSION_FILE := Cargo.toml

publish:
	sed -i -r "s/version=\"0\.0\.0\"/version=\"${VERSION}\"/g" $(VERSION_FILE) \
	  && cargo publish --allow-dirty

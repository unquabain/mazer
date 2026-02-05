dev:
	trunk serve
build:
	trunk clean
	trunk build --release
build-image:
	docker build \
		--platform linux/amd64 \
		-t unquabain/mazer:v$(fq -r -d toml .package.version Cargo.toml) \
		-t unquabain/mazer:$(git log -1 --format=format:%cs)  \
		-t unquabain/mazer:$(git log -1 --format=format:%h) \
		-t unquabain/mazer:latest \
		-t mazer:v$(fq -r -d toml .package.version Cargo.toml) \
		-t mazer:$(git log -1 --format=format:%cs)  \
		-t mazer:$(git log -1 --format=format:%h) \
		-t mazer:latest \
		.
run-image:
	docker run --name mazer --rm --detach -p 8989:80 --platform linux/amd64 unquabain/mazer:latest
	open http://localhost:8989

push-image:
	docker push unquabain/mazer:v$(fq -r -d toml .package.version Cargo.toml)
	docker push unquabain/mazer:$(git log -1 --format=format:%cs)
	docker push unquabain/mazer:$(git log -1 --format=format:%h)
	docker push unquabain/mazer:latest

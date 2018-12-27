# https://docs.docker.com/engine/reference/commandline/build/#parent-command
docker build -t raspberry-build .

# https://hackernoon.com/seamlessly-cross-compiling-rust-for-raspberry-pis-ede5e2bd3fe2
# https://matthiasnoback.nl/2017/04/docker-build-patterns/
# https://stackoverflow.com/questions/8426058/getting-the-parent-of-a-directory-in-bash
docker run \
	--name docker-tutorial \
	--rm \
	--mount type=bind,source="$(dirname "$(pwd)")",target=/app \
	--mount type=bind,source="$(echo $HOME)"/raspberry-build/cargo/registry,target=/usr/local/cargo/registry \
	--shm-size=1g \
	raspberry-build
	
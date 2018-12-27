# https://docs.docker.com/engine/reference/commandline/build/#parent-command
docker build -t raspberry-build .

# Create directory for storing cargo registry on host, if it does not exist already
registry="$(echo $HOME)"/raspberry-build/cargo/registry
mkdir -p $registry

# https://hackernoon.com/seamlessly-cross-compiling-rust-for-raspberry-pis-ede5e2bd3fe2
# https://matthiasnoback.nl/2017/04/docker-build-patterns/
# https://stackoverflow.com/questions/8426058/getting-the-parent-of-a-directory-in-bash
docker run \
	--name raspberry-build \
	--rm \
	--mount type=bind,source="$(dirname "$(pwd)")",target=/app \
	--mount type=bind,source=$registry,target=/usr/local/cargo/registry \
	--shm-size=1g \
	raspberry-build

.PHONY: release build-image image deploy-envoy clean

FILTER_NAME=wasm-locality-attribute
FILTER_TAG=v0.1
IMAGE ?= kirecek/$(FILTER_NAME):$(FILTER_TAG)

release:
	cargo wasi build --release

build-image:
	buildah build -t $(IMAGE)

build-release-image: release build-image

push-image:
	buildah push $(IMAGE)

clean:
	cargo clean

build.imge:
	docker build --tag rust-wasm:latest .

generate.project:
	docker run --rm --name wasm -it --user root -v ${PWD}:/app -w /app rust-wasm:latest cargo generate --git https://github.com/rustwasm/wasm-pack-template

wasm.cargo:
	# docker run --rm --name wasm-service -it --user root -v ${PWD}/wasm-game-of-life:/app -w /app rust-wasm:latest cargo install --path /app
	docker run --rm --name wasm-service -it --user root -v ${PWD}/wasm-game-of-life:/app -w /app rust-wasm:latest bash

wasm.build:
	docker run --rm --name wasm-service -it --user root -v ${PWD}/wasm-game-of-life/.cache:/root/.cache -v ${PWD}/wasm-game-of-life:/app -w /app rust-wasm:latest wasm-pack build

wasm.app.install:
	docker run --rm --name wasm -it --user root -v ${PWD}/wasm-game-of-life:/app -w /app/app rust-wasm:latest npm install

wasm.app.build:
	docker run --rm --name wasm -it --user root -v ${PWD}/wasm-game-of-life:/app -w /app/app rust-wasm:latest npm run build

wasm.app.dev:
	docker run --rm --name wasm -it --user root -v ${PWD}/wasm-game-of-life:/app -w /app/app -p 8080:8080 -e 8080 rust-wasm:latest npm run start

wasm.app.bash:
	docker run --rm --name wasm -it --user root -v ${PWD}/wasm-game-of-life:/app -w /app/app rust-wasm:latest bash

wasm.app.serve: wasm.app.build
	docker run --rm --name wasm -it --user root -v ${PWD}/wasm-game-of-life:/app -w /app/app -p 8080:8080 -e 8080 rust-wasm:latest npm run serve
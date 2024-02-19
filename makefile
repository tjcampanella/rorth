FORCE:

commit: FORCE lint tests build
	git add .
	git commit -a

prod: FORCE commit
	git push

run: FORCE clean
	cargo run

tests: FORCE
	cargo build --release
	cargo run --release --bin test examples

test_record: FORCE
	cargo build --release
	cargo run --release --bin test record examples

lint: FORCE
	cargo clippy --all-targets --color always  --allow-dirty --allow-staged --fix -- -D warnings -D clippy::pedantic -D clippy::nursery -D clippy::unwrap_used -D clippy::expect_used

build: FORCE lint tests
	cargo build

build_release: FORCE lint tests
	cargo build --release

compile_asm:
	as -arch arm64 -o out.o out.s
	ld -o out out.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64

compile_run: compile_asm
	./out





fzfmenu_source_code:=$(shell find ./src)
fzfmenu_target_file:=./target/release/fzfmenu

install: build
	sudo install -d /usr/share/zsh/site-functions
	sudo install -Dm644 ./assets/completions/zsh/* /usr/share/zsh/site-functions/
	sudo install ./target/release/fzfmenu /usr/bin/

build: $(fzfmenu_target_file)

$(fzfmenu_target_file): $(fzfmenu_source_code)
	cargo build --release

.PHONY: install

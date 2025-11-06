install:
	sudo install -d /usr/share/zsh/site-functions
	sudo install -Dm644 ./assets/completions/zsh/* /usr/share/zsh/site-functions/

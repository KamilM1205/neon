BUILD_FOLDER = build/debug/

debug:
	cargo build
	mkdir -p $(BUILD_FOLDER)
	cp -f target/debug/neon $(BUILD_FOLDER)
	cp -R plugins/ $(BUILD_FOLDER)/plugins/
	cp -R config/ $(BUILD_FOLDER)/config/
	cp -R themes/ $(BUILD_FOLDER)/themes/

all:
	debug
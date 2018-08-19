build:
	mkdir build
	cd ssdpserver;\
	cargo build --release;
	cp ssdpserver/target/release/ssdpserver build
	cd soapserver;\
	cargo build --release;
	cp soapserver/target/release/soapserver build

run: build
	build/ssdpserver &
	build/soapserver &

clean:
	rm -rf build

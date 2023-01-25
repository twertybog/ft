RC=cargo

FLAGS=+nightly build -Z build-std=std,panic_abort --target

MODE=--release

CCLINUXARM64=aarch64-linux-gnu-gcc
CXXLINUXARM64=aarch64-linux-gnu-g++

CCAPPLEARM64=aarch64-apple-darwin20.2-cc
CXXAPPLEARM64=aarch64-apple-darwin20.2-c++

CCAPPLEX86=x86_64-apple-darwin20.2-cc
CXXAPPLEX86=x86_64-apple-darwin20.2-c++

all: linux_x86 linux_aarch64 windows_x86 apple_x86 apple_aarch64
	
linux_x86:
	$(RC) $(FLAGS) x86_64-unknown-linux-musl $(MODE)

linux_aarch64:
	CC=$(CCLINUXARM64) CXX=$(CXXLINUXARM64) $(RC) $(FLAGS) aarch64-unknown-linux-musl $(MODE)
	
windows_x86:
	$(RC) $(FLAGS) x86_64-pc-windows-gnu $(MODE)

apple_x86:
	CC=$(CCAPPLEX86) CXX=$(CXXAPPLEX86) $(RC) $(FLAGS) x86_64-apple-darwin $(MODE)

apple_aarch64:
	CC=$(CCAPPLEX86) CXX=$(CXXAPPLEARM64) $(RC) $(FLAGS) x86_64-apple-darwin $(MODE)
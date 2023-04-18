DESTDIR =
prefix = /usr/local
bindir = $(prefix)/bin
INSTALL = install
INSTALL_PROGRAM = $(INSTALL) -m 755

all: build

build:
	cargo build --release
	cp ./target/release/sensi-mouse ./sensi-mouse

install:
	$(INSTALL_PROGRAM) sensi-mouse $(DESTDIR)$(bindir)/sensi-mouse

uninstall:
	$(RM) $(DESTDIR)$(bindir)/sensi-mouse

# add launch service
launchd-add:
	cp launch.plist ~/Library/LaunchAgents/com.relicx-me.sensi-mouse-cli.plist
	launchctl load ~/Library/LaunchAgents/com.relicx-me.sensi-mouse-cli.plist

# remove launch service
launchd-remove:
	launchctl unload ~/Library/LaunchAgents/com.relicx-me.sensi-mouse-cli.plist
	$(RM) ~/Library/LaunchAgents/com.relicx-me.sensi-mouse-cli.plist

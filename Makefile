DESTDIR =
prefix = /usr/local
bindir = $(prefix)/bin
INSTALL = install
INSTALL_PROGRAM = $(INSTALL) -m 755

all: build

build:
	cargo build --release
	cp ./target/release/sensi-mouse-cli ./sensi-mouse

install:
	$(INSTALL_PROGRAM) sensi-mouse $(DESTDIR)$(bindir)/sensi-mouse

uninstall:
	$(RM) $(DESTDIR)$(bindir)/sensi-mouse

# add launch service
launchd-add:
	cp launch-agent.plist ~/Library/LaunchAgents/com.relicx-me.mousetune.plist
	launchctl load ~/Library/LaunchAgents/com.relicx-me.mousetune.plist

# remove launch service
launchd-remove:
	launchctl unload ~/Library/LaunchAgents/com.relicx-me.mousetune.plist
	$(RM) ~/Library/LaunchAgents/com.relicx-me.mousetune.plist

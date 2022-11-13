.PHONY: build build-release-tar build-release-zip check fmt install-local publish-all run-integration

build:
	cargo build

build-release-tar:
	cd $(target)-$(tag)-bin && \
		chmod +x sps && \
		tar czvf sps-$(tag).$(target).tar.gz sps && \
		shasum -a 256 sps-$(tag).$(target).tar.gz > sps-$(tag).$(target).tar.gz.sha256 && \
		mv *.tar.gz* .. && cd ..

build-release-zip:
	cd $(target)-$(tag)-bin && \
		zip sps-$(tag).$(target).zip sps.exe && \
		shasum -a 256 sps-$(tag).$(target).zip > sps-$(tag).$(target).zip.sha256 && \
		mv *.zip* .. && cd ..

check:
	cargo check
	cargo +nightly udeps

fmt:
	cargo +nightly fmt --all

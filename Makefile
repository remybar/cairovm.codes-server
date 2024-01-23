deps:
	git clone https://github.com/starkware-libs/cairo.git \
	&& cd cairo \
	&& git checkout v2.5.0 \
	&& cd .. \
	&& mv cairo/corelib/ . \
	&& rm -rf cairo/
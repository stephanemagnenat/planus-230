src/planus_serialization.rs: schema/condition.fbs
	planus rust -o $@ schema/condition.fbs

generated: src/planus_serialization.rs

install-planus-cli:
	cargo install planus-cli@0.4.0

.PHONY: generated install-planus-cli
EXAMPLE := "examples/reference/PING_v0.1.0.msi"
default:
    just --list

tables:
    msiinfo tables {{EXAMPLE}} | sort
table TABLE:
    msiinfo export -s {{EXAMPLE}} {{TABLE}}

streams:
    msiinfo streams {{EXAMPLE}} | sort
stream STREAM:
    msiinfo extract {{EXAMPLE}} {{STREAM}} > {{STREAM}}.cab

alias tp: test-print
test-print:
    cargo test -- --nocapture

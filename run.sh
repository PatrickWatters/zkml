#!/bin/bash
#ps -ef | grep "cms_server.py" | awk '{print $2}' | xargs sudo kill

for i in {1..10}; do
    cargo run --package zkml --bin time_circuit

done
#!/usr/bin/env bash
set -e

ELF=$(realpath "$1")
DIR=$(cd "$(dirname "$0")" && pwd)

TMPFILE=$(mktemp /tmp/renode-XXXXXX.resc)
trap 'rm -f "$TMPFILE"' EXIT

cat > "$TMPFILE" <<EOF
mach create "hello-m4"
machine LoadPlatformDescription @$DIR/hello_m4.repl
showAnalyzer sysbus.uart0 Antmicro.Renode.Analyzers.LoggingUartAnalyzer
macro reset
"""
    sysbus LoadELF @$ELF
"""
runMacro \$reset
start
EOF

exec timeout 30 renode --console "$TMPFILE"

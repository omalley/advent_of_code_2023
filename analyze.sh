#!/bin/bash
# This script analyzes the times and normalizes them to be in seconds.
gawk 'BEGIN{times["µs"] = 0.000001; times["s"] = 1.0; times["ms"] = 0.001} /Day/{match($3, /([0-9.]+)([^0-9]?s)/, pats); printf "%10.6f %4s\n", pats[1] * times[pats[2]], $2}' | sort -nr

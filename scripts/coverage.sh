#!/usr/bin/env bash
# Runs xUnit tests with Coverlet coverage and enforces a minimum threshold.

LIMIT=95

INFO='\033[0;34m'
ERROR='\033[0;31m'
SUCCESS='\033[0;32m'
DEFAULT='\033[0m'

# Clean up old test results and build first
echo "Building and running tests with coverage..."
rm -rf ./tests/TestResults/
dotnet build --no-restore > /dev/null 2>&1
dotnet test --collect:"XPlat Code Coverage" --logger "console;verbosity=quiet"
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo -e "${ERROR}Tests failed.${DEFAULT}"
    exit $EXIT_CODE
fi

# Find the most recent coverage report
COVERAGE_FILE=$(find ./tests/TestResults -name 'coverage.cobertura.xml' -type f | head -n 1)
if [ ! -f "$COVERAGE_FILE" ]; then
    echo -e "${ERROR}Coverage report not found in ./tests/TestResults${DEFAULT}"
    exit 2
fi

# Extract line coverage percentage - handle decimal line-rate values
LINE_RATE=$(grep -oP 'line-rate="\K[0-9.]+' "$COVERAGE_FILE" | head -n1)
if [ -z "$LINE_RATE" ]; then
    echo -e "${ERROR}Could not extract line-rate from $COVERAGE_FILE${DEFAULT}"
    exit 3
fi

# Convert to percentage using awk (multiply by 100 and round)
PERCENTAGE=$(echo "$LINE_RATE" | awk '{printf("%.0f", $1*100)}')

echo -e "${INFO}Line coverage: $PERCENTAGE% (from rate: $LINE_RATE)${DEFAULT}"

if [ "$PERCENTAGE" -lt "$LIMIT" ]; then
    echo -e "${ERROR}Test coverage is below $LIMIT% ($PERCENTAGE%).${DEFAULT}"
    echo -e "${INFO}Info:${DEFAULT} Add more tests to increase coverage."
    exit 1
else
    echo -e "${SUCCESS}Coverage meets threshold of $LIMIT%.${DEFAULT}"
fi
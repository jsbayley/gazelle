# Counts the lines of F# code across the Gazelle project. Whilst line count 
# is not a perfect metric for complexity, it can be a useful heuristic to 
# suggest that a codebase has become too complex given the scoped software 
# requirements.

lines_of_code=$(cloc ./src ./cli ./tests --include-ext=fs --csv --quiet | grep SUM | awk -F, '{print $5}')

echo "$lines_of_code lines of F# code across cli, src, and tests directories."
echo ""
echo "  src: $(cloc ./src --include-ext=fs --csv --quiet | grep SUM | awk -F, '{print $5}')."
echo "  tests: $(cloc ./tests --include-ext=fs --csv --quiet | grep SUM | awk -F, '{print $5}')."
echo "  cli: $(cloc ./cli --include-ext=fs --csv --quiet | grep SUM | awk -F, '{print $5}')."
echo ""

RED='\033[0;31m'
DEFAULT='\033[0m'
loc_limit=2500

if [ $lines_of_code -gt $loc_limit ]; then
  echo -e "${RED}Line count exceeded imposed limit: $loc_limit lines.${DEFAULT}"
  exit 1
fi
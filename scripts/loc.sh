# Counts the lines of F# code across the Gazelle project. Whilst line count 
# is not a perfect metric for complexity, it can be a useful heuristic to 
# suggest that a codebase has become too complex given the scoped software 
# requirements.

loc_limit=2500
RED='\033[0;31m'
DEFAULT='\033[0m'

loc=$(cloc ./src ./cli ./tests --include-ext=fs --csv --quiet | awk -F, '/SUM/ {print $5}')

echo "$loc lines of F# code across cli, src, and tests directories."
echo ""

for dir in src tests cli; do
  count=$(cloc ./$dir --include-ext=fs --csv --quiet | awk -F, '/SUM/ {print $5}')
  echo "  $dir: $count."
done
echo ""

if [ "$loc" -gt "$loc_limit" ]; then
  echo -e "${RED}Line count exceeded imposed limit: $loc_limit lines.${DEFAULT}"
  exit 1
fi
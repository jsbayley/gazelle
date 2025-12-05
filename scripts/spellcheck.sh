# Runs the hunspell spell checker against README file.

DICTIONARY="en_GB"
ALLOWED_WORDS=".hunspell/custom.dic"
FILES=$(find . -maxdepth 3 -name "*.md")
TEMP_OUTPUT=$(mktemp)

# Colour codes.
INFO='\033[0;34m'
ERROR='\033[0;31m'
DEFAULT='\033[0m'

hunspell -d $DICTIONARY -i utf-8 -p $ALLOWED_WORDS -l $FILES > $TEMP_OUTPUT

if [ -s $TEMP_OUTPUT ]; then
    echo -e "${ERROR}Spelling errors found:${DEFAULT}\n"

    while read -r line; do
      echo -e "${line}"
    done < $TEMP_OUTPUT

    user_note="If you believe a spelling to be correct, \
you can add the word to the custom dictionary \
found here: './.hunspell/custom.dic'."

    echo -e "\n${INFO}Info: ${DEFAULT}$user_note"
    exit 1
else
    echo "No spelling errors found."
    exit 0
fi

rm -f $TEMP_OUTPUT
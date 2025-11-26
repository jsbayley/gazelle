#!/bin/bash
# update-version.sh - Update version across all project files

if [ -z "$1" ]; then
    echo "Usage: ./update-version.sh <new-version>"
    echo "Example: ./update-version.sh 0.0.4"
    exit 1
fi

NEW_VERSION="$1"
CURRENT_VERSION=$(grep -o '<Version>.*</Version>' Directory.Build.props | sed 's/<Version>\(.*\)<\/Version>/\1/')

echo "Updating version from $CURRENT_VERSION to $NEW_VERSION..."

# Update Directory.Build.props
sed -i "s/<Version>$CURRENT_VERSION<\/Version>/<Version>$NEW_VERSION<\/Version>/" Directory.Build.props

# Update documentation
find . -name "*.md" -exec sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" {} \;
find . -name "*.md" -exec sed -i "s/v$CURRENT_VERSION/v$NEW_VERSION/g" {} \;

# Update website
find docs/ -name "*.html" -exec sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" {} \;
find docs/ -name "*.html" -exec sed -i "s/v$CURRENT_VERSION/v$NEW_VERSION/g" {} \;

echo "Version updated successfully!"
echo "Don't forget to:"
echo "1. dotnet build"  
echo "2. git add ."
echo "3. git commit -m 'Update version to $NEW_VERSION'"
echo "4. git tag v$NEW_VERSION"
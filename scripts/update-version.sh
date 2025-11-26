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

# Update Directory.Build.props (central version management)
sed -i "s/<Version>$CURRENT_VERSION<\/Version>/<Version>$NEW_VERSION<\/Version>/" Directory.Build.props

# Update documentation
find . -name "*.md" -exec sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" {} \;
find . -name "*.md" -exec sed -i "s/v$CURRENT_VERSION/v$NEW_VERSION/g" {} \;

# Update website (handle both plain and v-prefixed versions)
find docs/ -name "*.html" -exec sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" {} \;
find docs/ -name "*.html" -exec sed -i "s/v$CURRENT_VERSION/v$NEW_VERSION/g" {} \;
# Also handle escaped versions in HTML/JSON
find docs/ -name "*.html" -exec sed -i "s/\"$CURRENT_VERSION\"/\"$NEW_VERSION\"/g" {} \;

# Update AI integration documentation
find ai-agents/ -name "*.md" -exec sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" {} \;

echo "Version updated successfully!"
echo ""
echo "Next steps:"
echo "1. Build and test:"
echo "   dotnet build"
echo "   dotnet test"
echo "2. Update CLI tool:"
echo "   cd cli && dotnet pack"
echo "   dotnet tool update --global --add-source ./bin/Release Gazelle.CLI"
echo "3. Commit changes:"
echo "   git add ."
echo "   git commit -m 'Update version to $NEW_VERSION'"
echo "   git tag v$NEW_VERSION"
echo "4. Push:"
echo "   git push origin main --tags"
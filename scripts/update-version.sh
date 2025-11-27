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
echo "Updating documentation files..."
find . -name "*.md" -exec sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" {} \;
find . -name "*.md" -exec sed -i "s/v$CURRENT_VERSION/v$NEW_VERSION/g" {} \;

# Explicitly update NuGet package README (src/README.md)
echo "Updating NuGet package README..."
if [ -f "src/README.md" ]; then
    sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" src/README.md
    sed -i "s/v$CURRENT_VERSION/v$NEW_VERSION/g" src/README.md
    echo "✓ Updated src/README.md (NuGet package README)"
else
    echo "⚠ src/README.md not found - NuGet package README may need manual review"
fi

# Update website (handle both plain and v-prefixed versions)
echo "Updating website files..."
if [ -d ".github/pages/" ]; then
    WEBSITE_DIR=".github/pages"
else
    echo "⚠ No website directory found (looking for .github/pages/)"
    WEBSITE_DIR=""
fi

if [ -n "$WEBSITE_DIR" ]; then
    find "$WEBSITE_DIR/" -name "*.html" -exec sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" {} \;
    find "$WEBSITE_DIR/" -name "*.html" -exec sed -i "s/v$CURRENT_VERSION/v$NEW_VERSION/g" {} \;
    # Also handle escaped versions in HTML/JSON
    find "$WEBSITE_DIR/" -name "*.html" -exec sed -i "s/\"$CURRENT_VERSION\"/\"$NEW_VERSION\"/g" {} \;
    echo "✓ Updated website files in $WEBSITE_DIR/"
fi

# Update AI integration documentation
echo "Updating AI integration documentation..."
find ai-agents/ -name "*.md" -exec sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" {} \;

echo ""
echo "Verifying NuGet package configuration..."
if grep -q "PackageReadmeFile.*README.md" src/Gazelle.fsproj && [ -f "src/README.md" ]; then
    echo "✓ NuGet package README configuration verified"
else
    echo "⚠ NuGet package README configuration may need attention"
fi

echo ""
echo "🎉 Version updated successfully from $CURRENT_VERSION to $NEW_VERSION!"
echo ""
echo "📋 Next steps:"
echo "1. Build and test:"
echo "   dotnet build"
echo "   dotnet test"
echo "2. Update CLI tool:"
echo "   cd cli && dotnet pack"
echo "   dotnet tool update --global --add-source ./bin/Release Gazelle.CLI"
echo "3. Verify NuGet package:"
echo "   dotnet pack src/ --configuration Release"
echo "   # Check generated .nupkg contains updated README"
echo "4. Commit changes:"
echo "   git add ."
echo "   git commit -m 'Update version to $NEW_VERSION'"
echo "   git tag v$NEW_VERSION"
echo "5. Push:"
echo "   git push origin main --tags"
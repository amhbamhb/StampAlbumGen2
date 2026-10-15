# Format code and fix common issues before committing.
# Format code and fix common issues before committing.
# Also runs git commitlint validation on staged/uncommitted changes.
# Also runs git commitlint validation on staged/uncommitted changes.


set -e  # Exit on any error



# Default timeout for running the test-suite (seconds). Can be overridden by
# Default timeout for running the test-suite (seconds). Can be overridden by
# setting TEST_TIMEOUT in the environment when invoking the script.
# setting TEST_TIMEOUT in the environment when invoking the script.
TEST_TIMEOUT=${TEST_TIMEOUT:-600}


echo "🔎 Verifying config baseline files..."

if [ ! -f "config_template.toml" ]; then

    echo "❌ Missing config_template.toml in repo root."
    
    echo "Please restore it before running checks."
  

    exit 1

fi



if [ ! -f "config.toml" ]; then
    echo "🛠️  config.toml missing; creating it from config_template.toml"
   
    cp "config_template.toml" "config.toml"
    

fi


echo "✅ Config baseline files present"

echo ""



echo "🔍 Checking current formatting status..."



# First check if formatting is needed

if cargo fmt --all -- --check > /dev/null 2>&1; then
    echo "✅ Code is already properly formatted!"


else
    echo "🔧 Formatting Rust code..."



    # Show what files need formatting
    # Show what files need formatting
  
    echo "Files that need formatting:"

    cargo fmt --all -- --check 2>&1 | grep "Diff in" | sed 's/Diff in /  • /' || true

    echo ""


    # Actually format the code
    # Actually format the code

    cargo fmt --all

    echo "✅ Code has been formatted"
fi



echo ""

echo "🔍 Verifying formatting is now correct..."

if ! cargo fmt --all -- --check; then
    echo "❌ Formatting verification failed!"

    echo "There may be syntax errors preventing proper formatting."
    exit 1


fi



echo "✅ Formatting verification passed"
echo ""




echo "📋 Running a quick compilation check..."

if ! cargo check ${CARGO_FEATURES} --quiet; then
    echo "❌ Code doesn't compile!"

    echo "Please fix compilation errors before committing."
    exit 1

fi


echo "✅ Code compiles successfully"

echo ""

# ─── NEW: Git commit linting step ──────────────────────────────
echo "🔍 Running git commitlint validation..."

# Determine the range of commits to validate
LAST_COMMIT=$(git log -1 --format="%H" HEAD || echo "None")
if [ "$LAST_COMMIT" = "None" ]; then
    PREV_COMMIT=""
else
    PREV_COMMIT=$(git log -2 --format="%H" HEAD | tail -1)
fi

# Build range: from the last committed SHA (exclusive) to current HEAD
if [ -n "$PREV_COMMIT" ]; then
    COMMIT_RANGE="$PREV_COMMIT..HEAD"
else
    COMMIT_RANGE="HEAD"
fi
## NEW: Git commitlint validation step
if [ -n "$COMMIT_RANGE" ]; then
    npx --no-install commitlint --from "$PREV_COMMIT" --to HEAD || true
fi

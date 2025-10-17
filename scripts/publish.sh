#!/usr/bin/env bash
#
# Publish tuxtui crates to crates.io in the correct dependency order
#
# Usage:
#   ./scripts/publish.sh           # Dry run (test without publishing)
#   ./scripts/publish.sh --execute # Actually publish

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DRY_RUN=true
WAIT_TIME=120  # Seconds to wait between publishes

# Parse arguments
if [[ "${1:-}" == "--execute" ]]; then
    DRY_RUN=false
    echo -e "${YELLOW}⚠️  EXECUTE MODE - Will actually publish to crates.io!${NC}"
    read -p "Are you sure? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        echo "Aborted."
        exit 1
    fi
else
    echo -e "${BLUE}🧪 DRY RUN MODE - Testing without publishing${NC}"
    echo "Use './scripts/publish.sh --execute' to actually publish"
    echo ""
fi

# Crates to publish in dependency order
CRATES=(
    "tuxtui-core"
    "tuxtui-macros"
    "tuxtui-widgets"
    "tuxtui-crossterm"
    "tuxtui-termion"
    "tuxtui-termwiz"
    "tuxtui"
)

# Function to publish a single crate
publish_crate() {
    local crate=$1
    local crate_path="crates/${crate}"
    
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}📦 Publishing: ${GREEN}${crate}${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    # Change to crate directory
    pushd "${crate_path}" > /dev/null
    
    # Build publish command
    if [ "$DRY_RUN" = true ]; then
        cmd="cargo publish --dry-run"
    else
        cmd="cargo publish"
    fi
    
    # Execute publish
    echo -e "${YELLOW}Running: ${cmd}${NC}"
    if $cmd; then
        echo -e "${GREEN}✅ Success: ${crate}${NC}"
        popd > /dev/null
        return 0
    else
        echo -e "${RED}❌ Failed: ${crate}${NC}"
        popd > /dev/null
        return 1
    fi
}

# Function to wait between publishes
wait_for_crates_io() {
    if [ "$DRY_RUN" = false ]; then
        echo ""
        echo -e "${YELLOW}⏳ Waiting ${WAIT_TIME}s for crates.io to process...${NC}"
        for ((i=WAIT_TIME; i>0; i--)); do
            printf "\r${YELLOW}   %3d seconds remaining...${NC}" "$i"
            sleep 1
        done
        printf "\r${GREEN}   Ready to continue!        ${NC}\n"
    fi
}

# Pre-flight checks
echo -e "${BLUE}🔍 Pre-flight checks...${NC}"

# Check if logged in to crates.io
if [ "$DRY_RUN" = false ]; then
    if ! cargo whoami > /dev/null 2>&1; then
        echo -e "${RED}❌ Not logged in to crates.io${NC}"
        echo "Run: cargo login <YOUR_API_TOKEN>"
        exit 1
    fi
    echo -e "${GREEN}✅ Logged in as: $(cargo whoami)${NC}"
fi

# Verify all crates exist
for crate in "${CRATES[@]}"; do
    if [ ! -d "crates/${crate}" ]; then
        echo -e "${RED}❌ Crate directory not found: crates/${crate}${NC}"
        exit 1
    fi
done
echo -e "${GREEN}✅ All crate directories found${NC}"

# Run tests
echo ""
echo -e "${BLUE}🧪 Running tests...${NC}"
if cargo test --workspace --lib; then
    echo -e "${GREEN}✅ All tests passed${NC}"
else
    echo -e "${RED}❌ Tests failed${NC}"
    exit 1
fi

# Check formatting
echo ""
echo -e "${BLUE}🎨 Checking formatting...${NC}"
if cargo fmt --all --check 2>&1 | grep -v "Warning:" > /dev/null; then
    echo -e "${YELLOW}⚠️  Code formatting issues detected${NC}"
    echo "Run: cargo fmt --all"
    read -p "Continue anyway? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        exit 1
    fi
else
    echo -e "${GREEN}✅ Formatting looks good${NC}"
fi

# Start publishing
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🚀 Starting publish process...${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

# Publish each crate
FAILED_CRATES=()
for crate in "${CRATES[@]}"; do
    if ! publish_crate "$crate"; then
        FAILED_CRATES+=("$crate")
    fi
    
    # Wait between publishes (except after last one)
    if [ "$crate" != "${CRATES[-1]}" ]; then
        wait_for_crates_io
    fi
done

# Summary
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📊 Publishing Summary${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

if [ ${#FAILED_CRATES[@]} -eq 0 ]; then
    echo -e "${GREEN}✅ All crates published successfully!${NC}"
    echo ""
    if [ "$DRY_RUN" = false ]; then
        echo -e "${YELLOW}📝 Next steps:${NC}"
        echo "1. Verify on crates.io: https://crates.io/crates/tuxtui"
        echo "2. Check docs.rs: https://docs.rs/tuxtui (wait 10-15 min)"
        echo "3. Tag the release: git tag -a v0.1.0 -m 'Release v0.1.0'"
        echo "4. Push the tag: git push origin v0.1.0"
        echo "5. Create GitHub release"
    else
        echo -e "${YELLOW}Ready to publish for real!${NC}"
        echo "Run: ./scripts/publish.sh --execute"
    fi
    exit 0
else
    echo -e "${RED}❌ Failed crates:${NC}"
    for crate in "${FAILED_CRATES[@]}"; do
        echo -e "${RED}  - ${crate}${NC}"
    done
    exit 1
fi

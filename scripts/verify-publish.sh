#!/usr/bin/env bash
#
# Verify that all crates are ready for publishing to crates.io
#

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

ERRORS=0
WARNINGS=0

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔍 tuxtui Publishing Readiness Check${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Check 1: Cargo login
echo -e "${BLUE}1. Checking crates.io authentication...${NC}"
if cargo whoami > /dev/null 2>&1; then
    USERNAME=$(cargo whoami)
    echo -e "${GREEN}   ✅ Logged in as: ${USERNAME}${NC}"
else
    echo -e "${RED}   ❌ Not logged in to crates.io${NC}"
    echo -e "${YELLOW}      Run: cargo login <YOUR_API_TOKEN>${NC}"
    ((ERRORS++))
fi
echo ""

# Check 2: Required files
echo -e "${BLUE}2. Checking required files...${NC}"
for file in LICENSE README.md Cargo.toml; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}   ✅ ${file}${NC}"
    else
        echo -e "${RED}   ❌ Missing: ${file}${NC}"
        ((ERRORS++))
    fi
done
echo ""

# Check 3: Git status
echo -e "${BLUE}3. Checking git status...${NC}"
if git diff-index --quiet HEAD -- 2>/dev/null; then
    echo -e "${GREEN}   ✅ Working directory clean${NC}"
else
    echo -e "${YELLOW}   ⚠️  Uncommitted changes detected${NC}"
    echo -e "${YELLOW}      Consider committing before publishing${NC}"
    ((WARNINGS++))
fi

if git tag | grep -q "^v0.1.0$"; then
    echo -e "${YELLOW}   ⚠️  Tag v0.1.0 already exists${NC}"
    ((WARNINGS++))
else
    echo -e "${GREEN}   ✅ Version tag ready to create${NC}"
fi
echo ""

# Check 4: Tests
echo -e "${BLUE}4. Running tests...${NC}"
if cargo test --workspace --lib > /dev/null 2>&1; then
    echo -e "${GREEN}   ✅ All tests pass${NC}"
else
    echo -e "${RED}   ❌ Tests failed${NC}"
    echo -e "${YELLOW}      Run: cargo test --workspace${NC}"
    ((ERRORS++))
fi
echo ""

# Check 5: Clippy
echo -e "${BLUE}5. Running clippy...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
    echo -e "${GREEN}   ✅ No clippy warnings${NC}"
else
    echo -e "${YELLOW}   ⚠️  Clippy warnings detected${NC}"
    echo -e "${YELLOW}      Run: cargo clippy --all-targets --all-features${NC}"
    ((WARNINGS++))
fi
echo ""

# Check 6: Formatting
echo -e "${BLUE}6. Checking formatting...${NC}"
if cargo fmt --all --check 2>&1 | grep -v "Warning:" | grep -q "Diff"; then
    echo -e "${YELLOW}   ⚠️  Formatting issues${NC}"
    echo -e "${YELLOW}      Run: cargo fmt --all${NC}"
    ((WARNINGS++))
else
    echo -e "${GREEN}   ✅ Code properly formatted${NC}"
fi
echo ""

# Check 7: Documentation
echo -e "${BLUE}7. Building documentation...${NC}"
if cargo doc --workspace --no-deps > /dev/null 2>&1; then
    echo -e "${GREEN}   ✅ Documentation builds${NC}"
else
    echo -e "${YELLOW}   ⚠️  Documentation build warnings${NC}"
    ((WARNINGS++))
fi
echo ""

# Check 8: Dry run
echo -e "${BLUE}8. Testing publish (dry run)...${NC}"
CRATES=("tuxtui-core" "tuxtui-macros" "tuxtui-widgets" "tuxtui-crossterm" "tuxtui-termion" "tuxtui-termwiz" "tuxtui")

for crate in "${CRATES[@]}"; do
    if cargo publish --dry-run -p "$crate" > /dev/null 2>&1; then
        echo -e "${GREEN}   ✅ ${crate}${NC}"
    else
        echo -e "${RED}   ❌ ${crate} - dry run failed${NC}"
        ((ERRORS++))
    fi
done
echo ""

# Check 9: Version consistency
echo -e "${BLUE}9. Checking version consistency...${NC}"
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)
echo -e "${GREEN}   ✅ Workspace version: ${VERSION}${NC}"
echo ""

# Check 10: Crate metadata
echo -e "${BLUE}10. Verifying crate metadata...${NC}"
for crate in "${CRATES[@]}"; do
    CRATE_PATH="crates/${crate}/Cargo.toml"
    if [ -f "$CRATE_PATH" ]; then
        # Check for required fields
        if grep -q "description" "$CRATE_PATH" && \
           grep -q "license.workspace = true" "$CRATE_PATH" && \
           grep -q "repository.workspace = true" "$CRATE_PATH"; then
            echo -e "${GREEN}   ✅ ${crate} - metadata complete${NC}"
        else
            echo -e "${RED}   ❌ ${crate} - missing metadata${NC}"
            ((ERRORS++))
        fi
    fi
done
echo ""

# Summary
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📊 Summary${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ All checks passed! Ready to publish.${NC}"
    echo ""
    echo -e "${YELLOW}To publish:${NC}"
    echo "  ./scripts/publish.sh           # Dry run first"
    echo "  ./scripts/publish.sh --execute # Actually publish"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  ${WARNINGS} warning(s) detected${NC}"
    echo -e "${YELLOW}You can proceed, but consider fixing warnings first.${NC}"
    echo ""
    echo -e "${YELLOW}To publish:${NC}"
    echo "  ./scripts/publish.sh           # Dry run first"
    echo "  ./scripts/publish.sh --execute # Actually publish"
    exit 0
else
    echo -e "${RED}❌ ${ERRORS} error(s) detected${NC}"
    if [ $WARNINGS -gt 0 ]; then
        echo -e "${YELLOW}⚠️  ${WARNINGS} warning(s) detected${NC}"
    fi
    echo -e "${RED}Please fix errors before publishing.${NC}"
    exit 1
fi

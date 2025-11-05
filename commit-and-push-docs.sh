#!/bin/bash
# Wadah Engine - Commit and Push Docs Script
# Run this AFTER wadah-ui script

set -e

echo "🚀 Committing and pushing Wadah Engine documentation updates..."
echo ""

cd ~/Projects/wadah-engine

echo "📝 Staging documentation changes..."
git add docs/CROSS-REPO.md docs/UI-SPEC.md README.md

echo "💾 Creating commit..."
git commit -m "docs: Add Electron Desktop documentation

Complete cross-repo integration documentation

**Added:**
- docs/CROSS-REPO.md - Comprehensive integration guide
  * Repository structure for both projects
  * Development workflow
  * Integration patterns
  * Debugging guides
  * Deployment strategies

**Updated:**
- README.md - Added Wadah Desktop references
  * Link to wadah-ui repository
  * Desktop app quick start
  * Documentation updates

- docs/UI-SPEC.md - Complete frontend specification
  * Electron architecture
  * Component library
  * Design system
  * Deployment guide

**Wadah Desktop (wadah-ui):**
- Repository: https://github.com/devwadahai/wadah-ui
- Status: Electron conversion complete
- Version: 0.2.0
- Model: Following Docker Desktop

Ready for testing! 🎨"

echo "☁️  Pushing to GitHub..."
git push origin dev/initial-implementation

echo ""
echo "✅ SUCCESS! Documentation pushed to GitHub!"
echo ""
echo "🔗 https://github.com/devwadahai/wadah-engine"
echo ""
echo "📚 Both repositories updated!"
echo ""


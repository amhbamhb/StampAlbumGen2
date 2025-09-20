#!/bin/bash

echo "Testing GitLab CI/CD pipeline:"
echo ""

if [ -f ".gitlab-ci.yml" ]; then
    echo "✅ GitLab CI/CD pipeline file exists"
else
    echo "❌ GitLab CI/CD pipeline file missing"
fi

echo ""
echo "🎉 Test complete!"

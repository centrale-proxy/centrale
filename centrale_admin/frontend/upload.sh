#!/bin/bash

# Push current changes to the master branch
git push

# Navigate to the dist directory and clean it
cd dist
find . -mindepth 1 -maxdepth 1 ! -name '.git' -exec rm -rf {} +
cd ..

# Build the project with Parcel
parcel build index.html

# Commit and push the build to the main branch
git add .
git commit -a -m 'build'
git push
cd ..

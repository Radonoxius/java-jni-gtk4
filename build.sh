#!/bin/bash

echo "----------------"
echo Generating native libs...
echo "----------------"
cargo b --release

echo "----------------"
echo Generating Java archive...
echo "----------------"
javac -d build ix/radon/guiframe/*/*.java
jar --create --file test.jar --main-class ix.radon.guiframe.test.runner -C build .
rm -rf build
java -jar test.jar

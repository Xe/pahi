#!/bin/sh

(cd errors && dhall text < renderErrorMD.dhall > ../errors.md)

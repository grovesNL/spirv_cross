#!/bin/bash
cd spirv_cross/src/vendor/SPIRV-Cross \
    && rm -rf \
        cmake \
        gn \
        samples \
        shaders* \
        reference \
        tests* \
        *.sh \
        *.py

#!/bin/bash

set -eu

if [[ ${MYNEWT_VAL_ARCH_NAME} == '"cortex_m0"' ]]; then
  TARGET="thumbv6m-none-eabi"
elif [[ ${MYNEWT_VAL_ARCH_NAME} == '"cortex_m3"' ]]; then
  TARGET="thumbv7m-none-eabi"
elif [[ ${MYNEWT_VAL_ARCH_NAME} == '"cortex_m4"' || ${MYNEWT_VAL_ARCH_NAME} == '"cortex_m7"' ]]; then
  if [[ $MYNEWT_VAL_HARDFLOAT -eq 1 ]]; then
    TARGET="thumbv7em-none-eabihf"
  else
    TARGET="thumbv7em-none-eabi"
  fi
else
  echo "The ARCH_NAME ${MYNEWT_VAL_ARCH_NAME} is not supported"
  exit 1
fi

cargo build --target="${TARGET}" --target-dir="${MYNEWT_PKG_BIN_DIR}"
cp "${MYNEWT_PKG_BIN_DIR}"/${TARGET}/debug/*.a "${MYNEWT_PKG_BIN_ARCHIVE}"

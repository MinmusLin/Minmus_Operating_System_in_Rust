#!/usr/bin/bash

BINUTIL_VERSION=2.37
BINUTIL_URL="https://mirrors.aliyun.com/gnu/binutils/binutils-2.37.tar.xz?" \
            "spm=a2c6h.25603864.0.0.5f4539e11JPeX4"

GCC_VERSION=11.2.0
GCC_URL="https://mirrors.aliyun.com/gnu/gcc/gcc-11.2.0/gcc-11.2.0.tar.xz?" \
        "spm=a2c6h.25603864.0.0.6c5d9698I99N4Y"

GCC_SRC="gcc-${GCC_VERSION}"
BINUTIL_SRC="binutils-${BINUTIL_VERSION}"

export PREFIX="$HOME/cross-compiler"
export TARGET=i686-elf
export PATH="$PREFIX/bin:$PATH"

mkdir -p "${PREFIX}"
mkdir -p "${HOME}/toolchain/binutils-build"
mkdir -p "${HOME}/toolchain/gcc-build"

cd "${HOME}/toolchain"

if [ ! -d "${HOME}/toolchain/${GCC_SRC}" ]
then
    (wget -O "${GCC_SRC}.tar" ${GCC_URL} \
        && tar -xf "${GCC_SRC}.tar") || exit
    rm -f "${GCC_SRC}.tar"
else
    echo "Skip downloading gcc"
fi

if [ ! -d "${HOME}/toolchain/${BINUTIL_SRC}" ]
then
    (wget -O "${BINUTIL_SRC}.tar" ${BINUTIL_URL} \
        && tar -xf "${BINUTIL_SRC}.tar") || exit
    rm -f "${BINUTIL_SRC}.tar"
else
    echo "Skip downloading binutils"
fi

echo "Building Binutils 2.37 ..."

cd "${HOME}/toolchain/binutils-build"

("${HOME}/toolchain/${BINUTIL_SRC}/configure" \
    --target=$TARGET \
    --prefix="$PREFIX" \
    --with-sysroot \
    --disable-nls \
    --disable-werror) || exit

(make && make install) || exit

echo "Building GCC 11.2.0 ..."

cd "${HOME}/toolchain/gcc-build"

which -- "$TARGET-as" || echo "$TARGET-as is not in the PATH"

("${HOME}/toolchain/${GCC_SRC}/configure" \
    --target=$TARGET \
    --prefix="$PREFIX" \
    --disable-nls \
    --enable-languages=c,c++ \
    --without-headers) || exit

(make all-gcc && \
 make all-target-libgcc && \
 make install-gcc && \
 make install-target-libgcc) || exit

echo "Done"
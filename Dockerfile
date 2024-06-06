FROM ubuntu:latest AS base

RUN apt update && apt install -y build-essential git wget unzip

# Install raylib as systems library
RUN apt install -y libasound2-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libglu1-mesa-dev libxcursor-dev libxinerama-dev libwayland-dev libxkbcommon-dev
RUN git clone https://github.com/raysan5/raylib /tmp/raylibbuild/raylib
WORKDIR "/tmp/raylibbuild/raylib/src"
RUN make PLATFORM=PLATFORM_DESKTOP RAYLIB_LIBTYPE=SHARED # Make dynamic raylib
RUN make install RAYLIB_LIBTYPE=SHARED                   # Install as a dynamic shared library into /usr/local/lib/


# Install zig
RUN wget https://ziglang.org/download/0.12.0/zig-linux-x86_64-0.12.0.tar.xz -O /tmp/zig.tar.xz
RUN mkdir -p /usr/share/zig-linux
RUN tar -xvf /tmp/zig.tar.xz --strip-components=1 -C /usr/share/zig-linux
RUN ln -s /usr/share/zig-linux/zig /usr/bin/zig



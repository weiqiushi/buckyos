
import os
import sys
import tempfile
import shutil

# git clone https://github.com/openssl/openssl.git
# cd openssl
# ./Configure linux-aarch64 no-shared --prefix=/usr/local/arm-openssl
# make CC=aarch64-linux-gnu-gcc
# make install
# export OPENSSL_DIR=/usr/local/arm-openssl
# export OPENSSL_LIB_DIR=$OPENSSL_DIR/lib
# export OPENSSL_INCLUDE_DIR=$OPENSSL_DIR/include

# in ~/.cargo/config.toml
# [target.aarch64-unknown-linux-gnu]
# linker = "aarch64-linux-gnu-gcc"

build_dir = os.path.dirname(os.path.abspath(__file__))


temp_dir = tempfile.gettempdir()
project_name = "buckyos"
target_dir = os.path.join(temp_dir, "rust_build", project_name)
os.makedirs(target_dir, exist_ok=True)

args = sys.argv[1:]
if len(args) > 0:
    if args[0] == "clean":
        cargo_command = f'cargo clean --target-dir "{target_dir}"'
        os.system(cargo_command)


os.environ["RUSTFLAGS"] = "-C target-feature=+crt-static"
cargo_command = f'cargo build --target aarch64-unknown-linux-gnu --release --target-dir "{target_dir}"'
build_result = os.system(cargo_command)
if build_result != 0:
    print(f'build failed: {build_result}')
    exit(1)

target_dir = os.path.join(temp_dir, "rust_build", project_name,"aarch64-unknown-linux-gnu")
print(f'build success at: {target_dir}')

vite_build_dir = os.path.join(build_dir, "kernel/node_active")
vite_build_cmd = f'cd {vite_build_dir} && npm run build'
os.system(vite_build_cmd)

print(f'npm build success at: {vite_build_dir}')

print('copying files to rootfs')
destination_dir = os.path.join(build_dir, "rootfs/bin")
shutil.copy(os.path.join(target_dir, "release", "node_daemon"), destination_dir)
strip_cmd = f'aarch64-linux-gnu-strip {os.path.join(target_dir, "release", "node_daemon")}'
os.system(strip_cmd)

destination_dir = os.path.join(build_dir, "rootfs/bin/system_config")
shutil.copy(os.path.join(target_dir, "release", "system_config"), destination_dir)
strip_cmd = f'aarch64-linux-gnu-strip {os.path.join(target_dir, "release", "system_config")}'
os.system(strip_cmd)

destination_dir = os.path.join(build_dir, "rootfs/bin/verify_hub")
shutil.copy(os.path.join(target_dir, "release", "verify_hub"), destination_dir)
strip_cmd = f'aarch64-linux-gnu-strip {os.path.join(target_dir, "release", "verify_hub")}'
os.system(strip_cmd)

destination_dir = os.path.join(build_dir, "rootfs/bin/scheduler")
shutil.copy(os.path.join(target_dir, "release", "scheduler"), destination_dir)
strip_cmd = f'aarch64-linux-gnu-strip {os.path.join(target_dir, "release", "scheduler")}'
os.system(strip_cmd)

destination_dir = os.path.join(build_dir, "rootfs/bin/cyfs_gateway")
shutil.copy(os.path.join(target_dir, "release", "cyfs_gateway"), destination_dir)
strip_cmd = f'aarch64-linux-gnu-strip {os.path.join(target_dir, "release", "cyfs_gateway")}'
os.system(strip_cmd)

destination_dir = os.path.join(build_dir, "./web3_bridge/web3_gateway")
shutil.copy(os.path.join(target_dir, "release", "cyfs_gateway"), destination_dir)
strip_cmd = f'aarch64-linux-gnu-strip {os.path.join(target_dir, "release", "cyfs_gateway")}'
os.system(strip_cmd)

destination_dir = os.path.join(build_dir, "rootfs/bin")
shutil.copy(os.path.join(build_dir, "killall.py"), destination_dir)

src_dir = os.path.join(vite_build_dir, "dist")
destination_dir = os.path.join(build_dir, "rootfs/bin/active")
print(f'copying vite build {src_dir} to {destination_dir}')
shutil.rmtree(destination_dir)
shutil.copytree(src_dir, destination_dir)
print('copying files to rootfs & web3_bridge done')


# if /opt/buckyos not exist, copy rootfs to /opt/buckyos
if not os.path.exists("/opt/buckyos"):
    print('copying rootfs to /opt/buckyos')
    shutil.copytree(os.path.join(build_dir, "rootfs"), "/opt/buckyos")
else:
    print('updating files in /opt/buckyos/bin')
    shutil.rmtree("/opt/buckyos/bin")
    #just update bin
    shutil.copytree(os.path.join(build_dir, "rootfs/bin"), "/opt/buckyos/bin")





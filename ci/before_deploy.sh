# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    cross rustc --bin lovepack --target $TARGET --release -- -C lto

    # TODO Update this to package the right artifacts
    cp target/$TARGET/release/lovepack $stage/

    case $TARGET in
        x86_64-unknown-linux-gnu)
            TARGET_STRING=nix-x86_64
            ;;
        i686-unknown-linux-gnu)
            TARGET_STRING=nix-x32
            ;;
        x86_64-pc-windows-gnu)
            TARGET_STRING=nix-x86_64
            ;;
        i686-pc-windows-gnu)
            TARGET_STRING=nix-x32
            ;;
        x86_64-apple-darwin)
            TARGET_STRING=nix-x86_64
            ;;
        i686-apple-darwin)
            TARGET_STRING=nix-x32
            ;;
    esac

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TRAVIS_OS_NAME-$TARGET_STRING.tar.gz *
    cd $src

    rm -rf $stage
}

main

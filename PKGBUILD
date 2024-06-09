# Maintainer: Sushant Ray <contact@neon.is-a.dev>

pkgname='sprofile'
pkgver=0.0.1
pkgrel=0.0.1
epoch=
pkgdesc="A tool for accessing your spotify statistics without ever leaving the terminal"
arch=('x86_64')
url="https://github.com/goodboyneon/sprofile"
license=('MIT')
groups=()
depends=('nodejs>=18' 'rust' 'cargo')
makedepends=('git' 'cargo')
checkdepends=()
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("$pkgname-$pkgver.tar.gz::https://github.com/goodboyneon/$pkgname/archive/v$pkgver.tar.gz")
noextract=()
sha256sums=()
validpgpkeys=()

# prepare() {
# 	cd "$pkgname-$pkgver"
# 	patch -p1 -i "$srcdir/$pkgname-$pkgver.patch"
# }

build() {
	cd "$srcdir/$pkgname-$pkgver"
  cargo build --release
}

# check() {
# 	cd "$pkgname-$pkgver"
# 	make -k check
# }

package() {
	cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}

# Maintainer: Sushant Ray <contact@neon.is-a.dev>

pkgname=sprofile
pkgver=0.0.1
pkgrel=1
epoch=
pkgdesc="A tool for accessing your spotify statistics without ever leaving the terminal"
arch=(x86_64)
url="https://github.com/goodboyneon/sprofile.git"
license=('MIT')
groups=()
depends=(nodejs)
makedepends=(git)
checkdepends=()
optdepends=()
provides=(sprofile)
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("git+$url")
noextract=()
sha256sums=()
validpgpkeys=()

prepare() {
	cd "$pkgname-$pkgver"
	patch -p1 -i "$srcdir/$pkgname-$pkgver.patch"
}

build() {
	cd "$pkgname-$pkgver"
	./configure --prefix=/usr
	make
}

check() {
	cd "$pkgname-$pkgver"
	make -k check
}

package() {
	cd "$pkgname-$pkgver"
	make DESTDIR="$pkgdir/" install
}

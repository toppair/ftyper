# Maintainer: toppair <jbartapp@gmail.com>
pkgname=ftyper
pkgver=0.0.1_alpha
pkgrel=1
arch=("x86_64")
pkgdesc="Terminal typing practice"
license=("MIT")
depends=(gcc-libs)
url="https://github.com/toppair/ftyper"
source=("$pkgname-$pkgver.tar.gz::https://github.com/toppair/ftyper/releases/download/v0.0.1-alpha/ftyper-0.0.1_alpha.tar.gz")
package() {
   cd "$pkgname-$pkgver"

   install -Dm755 "$pkgname" "$pkgdir/usr/bin/$pkgname"
   install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
md5sums=('2460549809982788b9f3edde6bc41bcc')

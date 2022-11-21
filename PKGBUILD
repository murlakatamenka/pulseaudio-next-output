# Maintainer: Sergey A. <murlakatamenka@disroot.org>

pkgname=pulseaudio-next-output-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="Simple CLI utility to cycle through PulseAudio outputs."
url="https://github.com/murlakatamenka/pulseaudio-next-output"
license=("MIT")
arch=("x86_64")
provides=("pulseaudio-next-output")
depends=("libpulse")
source=("https://github.com/murlakatamenka/pulseaudio-next-output/releases/download/0.1.0/pulseaudio-next-output-$pkgver-x86_64-unknown-linux-gnu.tar.gz")
sha256sums=("07f4108fe9c434872b682ce15843c9d12c24f4f9855b2a7c5e98e0682dfb0ed4")
options=(!strip)

package() {
    install -Dm755 pulseaudio-next-output -t "$pkgdir/usr/bin/"
}

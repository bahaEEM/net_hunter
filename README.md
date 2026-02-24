#  NET-HUNTER

**NET-HUNTER**, Rust programlama dili ile geliştirilmiş, çok iş parçacıklı (multi-threaded) mimariye sahip, hızlı ve basit bir port tarama aracıdır. TCP el sıkışması (handshake) simülasyonu yaparak hedef sistem üzerindeki açık servisleri tespit eder.

##  Özellikler
- Belirlenen IP adresindeki ilk 1024 portu tarar.
- Renkli ve kullanıcı dostu CLI arayüzü.
- TCP bağlantı zaman aşımı (timeout) kontrolü.

##  Kurulum ve Kullanım

```bash
# Projeyi klonlayın
git clone [https://github.com/bahaEEM/net-hunter.git](https://github.com/bahaEEM/net-hunter.git)
cd net-hunter

# Çalıştırın (Örnek: Google DNS)
cargo run 8.8.8.8

[TR]
Bu yazılım (NET-HUNTER) tamamen eğitim ve ağ güvenliği testi amacıyla geliştirilmiştir. Geliştirici, bu aracın kötü niyetli kullanımından, yasalara aykırı eylemlerde kullanılmasından veya hedef sistemlerde doğurabileceği herhangi bir zarardan sorumlu tutulamaz. Kullanıcı, bu aracı sadece kendisine ait ağlarda veya yazılı izni olduğu sistemlerde kullanmayı kabul eder.

[EN]
This tool (NET-HUNTER) is created for educational and network security testing purposes only. The developer is not responsible for any misuse, illegal activities, or damage caused by this tool. The user agrees to use this software only on networks they own or have explicit permission to scan.
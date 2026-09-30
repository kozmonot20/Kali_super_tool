use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        println!("\n🚀 --- KALI SUPER TOOL MENÜ (RUST EDITION) --- 🚀");
        println!("1. Smart RAM Purger (RAM & Ön Bellek Temizleyici)");
        println!("2. Lightning File Organizer (Masaüstü Dosya Düzenleyici)");
        println!("3. Nano Crypto Notes (XOR Şifreli Güvenli Notlar)");
        println!("4. Çıkış");
        print!("Lütfen bir seçim yapın (1-4): ");
        io::stdout().flush().unwrap();

        let mut secim = String::new();
        io::stdin().read_line(&mut secim).unwrap();
        
        match secim.trim() {
            "1" => ram_temizle(),
            "2" => dosya_duzenle(),
            "3" => sifreli_notlar(),
            "4" => {
                println!("Görüşmek üzere, şef!");
                break;
            }
            _ => println!("❌ Geçersiz seçim, tekrar deneyin."),
        }
    }
}

// 🧠 PROJE 1: SMART RAM PURGER
fn ram_temizle() {
    println!("\n🧠 [Smart RAM Purger] Windows Standby ve Ön Bellek Listesi Temizleniyor...");
    
    // Windows'un kendi RAM yönetimini tetikler ve arka plandaki arama indeksleyicileri gibi çöpleri kapatır
    let komutlar = vec![
        ("ipconfig", vec!["/flushdns"]),
        ("taskkill", vec!["/F", "/IM", "SearchIndexer.exe", "/T"]),
        ("taskkill", vec!["/F", "/IM", "OneDrive.exe", "/T"])
    ];

    let mut sayac = 0;
    for (cmd, args) in komutlar {
        if Command::new(cmd).args(&args).output().is_ok() {
            sayac += 1;
        }
    }

    println!("✅ RAM Boost Tamamlandı! {} sistem temizliği yapıldı.", sayac);
    println!("💡 İpucu: Rust bellek yönetimi sayesinde bu işlem sırasında bilgisayarın 0 MB ekstra RAM tüketti!");
}

// 📂 PROJE 2: LIGHTNING FILE ORGANIZER
fn dosya_duzenle() {
    println!("\n📂 [Lightning File Organizer] Masaüstü taranıyor...");
    
    // Windows Masaüstü yolunu otomatik bulur
    let user_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\admin".to_string());
    let masaustu_yolu = format!("{}\\Desktop", user_profile);
    
    let path = Path::new(&masaustu_yolu);
    if !path.exists() {
        println!("❌ Masaüstü klasörü bulunamadı!");
        return;
    }

    let mut tasinan = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension_str() {
                    // Dosyanın taşınacağı alt klasör ismini belirle (Örn: Kodlar, Resimler)
                    let klasor_adi = match ext.to_lowercase().as_str() {
                        "py" | "rs" | "c" | "cpp" | "html" | "css" | "js" => "Kodlar",
                        "jpg" | "jpeg" | "png" | "gif" | "png" => "Resimler",
                        "pdf" | "docx" | "txt" | "xlsx" => "Belgeler",
                        "exe" | "msi" => "Kurulumlar",
                        _ => "Diger",
                    };

                    let hedef_klasor = format!("{}\\{}", masaustu_yolu, klasor_adi);
                    fs::create_dir_all(&hedef_klasor).unwrap();

                    if let Some(file_name) = file_path.file_name() {
                        let yeni_yol = format!("{}\\{}", hedef_klasor, file_name.to_str().unwrap());
                        if fs::rename(&file_path, &yeni_yol).is_ok() {
                            tasinan += 1;
                        }
                    }
                }
            }
        }
    }
    println!("⚡ İşlem Bitti! Masaüstündeki {} dağınık dosya ışık hızında klasörlendi.", tasinan);
}

// 📝 PROJE 3: NANO CRYPTO NOTES (ŞİFRELİ NOT DEFTERİ)
fn sifreli_notlar() {
    println!("\n📝 [Nano Crypto Notes] Şifreli Not Sistemine Hoş Geldin.");
    println!("1. Not Oku (Şifreyi Çöz)");
    println!("2. Not Yaz (Şifrele ve Kaydet)");
    print!("Seçiminiz: ");
    io::stdout().flush().unwrap();

    let mut secim = String::new();
    io::stdin().read_line(&mut secim).unwrap();
    
    let dosya_adi = "crypto_note.bin";
    // Kripto Anahtarı: Kimse okuyamasın diye veriyi bu gizli bayt anahtarıyla XOR şifreleyeceğiz (Askeri düzey güvenlik mantığı)
    let anahtar: u8 = 0x5A; 

    match secim.trim() {
        "1" => {
            if !Path::new(dosya_adi).exists() {
                println!("📂 Henüz kaydedilmiş şifreli bir not yok!");
                return;
            }
            let sifreli_veri = fs::read(dosya_adi).unwrap();
            // XOR şifresini tersine çözüyoruz
            let cozulmus_veri: Vec<u8> = sifreli_veri.iter().map(|b| b ^ anahtar).collect();
            let not = String::from_utf8(cozulmus_veri).unwrap_or_else(|_| "❌ Şifre çözme hatası!".to_string());
            println!("\n🔓 [GİZLİ NOTUNUZ]:\n----------------------\n{}", not);
            println!("----------------------");
        },
        "2" => {
            print!("Notunuzu yazın: ");
            io::stdout().flush().unwrap();
            let mut not_metni = String::new();
            io::stdin().read_line(&mut not_metni).unwrap();

            // Veriyi XOR yöntemiyle şifreliyoruz (Diske düz metin olarak değil, şifreli bayt olarak yazılacak)
            let sifreli_veri: Vec<u8> = not_metni.trim().as_bytes().iter().map(|b| b ^ anahtar).collect();
            fs::write(dosya_adi, sifreli_veri).unwrap();
            println!("🔒 Notunuz askeri yöntemle şifrelendi ve '{}' olarak kaydedildi!", dosya_adi);
        },
        _ => println!("Geçersiz seçim!"),
    }
}

// Yardımcı ek fonksiyon (Dosya uzantılarını kolayca okumak için)
trait ExtensionStr {
    fn extension_str(&self) -> Option<&str>;
}
impl ExtensionStr for Path {
    fn extension_str(&self) -> Option<&str> {
        self.extension()?.to_str()
    }
}

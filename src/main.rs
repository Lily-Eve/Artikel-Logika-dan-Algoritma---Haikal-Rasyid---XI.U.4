use std::io; // Mengimpor library untuk input/output

fn main() {
    println!("--- Program Hitung Luas Persegi Panjang ---");

    // 1. Deklarasi variabel untuk menyimpan input
    let mut input_panjang = String::new();
    let mut input_lebar = String::new();

    // 2. Input Panjang
    println!("Masukkan panjang: ");
    io::stdin()
        .read_line(&mut input_panjang)
        .expect("Gagal membaca input");

    // 3. Input Lebar
    println!("Masukkan lebar: ");
    io::stdin()
        .read_line(&mut input_lebar)
        .expect("Gagal membaca input");

    // 4. Konversi String ke angka (float 64-bit) agar bisa dihitung
    let panjang: f64 = input_panjang.trim().parse().expect("Mohon masukkan angka!");
    let lebar: f64 = input_lebar.trim().parse().expect("Mohon masukkan angka!");

    // 5. Hitung Luas
    let luas = panjang * lebar;

    // 6. Tampilkan Hasil
    println!("Luas persegi panjang adalah: {}", luas);
}

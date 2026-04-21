print("--- Program Hitung Luas Persegi Panjang ---")

try:
    panjang = float(input("Masukkan panjang: "))
    lebar = float(input("Masukkan lebar: "))
    
    luas = panjang * lebar
    
    print(f"Luas persegi panjang adalah: {luas}")
except ValueError:
    print("Mohon masukkan angka yang valid!")

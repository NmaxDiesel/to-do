use std::io;

fn main() {
    let mut to_do_list: Vec<String> = Vec::new();

    loop {
        println!("====================================");
        println!("To-Do List");
        println!("====================================");
        println!("1. Tambah Tugas");
        println!("2. Hapus Tugas");
        println!("3. Lihat Daftar Tugas");
        println!("4. Keluar");
        println!("====================================");
        println!("Masukkan pilihan Anda:");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca baris");

        match pilihan.trim().parse() {
            Ok(1) => tambah_tugas(&mut to_do_list),
            Ok(2) => hapus_tugas(&mut to_do_list),
            Ok(3) => lihat_daftar_tugas(&to_do_list),
            Ok(4) => {
                println!("Terima kasih telah menggunakan To-Do List!");
                break;
            }
            _ => println!("Pilihan tidak valid, coba lagi."),
        }
    }
}

fn tambah_tugas(to_do_list: &mut Vec<String>) {
    println!("Masukkan nama tugas yang ingin ditambahkan:");
    let mut tugas_baru = String::new();
    io::stdin()
        .read_line(&mut tugas_baru)
        .expect("Gagal membaca baris");

    to_do_list.push(tugas_baru.trim().to_string());

    println!("Tugas berhasil ditambahkan ke daftar.");
}

fn hapus_tugas(to_do_list: &mut Vec<String>) {
    if to_do_list.is_empty() {
        println!("Daftar tugas kosong.");
    } else {
        lihat_daftar_tugas(&to_do_list);
        println!("Masukkan nomor tugas yang ingin dihapus:");

        let mut nomor_tugas = String::new();
        io::stdin()
            .read_line(&mut nomor_tugas)
            .expect("Gagal membaca baris");

        match nomor_tugas.trim().parse::<usize>() {
            Ok(nomor) if nomor > 0 && nomor <= to_do_list.len() => {
                to_do_list.remove(nomor - 1);
                println!("Tugas berhasil dihapus dari daftar.");
            }
            _ => println!("Nomor tugas tidak valid."),
        }
    }
}

fn lihat_daftar_tugas(to_do_list: &Vec<String>) {
    if to_do_list.is_empty() {
        println!("Daftar tugas kosong.");
    } else {
        println!("Daftar Tugas:");
        for (index, tugas) in to_do_list.iter().enumerate() {
            println!("{}. {}", index + 1, tugas);
        }
    }
}
